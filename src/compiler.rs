use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    animation::MPLAnimation, mpl::MPLKeyFrame, pose::MPLPose, VMDReader, VMDWriter, VPDReader,
};

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
    pub fn to_key_frames(&self) -> Vec<MPLKeyFrame> {
        let mut key_frames = vec![];
        for name in &self.main {
            if self.animations.contains_key(name) {
                let anim = self.animations.get(name).unwrap();
                for statement in &anim.statements {
                    let mut bone_frames = vec![];
                    if statement.poses.len() == 1 {
                        let pose_name = statement.poses[0].clone();
                        let pose = self.poses.get(&pose_name).unwrap();
                        bone_frames.extend(pose.to_bone_frames());
                    } else {
                        let mut pose_statements = vec![];
                        for pose_name in &statement.poses {
                            let pose = self.poses.get(pose_name).unwrap();
                            pose_statements.extend(pose.statements.clone());
                        }
                        let pose = MPLPose::new("composite".to_string(), pose_statements);
                        bone_frames.extend(pose.to_bone_frames());
                    }
                    key_frames.push(MPLKeyFrame::new(statement.time, bone_frames, vec![]));
                }
            } else if self.poses.contains_key(name) {
                let pose = self.poses.get(name).unwrap();
                key_frames.push(MPLKeyFrame::new(0.0, pose.to_bone_frames(), vec![]));
            }
        }
        key_frames
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

    pub fn compile(&self, text: &str) -> Result<Vec<u8>, String> {
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
                        if script.animations.contains_key(&pose.name)
                            || script.poses.contains_key(&pose.name)
                        {
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
                        if script.animations.contains_key(&animation.name)
                            || script.poses.contains_key(&animation.name)
                        {
                            return Err(format!("Duplicate animation name: '{}'", animation.name));
                        }

                        if script.poses.contains_key(&animation.name) {
                            return Err(format!(
                                "Name '{}' already used by a pose",
                                animation.name
                            ));
                        }

                        // Validate that all referenced poses exist
                        for statement in &animation.statements {
                            for pose_name in &statement.poses {
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

        let key_frames = script.to_key_frames();
        let vmd_bytes = VMDWriter::new(key_frames);
        match vmd_bytes.write() {
            Ok(bytes) => Ok(bytes),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn from_vmd(&self, vmd_data: &[u8]) -> Result<String, String> {
        let mut script = String::new();
        let reader = VMDReader::new();
        let read_key_frames = reader.read(vmd_data).map_err(|e| e.to_string())?;

        let mut pose_map: std::collections::HashMap<String, MPLPose> =
            std::collections::HashMap::new(); // statements_content -> pose
        let mut animation_statements = Vec::new();
        let mut pose_counter = 0;
        let mut total_poses_processed = 0;

        for (i, keyframe) in read_key_frames.iter().enumerate() {
            let pose =
                MPLPose::from_bone_frames(&format!("pose_{}", i), keyframe.bone_frames.clone());

            total_poses_processed += 1;

            // Skip empty poses
            if pose.statements.is_empty() {
                continue;
            }

            let statements_content = pose.to_string(); // Just the statements, no pose name
            let pose_name = if let Some(existing_pose) = pose_map.get(&statements_content) {
                existing_pose.name.clone()
            } else {
                pose_counter += 1;
                let new_pose_name = format!("pose_{}", pose_counter - 1);
                let new_pose = MPLPose::new(new_pose_name.clone(), pose.statements.clone());
                pose_map.insert(statements_content, new_pose);
                new_pose_name
            };

            // Create animation statement
            animation_statements.push(format!("    {:.2}: {};\n", keyframe.time, pose_name));
        }

        // Add unique poses to script using to_block()
        let unique_poses_count = pose_map.len();
        for (_, pose) in pose_map {
            script.push_str(&pose.to_block());
            script.push_str("\n");
        }

        script.push_str("@animation extracted_animation {\n");
        for statement in animation_statements {
            script.push_str(&statement);
        }
        script.push_str("}\n\n");

        script.push_str("main {\n");
        script.push_str("    extracted_animation;\n");
        script.push_str("}\n");

        println!("Read {} keyframes", total_poses_processed);
        println!("Reversed to {} poses", unique_poses_count);

        Ok(script)
    }

    pub fn from_vpd(&self, vpd_data: &[u8]) -> Result<String, String> {
        let reader = VPDReader::new();
        match reader.read(vpd_data) {
            Ok(bone_frames) => {
                let pose = MPLPose::from_bone_frames(&"pose_1", bone_frames.clone());
                Ok(pose.to_script())
            }
            Err(e) => Err(e.to_string()),
        }
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
                    let parts: Vec<&str> = stmt_text.split(',').collect();

                    if parts.len() > 1 {
                        // Compound statement - extract bone from first part
                        let first_parts: Vec<&str> = parts[0].trim().split_whitespace().collect();
                        let bone_name = if first_parts.len() == 4 {
                            first_parts[0]
                        } else if first_parts.len() == 2 && first_parts[1] == "reset" {
                            first_parts[0]
                        } else {
                            return Err(format!(
                                "Line {} (statement 1): Invalid statement",
                                line_number + 1
                            ));
                        };

                        // Parse each part
                        for (i, part) in parts.iter().enumerate() {
                            let trimmed_part = part.trim();
                            if !trimmed_part.is_empty() {
                                let full_stmt = if i == 0 {
                                    trimmed_part.to_string()
                                } else {
                                    format!("{} {}", bone_name, trimmed_part)
                                };

                                match crate::pose::MPLPoseStatement::from_str(&full_stmt) {
                                    Ok(stmt) => statements.push(stmt),
                                    Err(e) => {
                                        return Err(format!(
                                            "Line {} (statement {}): {}",
                                            line_number + 1,
                                            i + 1,
                                            e
                                        ))
                                    }
                                }
                            }
                        }
                    } else {
                        // Single statement
                        match crate::pose::MPLPoseStatement::from_str(stmt_text) {
                            Ok(stmt) => statements.push(stmt),
                            Err(e) => return Err(format!("Line {}: {}", line_number + 1, e)),
                        }
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

        // if statements.is_empty() {
        //     return Err("Pose must contain at least one statement".to_string());
        // }

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
                    match crate::animation::MPLAnimationStatement::from_str(stmt_text) {
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

            // Parse animation/pose reference (must end with semicolon)
            if trimmed.ends_with(';') {
                let name = trimmed.trim_end_matches(';').trim();
                if !name.is_empty() {
                    animations.push(name.to_string());
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
