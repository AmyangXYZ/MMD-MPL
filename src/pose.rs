use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    bone::MPLBoneState,
    utils::{Quaternion, Vector3},
    with_bone_db,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MPLPoseStatement {
    pub bone: String,
    pub action: String,
    pub direction: String,
    pub degrees: f32,
}

impl MPLPoseStatement {
    pub fn from_str(text: &str) -> Result<Self, String> {
        if text.is_empty() {
            return Err("Empty statement".to_string());
        }
        let parts = text.split_whitespace().collect::<Vec<&str>>();
        if parts.len() != 4 {
            return Err("Invalid statement".to_string());
        }

        let bone = parts[0].to_string();
        let action = parts[1].to_string();
        let direction = parts[2].to_string();
        let degrees: f32 = parts[3]
            .trim()
            .parse()
            .map_err(|_| "Invalid degrees number".to_string())?;

        // Validate using bone database
        with_bone_db(|db| db.validate(&bone, &action, &direction, degrees))?;

        Ok(Self {
            bone,
            action,
            direction,
            degrees,
        })
    }

    pub fn to_quaternion(&self) -> Quaternion {
        // Get the rule from bone database
        let rule = with_bone_db(|db| {
            db.get_rule(&self.bone, &self.action, &self.direction)
                .cloned()
        });

        let rule = match rule {
            Some(r) => r,
            None => return Quaternion::identity(), // Default if no rule found
        };

        // Get normalized rotation axis
        let normalized_axis = rule.axis.normalize();

        // Create quaternion from axis-angle
        let radians = self.degrees * (std::f32::consts::PI / 180.0);
        let half_angle = radians / 2.0;
        let sin = half_angle.sin();
        let cos = half_angle.cos();

        Quaternion::new(
            normalized_axis.x * sin,
            normalized_axis.y * sin,
            normalized_axis.z * sin,
            cos,
        )
    }

