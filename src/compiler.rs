use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{animation::MPLAnimation, pose::MPLPose};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MPLScript {
    pub poses: HashMap<String, MPLPose>,
    pub animations: HashMap<String, MPLAnimation>,
    pub main: Vec<String>,
}

impl MPLScript {
    pub fn new() -> Self {
        Self {
            poses: HashMap::new(),
            animations: HashMap::new(),
            main: vec![],
        }
    }
}

enum BlockType {
    None,
    Pose,
    Animation,
    Main,
}

pub struct MPLCompiler {}

impl MPLCompiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(&self, text: &str) -> Result<MPLScript, String> {
        let mut in_block = false;
        let mut brace_count = 0;
        let mut current_block = String::new();
        let mut script = MPLScript::new();
        let mut block_type = BlockType::None;

        for (line_number, line) in text.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if trimmed.starts_with("@pose")
                || trimmed.starts_with("@animation")
                || trimmed.starts_with("main")
            {
                if in_block {
                    return Err(format!(
                        "Line {}: Nested block is not allowed",
                        line_number + 1
                    ));
                }
                in_block = true;
                match trimmed.split_whitespace().next().unwrap() {
                    "@pose" => block_type = BlockType::Pose,
                    "@animation" => block_type = BlockType::Animation,
                    "main" => block_type = BlockType::Main,
                    _ => return Err(format!("Line {}: Invalid block type", line_number + 1)),
                }
            }

            if !in_block {
                return Err(format!(
                    "Line {}: Invalid text outside of block",
                    line_number + 1
                ));
            }
            current_block.push_str(line);
            current_block.push('\n');

            brace_count += line.chars().filter(|&c| c == '{').count() as i32;
            brace_count -= line.chars().filter(|&c| c == '}').count() as i32;

            if brace_count < 0 {
                return Err(format!(
                    "Line {}: Unexpected closing brace",
                    line_number + 1
                ));
            }
            if brace_count == 0 {
                match block_type {
                    BlockType::Pose => {
                        let pose = self.parse_pose(&current_block)?;

                        // Check for duplicate pose name
                        if script.poses.contains_key(&pose.name) {
                            return Err(format!("Duplicate pose name: '{}'", pose.name));
                        }
                        if script.animations.contains_key(&pose.name) {
                            return Err(format!(
                                "Name '{}' already used by an animation",
                                pose.name
                            ));
                        }

                        script.poses.insert(pose.name.clone(), pose);
                    }
                    BlockType::Animation => {
                        let animation = self.parse_animation(&current_block)?;

                        // Check for duplicate animation name
                        if script.animations.contains_key(&animation.name) {
                            return Err(format!("Duplicate animation name: '{}'", animation.name));
                        }
                        if script.poses.contains_key(&animation.name) {
                            return Err(format!(
                                "Name '{}' already used by a pose",
                                animation.name
                            ));
                        }

                        // Validate that all referenced poses exist
                        for frame in &animation.frames {
                            for pose_name in &frame.poses {
                                if !script.poses.contains_key(pose_name) {
                                    return Err(format!(
                                        "Animation '{}' references unknown pose '{}'",
                                        animation.name, pose_name
                                    ));
                                }
                            }
                        }

                        script.animations.insert(animation.name.clone(), animation);
                    }
                    BlockType::Main => {
                        let main = self.parse_main(&current_block)?;

                        // Validate that all referenced animations/poses exist
                        for reference in &main {
                            if !script.animations.contains_key(reference)
                                && !script.poses.contains_key(reference)
                            {
                                return Err(format!(
                                    "Main references unknown animation or pose '{}'",
                                    reference
                                ));
                            }
                        }

                        script.main = main;
                    }
                    BlockType::None => {}
                }
                current_block.clear();
                in_block = false;
                block_type = BlockType::None;
            }
        }

        if in_block {
            return Err("Unclosed block".to_string());
        }

        Ok(script)
    }

    fn parse_pose(&self, text: &str) -> Result<MPLPose, String> {
        let mut pose_name = String::new();
        let mut statements = Vec::new();

        for (line_number, line) in text.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed == "{" || trimmed == "}" {
                continue;
            }

            if trimmed.starts_with("@pose") {
                let name_part = trimmed.trim_end_matches('{').trim();
                pose_name = name_part
                    .split_whitespace()
                    .nth(1)
                    .ok_or(format!("Line {}: Missing pose name", line_number + 1))?
                    .to_string();
                continue;
            }

            if trimmed.ends_with(';') {
                let stmt_text = trimmed.trim_end_matches(';').trim();
                if !stmt_text.is_empty() {
                    match crate::pose::MPLPoseStatement::from_str(stmt_text) {
                        Ok(stmt) => statements.push(stmt),
                        Err(e) => return Err(format!("Line {}: {}", line_number + 1, e)),
                    }
                }
            } else {
                return Err(format!(
                    "Line {}: Statement must end with semicolon",
                    line_number + 1
                ));
            }
        }

        if pose_name.is_empty() {
            return Err("No pose declaration found".to_string());
        }

        if statements.is_empty() {
            return Err("Pose must contain at least one statement".to_string());
        }

        Ok(MPLPose::new(pose_name, statements))
    }

    fn parse_animation(&self, text: &str) -> Result<MPLAnimation, String> {
        let mut animation_name = String::new();
        let mut statements = Vec::new();

        for (line_number, line) in text.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed == "{" || trimmed == "}" {
                continue;
            }

            if trimmed.starts_with("@animation") {
                let name_part = trimmed.trim_end_matches('{').trim();
                animation_name = name_part
                    .split_whitespace()
                    .nth(1)
                    .ok_or(format!("Line {}: Missing animation name", line_number + 1))?
                    .to_string();
                continue;
            }

            if trimmed.ends_with(';') {
                let stmt_text = trimmed.trim_end_matches(';').trim();
                if !stmt_text.is_empty() {
                    match crate::animation::MPLAnimationFrame::from_str(stmt_text) {
                        Ok(stmt) => statements.push(stmt),
                        Err(e) => return Err(format!("Line {}: {}", line_number + 1, e)),
                    }
                }
            } else {
                return Err(format!(
                    "Line {}: Statement must end with semicolon",
                    line_number + 1
                ));
            }
        }

        if animation_name.is_empty() {
            return Err("No animation declaration found".to_string());
        }

        if statements.is_empty() {
            return Err("Animation must contain at least one statement".to_string());
        }

        Ok(MPLAnimation::new(animation_name, statements))
    }

    fn parse_main(&self, text: &str) -> Result<Vec<String>, String> {
        let mut animations = Vec::new();

        for (line_number, line) in text.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed == "{" || trimmed == "}" {
                continue;
            }

            // Skip main declaration
            if trimmed == "main" || trimmed.starts_with("main") {
                continue;
            }

            // Parse animation reference (must end with semicolon)
            if trimmed.ends_with(';') {
                let animation_name = trimmed.trim_end_matches(';').trim();
                if !animation_name.is_empty() {
                    animations.push(animation_name.to_string());
                }
            } else {
                return Err(format!(
                    "Line {}: Animation reference must end with semicolon",
                    line_number + 1
                ));
            }
        }

        if animations.is_empty() {
            return Err("Main block must contain at least one animation reference".to_string());
        }

        Ok(animations)
    }
}
