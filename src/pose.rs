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
        let (normalized_x, normalized_y, normalized_z) =
            (normalized_axis.x, normalized_axis.y, normalized_axis.z);

        // Create quaternion from axis-angle
        let radians = self.degrees * (std::f32::consts::PI / 180.0);
        let half_angle = radians / 2.0;
        let sin = half_angle.sin();
        let cos = half_angle.cos();

        Quaternion::new(
            normalized_x * sin,
            normalized_y * sin,
            normalized_z * sin,
            cos,
        )
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

            states.push(MPLBoneState {
                bone_name_en: bone,
                bone_name_jp,
                position,
                quaternion: combined_quaternion,
            });
        }

        states
    }
}