    /// Convert a target quaternion for a given bone back into one or more MPL
    /// statements by optimising the per-axis rotation degrees so their composed
    /// quaternion matches the target. Returns an empty vector when no rule is found.
    pub fn from_quaternion(bone: &str, target_quat: Quaternion) -> Vec<Self> {
        use crate::bone::ActionRule;

        let bone = bone.to_string();

        // Gather all possible (action, direction) rules for this bone
        let possible_actions: Vec<(String, String, ActionRule)> = crate::with_bone_db(|db| {
            let mut vec = Vec::new();
            if let Some(actions) = db.actions(&bone) {
                for action in actions {
                    if let Some(directions) = db.directions(&bone, action) {
                        for direction in directions {
                            if let Some(rule) = db.get_rule(&bone, action, direction) {
                                vec.push((action.to_string(), direction.to_string(), rule.clone()));
                            }
                        }
                    }
                }
            }
            vec
        });

        if possible_actions.is_empty() {
            return vec![];
        }

        // Ensure deterministic order independent of HashMap iteration
        let mut possible_actions = possible_actions;
        possible_actions.sort_by(|a, b| {
            let key_a = format!("{}-{}", a.0, a.1);
            let key_b = format!("{}-{}", b.0, b.1);
            key_a.cmp(&key_b)
        });

        // 1.  Fast exact-axis check: if the target rotation is (almost) a pure
        //     rotation around one of the rule axes, return that single statement.
        if target_quat.w.abs() < 1.0 {
            let angle_rad = 2.0 * target_quat.w.acos();
            let sin_half = (angle_rad / 2.0).sin();
            if sin_half.abs() > 1e-6 {
                let axis_norm = Vector3::new(
                    target_quat.x / sin_half,
                    target_quat.y / sin_half,
                    target_quat.z / sin_half,
                )
                .normalize();

                // Try to find a rule axis that aligns (within ±0.001)
                for (action, direction, rule) in &possible_actions {
                    let rule_norm = rule.axis.normalize();
                    let dot = axis_norm.x * rule_norm.x
                        + axis_norm.y * rule_norm.y
                        + axis_norm.z * rule_norm.z;
                    if dot.abs() > 0.999 {
                        // Matching axis (sign may differ)
                        let chosen_action = action.clone();
                        let mut chosen_direction = direction.clone();
                        let mut degrees = angle_rad.to_degrees();

                        if dot < 0.0 {
                            // Opposite sign: try to find direction with opposite axis
                            if let Some(opposite_rule) =
                                possible_actions.iter().find(|(_, d, r)| {
                                    d != direction && (r.axis.normalize().dot(&rule_norm) < -0.999)
                                })
                            {
                                chosen_direction = opposite_rule.1.clone();
                            }
                            degrees = degrees.abs();
                        }

                        degrees = ((degrees.clamp(0.0, rule.limit) * 1000.0).round()) / 1000.0;
                        return vec![Self {
                            bone: bone.clone(),
                            action: chosen_action,
                            direction: chosen_direction,
                            degrees,
                        }];
                    }
                }
            }
        }

        // Evaluate a degree vector -> distance to target quaternion
        let evaluate = |degrees: &[f32]| -> f32 {
            let mut combined = Quaternion::identity();
            for (i, deg) in degrees.iter().enumerate() {
                let rule = &possible_actions[i].2;
                let clamped = deg.clamp(0.0, rule.limit);
                if clamped > 0.01 {
                    let q = Quaternion::from_axis_angle(rule.axis, clamped);
                    combined = combined.multiply(&q);
                }
            }
            target_quat.angular_distance(&combined)
        };

        // Nelder-Mead parameters
        let n = possible_actions.len();
        let alpha = 1.0;
        let gamma = 2.0;
        let rho = 0.5;
        let sigma = 0.5;
        let tolerance = 1e-6;
        let max_iter = 800;

        // Run optimisation from a few deterministic starting points
        let mut best_degrees = Vec::new();
        let mut best_distance = f32::INFINITY;

        let start_points: Vec<Vec<f32>> = vec![
            vec![0.0; n],                                               // zero
            possible_actions.iter().map(|a| a.2.limit * 0.5).collect(), // mid-range
            possible_actions
                .iter()
                .enumerate()
                .map(|(i, a)| {
                    if i % 2 == 0 {
                        a.2.limit * 0.3
                    } else {
                        a.2.limit * 0.7
                    }
                })
                .collect(), // mixed
        ];

        for initial_guess in start_points {
            // Build initial simplex
            let mut simplex: Vec<(Vec<f32>, f32)> = Vec::with_capacity(n + 1);
            let initial_val = evaluate(&initial_guess);
            simplex.push((initial_guess.clone(), initial_val));
            for i in 0..n {
                let mut point = initial_guess.clone();
                let range = possible_actions[i].2.limit;
                point[i] += range * 0.1;
                let val = evaluate(&point);
                simplex.push((point, val));
            }

            // Main loop
            for _ in 0..max_iter {
                simplex.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                let best = simplex[0].1;
                let worst = simplex[n].1;
                if worst - best < tolerance {
                    break;
                }

                // Centroid of all but worst
                let mut centroid = vec![0.0f32; n];
                for i in 0..n {
                    for j in 0..n {
                        centroid[j] += simplex[i].0[j];
                    }
                }
                for c in &mut centroid {
                    *c /= n as f32;
                }

                // Reflection
                let reflected: Vec<f32> = centroid
                    .iter()
                    .zip(&simplex[n].0)
                    .map(|(c, w)| c + alpha * (c - w))
                    .collect();
                let reflected_val = evaluate(&reflected);

                if reflected_val >= simplex[0].1 && reflected_val < simplex[n - 1].1 {
                    simplex[n] = (reflected, reflected_val);
                    continue;
                }

                // Expansion
                if reflected_val < simplex[0].1 {
                    let expanded: Vec<f32> = centroid
                        .iter()
                        .zip(&reflected)
                        .map(|(c, r)| c + gamma * (r - c))
                        .collect();
                    let expanded_val = evaluate(&expanded);
                    if expanded_val < reflected_val {
                        simplex[n] = (expanded, expanded_val);
                    } else {
                        simplex[n] = (reflected, reflected_val);
                    }
                    continue;
                }

                // Contraction
                let contracted: Vec<f32> = centroid
                    .iter()
                    .zip(&simplex[n].0)
                    .map(|(c, w)| c + rho * (w - c))
                    .collect();
                let contracted_val = evaluate(&contracted);
                if contracted_val < simplex[n].1 {
                    simplex[n] = (contracted, contracted_val);
                    continue;
                }

                // Shrink
                let best_point = simplex[0].0.clone();
                for i in 1..=n {
                    let new_point: Vec<f32> = simplex[i]
                        .0
                        .iter()
                        .zip(&best_point)
                        .map(|(p, b)| b + sigma * (p - b))
                        .collect();
                    let val = evaluate(&new_point);
                    simplex[i] = (new_point, val);
                }
            }

            simplex.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            let (candidate_deg, candidate_dist) = simplex[0].clone();
            if candidate_dist < best_distance {
                best_distance = candidate_dist;
                best_degrees = candidate_deg;
            }
        }

        // After global optimisation, sparsify: greedily zero the smallest axes while
        // keeping the quaternion within a tight error bound.

        let epsilon = 1e-4; // angular distance threshold (~0.006°)
        let mut sparse_degrees = best_degrees.clone();

        // Build list of indices sorted by absolute degree ascending
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by(|&i, &j| {
            sparse_degrees[i]
                .abs()
                .partial_cmp(&sparse_degrees[j].abs())
                .unwrap()
        });

        for idx in order {
            let saved = sparse_degrees[idx];
            if saved.abs() < 1e-4 {
                // Already virtually zero
                sparse_degrees[idx] = 0.0;
                continue;
            }
            sparse_degrees[idx] = 0.0;
            let err = evaluate(&sparse_degrees);
            if err > epsilon {
                // Need this axis, restore
                sparse_degrees[idx] = saved;
            }
        }

        // ------------------- 2nd optimisation on active axes -------------------
        let active_indices: Vec<usize> = sparse_degrees
            .iter()
            .enumerate()
            .filter(|(_, &d)| d.abs() > 0.0)
            .map(|(i, _)| i)
            .collect();

        if !active_indices.is_empty() {
            let m = active_indices.len();
            // Build initial guess vector for active axes
            let active_guess: Vec<f32> = active_indices
                .iter()
                .map(|&idx| sparse_degrees[idx])
                .collect();

            // Reuse Nelder-Mead on this reduced dimension
            let nm_evaluate = |active_vals: &[f32]| -> f32 {
                let mut full = sparse_degrees.clone();
                for (k, &idx) in active_indices.iter().enumerate() {
                    full[idx] = active_vals[k].clamp(0.0, possible_actions[idx].2.limit);
                }
                evaluate(&full)
            };

            // Build simplex for active space
            let mut simplex: Vec<(Vec<f32>, f32)> = Vec::with_capacity(m + 1);
            let initial_val = nm_evaluate(&active_guess);
            simplex.push((active_guess.clone(), initial_val));
            for i in 0..m {
                let mut point = active_guess.clone();
                let idx = active_indices[i];
                let range = possible_actions[idx].2.limit;
                point[i] += range * 0.05;
                let val = nm_evaluate(&point);
                simplex.push((point, val));
            }

            let tol_local = 1e-8;
            for _ in 0..600 {
                simplex.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                let best = simplex[0].1;
                let worst = simplex[m].1;
                if worst - best < tol_local {
                    break;
                }

                // Centroid excluding worst
                let mut centroid = vec![0.0f32; m];
                for i in 0..m {
                    for j in 0..m {
                        centroid[j] += simplex[i].0[j];
                    }
                }
                for c in &mut centroid {
                    *c /= m as f32;
                }

                // reflection
                let alpha = 1.0;
                let gamma = 2.0;
                let rho = 0.5;
                let sigma = 0.5;

                let reflected: Vec<f32> = centroid
                    .iter()
                    .zip(&simplex[m].0)
                    .map(|(c, w)| c + alpha * (c - w))
                    .collect();
                let reflected_val = nm_evaluate(&reflected);

                if reflected_val >= simplex[0].1 && reflected_val < simplex[m - 1].1 {
                    simplex[m] = (reflected, reflected_val);
                    continue;
                }

                if reflected_val < simplex[0].1 {
                    let expanded: Vec<f32> = centroid
                        .iter()
                        .zip(&reflected)
                        .map(|(c, r)| c + gamma * (r - c))
                        .collect();
                    let expanded_val = nm_evaluate(&expanded);
                    if expanded_val < reflected_val {
                        simplex[m] = (expanded, expanded_val);
                    } else {
                        simplex[m] = (reflected, reflected_val);
                    }
                    continue;
                }

                let contracted: Vec<f32> = centroid
                    .iter()
                    .zip(&simplex[m].0)
                    .map(|(c, w)| c + rho * (w - c))
                    .collect();
                let contracted_val = nm_evaluate(&contracted);
                if contracted_val < simplex[m].1 {
                    simplex[m] = (contracted, contracted_val);
                    continue;
                }

                // shrink
                let best_point = simplex[0].0.clone();
                for i in 1..=m {
                    let new_point: Vec<f32> = simplex[i]
                        .0
                        .iter()
                        .zip(&best_point)
                        .map(|(p, b)| b + sigma * (p - b))
                        .collect();
                    let val = nm_evaluate(&new_point);
                    simplex[i] = (new_point, val);
                }
            }

            simplex.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            let best_active = &simplex[0].0;
            // Update sparse_degrees
            for (k, &idx) in active_indices.iter().enumerate() {
                sparse_degrees[idx] = best_active[k];
            }
        }

        // Map optimal (sparsified) degrees back to MPL statements and simplify opposing directions
        let mut action_map: HashMap<String, HashMap<String, f32>> = HashMap::new();
        for (i, deg) in sparse_degrees.iter().enumerate() {
            let clamped = deg.clamp(0.0, possible_actions[i].2.limit);
            if clamped > 0.00001 {
                let (ref action, ref direction, _) = possible_actions[i];
                action_map
                    .entry(action.clone())
                    .or_default()
                    .insert(direction.clone(), clamped);
            }
        }

        let opposing_pairs = [("forward", "backward"), ("left", "right")];
        let mut results = Vec::new();
        for (action, directions) in action_map.into_iter() {
            let mut processed = std::collections::HashSet::new();
            for &(dir1, dir2) in &opposing_pairs {
                if processed.contains(dir1) || processed.contains(dir2) {
                    continue;
                }
                if let (Some(&deg1), Some(&deg2)) = (directions.get(dir1), directions.get(dir2)) {
                    let net = (deg1 - deg2).abs();
                    if net > 0.0001 {
                        let net_rounded = (net * 1000.0).round() / 1000.0;
                        let net_dir = if deg1 > deg2 { dir1 } else { dir2 };
                        results.push(Self {
                            bone: bone.clone(),
                            action: action.clone(),
                            direction: net_dir.to_string(),
                            degrees: net_rounded,
                        });
                    }
                    processed.insert(dir1);
                    processed.insert(dir2);
                }
            }
            for (dir, &deg) in directions.iter() {
                if processed.contains(dir.as_str()) {
                    continue;
                }
                if deg > 0.0001 {
                    let rounded = (deg * 1000.0).round() / 1000.0;
                    results.push(Self {
                        bone: bone.clone(),
                        action: action.clone(),
                        direction: dir.clone(),
                        degrees: rounded,
                    });
                }
            }
        }

        // Final sanity: sort by action name for stable output
        results.sort_by(|a, b| {
            let key_a = format!("{}-{}", a.action, a.direction);
            let key_b = format!("{}-{}", b.action, b.direction);
            key_a.cmp(&key_b)
        });

        // ------------------- Final greedy pruning of tiny statements -------------------
        let epsilon_final = 1e-4_f32; // same as sparsify accuracy goal
        let mut pruned = results.clone();
        pruned.sort_by(|a, b| a.degrees.partial_cmp(&b.degrees).unwrap());

        let mut idx = 0usize;
        while idx < pruned.len() {
            // attempt to remove pruned[idx]
            let trial: Vec<_> = pruned
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != idx)
                .map(|(_, s)| s.clone())
                .collect();

            // compose quaternion from trial statements
            let mut q_trial = Quaternion::identity();
            for stmt in &trial {
                if let Some(rule) = with_bone_db(|db| {
                    db.get_rule(&stmt.bone, &stmt.action, &stmt.direction)
                        .cloned()
                }) {
                    let q = Quaternion::from_axis_angle(rule.axis, stmt.degrees);
                    q_trial = q_trial.multiply(&q);
                }
            }

            if target_quat.angular_distance(&q_trial) <= epsilon_final {
                // removal acceptable, update pruned and restart
                pruned = trial;
                pruned.sort_by(|a, b| a.degrees.partial_cmp(&b.degrees).unwrap());
                idx = 0;
            } else {
                idx += 1;
            }
        }

