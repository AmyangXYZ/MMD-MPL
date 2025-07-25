use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    bone::MPLBoneState,
    utils::{Quaternion, Vector3},
    with_bone_db, ActionRule,
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

    pub fn to_string(&self) -> String {
        format!(
            "{} {} {} {:.0};",
            self.bone, self.action, self.direction, self.degrees
        )
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
    ///
    /// # Arguments
    /// * `bone` - The bone name to generate statements for
    /// * `target_quat` - The target quaternion to approximate
    /// * `precision` - The precision for degree values (e.g., 1.0, 0.1, 0.001)
    pub fn from_quaternion(bone: &str, target_quat: Quaternion) -> Vec<Self> {
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

        // Evaluate fitness of a degree combination
        let evaluate_combination = |degrees: &[f32]| -> f32 {
            if degrees.len() != possible_actions.len() {
                return f32::INFINITY;
            }

            let mut combined_quaternion = Quaternion::identity();

            for (i, deg) in degrees.iter().enumerate() {
                let clamped_deg = deg.max(0.0).min(possible_actions[i].2.limit);
                if clamped_deg > 0.01 {
                    // Only apply significant rotations
                    let q = Quaternion::from_axis_angle(possible_actions[i].2.axis, clamped_deg);
                    combined_quaternion = combined_quaternion.multiply(&q);
                }
            }

            target_quat.angular_distance(&combined_quaternion)
        };

        // Nelder-Mead simplex optimization algorithm
        let nelder_mead = |initial_guess: &[f32], max_iterations: usize| -> (Vec<f32>, f32) {
            let n = initial_guess.len();
            let alpha = 1.0; // reflection coefficient
            let gamma = 2.0; // expansion coefficient
            let rho = 0.5; // contraction coefficient
            let sigma = 0.5; // shrinkage coefficient

            // Initialize simplex with n+1 points
            let mut simplex: Vec<(Vec<f32>, f32)> = Vec::new();

            simplex.push((initial_guess.to_vec(), evaluate_combination(initial_guess)));

            // Create additional points by perturbing initial guess
            for i in 0..n {
                let mut point = initial_guess.to_vec();
                let range = possible_actions[i].2.limit;
                point[i] += range * 0.1;
                let value = evaluate_combination(&point);
                simplex.push((point, value));
            }

            // Main optimization loop
            for _ in 0..max_iterations {
                simplex.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

                let best_value = simplex[0].1;
                let worst_value = simplex[n].1;
                let second_worst_value = simplex[n - 1].1;

                // Check convergence
                if worst_value - best_value < 0.0001 {
                    break;
                }

                // Calculate centroid (excluding worst point)
                let mut centroid = vec![0.0f32; n];
                for i in 0..n {
                    for j in 0..n {
                        centroid[j] += simplex[i].0[j];
                    }
                }
                for j in 0..n {
                    centroid[j] /= n as f32;
                }

                // Reflection step
                let reflected: Vec<f32> = centroid
                    .iter()
                    .zip(&simplex[n].0)
                    .map(|(c, w)| c + alpha * (c - w))
                    .collect();
                let reflected_value = evaluate_combination(&reflected);

                if reflected_value >= best_value && reflected_value < second_worst_value {
                    simplex[n] = (reflected, reflected_value);
                    continue;
                }

                // Expansion step
                if reflected_value < best_value {
                    let expanded: Vec<f32> = centroid
                        .iter()
                        .zip(&reflected)
                        .map(|(c, r)| c + gamma * (r - c))
                        .collect();
                    let expanded_value = evaluate_combination(&expanded);

                    if expanded_value < reflected_value {
                        simplex[n] = (expanded, expanded_value);
                    } else {
                        simplex[n] = (reflected, reflected_value);
                    }
                    continue;
                }

                // Contraction step
                let contracted: Vec<f32> = centroid
                    .iter()
                    .zip(&simplex[n].0)
                    .map(|(c, w)| c + rho * (w - c))
                    .collect();
                let contracted_value = evaluate_combination(&contracted);

                if contracted_value < worst_value {
                    simplex[n] = (contracted, contracted_value);
                    continue;
                }

                // Shrinkage step
                let best_point = simplex[0].0.clone();
                for i in 1..=n {
                    for j in 0..n {
                        simplex[i].0[j] = best_point[j] + sigma * (simplex[i].0[j] - best_point[j]);
                    }
                    simplex[i].1 = evaluate_combination(&simplex[i].0);
                }
            }

            simplex.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            (simplex[0].0.clone(), simplex[0].1)
        };

        let mut best_result = (Vec::new(), f32::INFINITY);

        // Try optimization from multiple starting points for global search
        let starting_points = vec![
            vec![0.0; possible_actions.len()], // Zero start
            possible_actions
                .iter()
                .enumerate()
                .map(|(i, action)| {
                    let limit = action.2.limit.min(30.0);
                    // Use deterministic "random" values based on index
                    let pseudo_random = ((i * 12345) % 1000) as f32 / 1000.0;
                    (limit * pseudo_random).min(limit)
                })
                .collect(), // Pseudo-random start
            possible_actions
                .iter()
                .map(|action| action.2.limit * 0.5)
                .collect(), // Mid-range start
            possible_actions
                .iter()
                .enumerate()
                .map(|(i, action)| {
                    if i % 2 == 0 {
                        action.2.limit * 0.3
                    } else {
                        action.2.limit * 0.7
                    }
                })
                .collect(), // Mixed start
        ];

        for start in starting_points {
            let result = nelder_mead(&start, 1000);
            if result.1 < best_result.1 {
                best_result = result;
            }
        }

        // Convert optimal degrees to MPL statements and simplify opposing actions
        let mut action_map: HashMap<String, HashMap<String, f32>> = HashMap::new();

        // Group degrees by action and direction
        for (i, deg) in best_result.0.iter().enumerate() {
            if *deg > 0.01 {
                let action = &possible_actions[i];
                let clamped_deg = deg.max(0.0).min(action.2.limit);

                action_map
                    .entry(action.0.clone())
                    .or_default()
                    .insert(action.1.clone(), clamped_deg);
            }
        }

        // Simplify opposing directions within each action
        let mut statements = Vec::new();
        for (action, directions) in action_map.into_iter() {
            // Handle opposing pairs
            let opposing_pairs = [("forward", "backward"), ("left", "right")];
            let mut processed_directions = std::collections::HashSet::new();

            for (dir1, dir2) in opposing_pairs.iter() {
                if directions.contains_key(*dir1)
                    && directions.contains_key(*dir2)
                    && !processed_directions.contains(*dir1)
                    && !processed_directions.contains(*dir2)
                {
                    let deg1 = directions.get(*dir1).unwrap();
                    let deg2 = directions.get(*dir2).unwrap();
                    let net_degrees = (deg1 - deg2).abs();

                    if net_degrees > 0.01 {
                        let net_direction = if deg1 > deg2 { dir1 } else { dir2 };
                        statements.push(Self {
                            bone: bone.clone(),
                            action: action.clone(),
                            direction: net_direction.to_string(),
                            degrees: net_degrees,
                        });
                    }

                    processed_directions.insert(*dir1);
                    processed_directions.insert(*dir2);
                }
            }

            // Handle remaining directions that don't have opposing pairs
            for (direction, degrees) in directions.iter() {
                if !processed_directions.contains(direction.as_str()) && *degrees > 0.01 {
                    statements.push(Self {
                        bone: bone.clone(),
                        action: action.clone(),
                        direction: direction.clone(),
                        degrees: *degrees,
                    });
                }
            }
        }

        // Format statements to match TypeScript output format
        statements
            .into_iter()
            .filter(|stmt| !format!("{:.0}", stmt.degrees).ends_with("0"))
            .collect()
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

    pub fn to_string(&self) -> String {
        format!(
            "@pose {} {{\n{}\n}}",
            self.name,
            self.statements
                .iter()
                .map(|s| format!("    {}", s.to_string()))
                .collect::<Vec<String>>()
                .join("\n")
        )
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

    pub fn from_bone_states(name: &str, states: Vec<MPLBoneState>) -> Self {
        let mut statements = vec![];
        for state in states {
            statements.extend(MPLPoseStatement::from_quaternion(
                &state.bone_name_en(),
                state.quaternion(),
            ));
        }
        Self::new(name.to_string(), statements.into_iter().collect())
    }
}