        pruned.retain(|s| s.degrees >= 1.0);

        // ------------------- Final snap to neat 0.1° values -------------------
        let mut snapped = pruned.clone();
        for stmt in &mut snapped {
            stmt.degrees = (stmt.degrees * 10.0).round() / 10.0; // 0.1° granularity
        }

        let mut q_snap = Quaternion::identity();
        for stmt in &snapped {
            if let Some(rule) = with_bone_db(|db| {
                db.get_rule(&stmt.bone, &stmt.action, &stmt.direction)
                    .cloned()
            }) {
                let q = Quaternion::from_axis_angle(rule.axis, stmt.degrees);
                q_snap = q_snap.multiply(&q);
            }
        }

        if target_quat.angular_distance(&q_snap) <= epsilon_final {
            pruned = snapped;
        }

        pruned
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MPLPose {
    pub name: String,
    pub statements: Vec<MPLPoseStatement>,
}

impl MPLPose {
    pub fn new(name: String, statements: Vec<MPLPoseStatement>) -> Self {
        Self { name, statements }
    }

    pub fn to_bone_states(&self) -> Vec<MPLBoneState> {
        let mut states = vec![];

        // Group statements by bone
        let mut bone_groups: HashMap<String, Vec<&MPLPoseStatement>> = HashMap::new();
        for statement in &self.statements {
            bone_groups
                .entry(statement.bone.clone())
                .or_insert_with(Vec::new)
                .push(statement);
        }

        // Process each bone group
        for (bone, bone_statements) in bone_groups {
            let mut combined_quaternion = Quaternion::identity();

            for statement in bone_statements {
                let quaternion = statement.to_quaternion();
                combined_quaternion = combined_quaternion.multiply(&quaternion);
            }

            let position = Vector3::new(0.0, 0.0, 0.0);
            let bone_name_jp =
                with_bone_db(|db| db.japanese_name(&bone).unwrap_or(&bone).to_string());

            states.push(MPLBoneState::new(
                bone,
                bone_name_jp,
                position,
                combined_quaternion,
            ));
        }

        states
    }
}
