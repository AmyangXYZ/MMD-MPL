use crate::{
    mpl::MPLBoneFrame,
    utils::{Quaternion, Vector3},
    with_bone_db,
};
use encoding_rs::SHIFT_JIS;

#[derive(Debug, Clone)]
pub struct VPDReader;

impl VPDReader {
    pub fn new() -> Self {
        Self
    }

    /// Parse VPD data from bytes and extract bone frames
    pub fn read(&self, vpd_data: &[u8]) -> Result<Vec<MPLBoneFrame>, Box<dyn std::error::Error>> {
        let (data, _, _) = SHIFT_JIS.decode(vpd_data);
        let data = data.to_string();

        if !data.starts_with("Vocaloid Pose Data file") {
            return Err("VPD signature is not valid.".into());
        }

        let mut index = "Vocaloid Pose Data file".len();
        let chars: Vec<char> = data.chars().collect();

        // Skip model name and bone count
        index = self.skip_statement(&chars, index);
        index = self.skip_statement(&chars, index);

        let mut bone_frames = Vec::new();

        while index < chars.len() {
            index = self.skip_whitespace_and_comments(&chars, index);

            if index >= chars.len() {
                break;
            }

            let (type_and_index, new_index) = self.read_until_open_bracket(&chars, index);
            index = new_index;

            if type_and_index.starts_with("Bone") {
                let (bone_name_jp, new_index) = self.read_until_line_end(&chars, index);
                index = new_index;

                let (position, new_index) = self.parse_position(&chars, index);
                index = new_index;

                let (rotation, new_index) = self.parse_rotation(&chars, index);
                index = new_index;

                let bone_name_en = self.convert_bone_name_jp_to_en(&bone_name_jp);
                let bone_frame = MPLBoneFrame::new(bone_name_en, bone_name_jp, position, rotation);
                bone_frames.push(bone_frame);
            } else if type_and_index.starts_with("Morph") {
                let (_, new_index) = self.read_until_line_end(&chars, index);
                index = new_index;
                index = self.skip_statement(&chars, index);
            }

            index = self.skip_until_close_bracket(&chars, index);
        }

        Ok(bone_frames)
    }

    /// Skip whitespace and comments
    fn skip_whitespace_and_comments(&self, chars: &[char], index: usize) -> usize {
        let mut i = index;
        while i < chars.len() {
            let c = chars[i];
            if c == ' ' || c == '\t' || c == '\r' || c == '\n' {
                i += 1;
            } else if c == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
                while i < chars.len() && chars[i] != '\r' && chars[i] != '\n' {
                    i += 1;
                }
            } else {
                break;
            }
        }
        i
    }

    /// Skip a statement (until semicolon)
    fn skip_statement(&self, chars: &[char], index: usize) -> usize {
        let mut i = index;
        i = self.skip_whitespace_and_comments(chars, i);

        while i < chars.len() {
            let c = chars[i];
            if c == ';' {
                i += 1;
                break;
            }
            i += 1;
        }
        i
    }

    /// Read until open bracket
    fn read_until_open_bracket(&self, chars: &[char], index: usize) -> (String, usize) {
        let mut result = String::new();
        let mut i = index;

        while i < chars.len() {
            let c = chars[i];
            if c == '{' {
                i += 1;
                break;
            }
            result.push(c);
            i += 1;
        }

        (result, i)
    }

    /// Read until line ending
    fn read_until_line_end(&self, chars: &[char], index: usize) -> (String, usize) {
        let mut result = String::new();
        let mut i = index;

        while i < chars.len() {
            let c = chars[i];
            if c == '\r' || c == '\n' {
                break;
            }
            result.push(c);
            i += 1;
        }

        (result, i)
    }

    /// Parse position vector
    fn parse_position(&self, chars: &[char], index: usize) -> (Vector3, usize) {
        let mut i = index;
        i = self.skip_whitespace_and_comments(chars, i);

        let mut position = Vector3::new(0.0, 0.0, 0.0);
        let mut values = Vec::new();
        let mut current_value = String::new();

        while i < chars.len() {
            let c = chars[i];
            if c == ';' {
                if !current_value.is_empty() {
                    values.push(current_value);
                }
                i += 1;
                break;
            } else if c == ',' {
                values.push(current_value);
                current_value = String::new();
            } else {
                current_value.push(c);
            }
            i += 1;
        }

        if values.len() >= 3 {
            if let (Ok(x), Ok(y), Ok(z)) = (
                values[0].trim().parse::<f32>(),
                values[1].trim().parse::<f32>(),
                values[2].trim().parse::<f32>(),
            ) {
                position = Vector3::new(x, y, z);
            }
        }

        (position, i)
    }

    /// Parse rotation quaternion
    fn parse_rotation(&self, chars: &[char], index: usize) -> (Quaternion, usize) {
        let mut i = index;
        i = self.skip_whitespace_and_comments(chars, i);

        let mut rotation = Quaternion::new(0.0, 0.0, 0.0, 1.0);
        let mut values = Vec::new();
        let mut current_value = String::new();

        while i < chars.len() {
            let c = chars[i];
            if c == ';' {
                if !current_value.is_empty() {
                    values.push(current_value);
                }
                i += 1;
                break;
            } else if c == ',' {
                values.push(current_value);
                current_value = String::new();
            } else {
                current_value.push(c);
            }
            i += 1;
        }

        if values.len() >= 4 {
            if let (Ok(x), Ok(y), Ok(z), Ok(w)) = (
                values[0].trim().parse::<f32>(),
                values[1].trim().parse::<f32>(),
                values[2].trim().parse::<f32>(),
                values[3].trim().parse::<f32>(),
            ) {
                rotation = Quaternion::new(x, y, z, w);
            }
        }

        (rotation, i)
    }

    /// Skip until closing bracket
    fn skip_until_close_bracket(&self, chars: &[char], index: usize) -> usize {
        let mut i = index;
        while i < chars.len() {
            if chars[i] == '}' {
                i += 1;
                break;
            }
            i += 1;
        }
        i
    }

    /// Convert Japanese bone name to English using the bone database
    fn convert_bone_name_jp_to_en(&self, jp_name: &str) -> String {
        let clean_name = jp_name.trim_matches('\0');

        if let Some(english_name) = with_bone_db(|db| {
            for bone in db.bones() {
                if let Some(jp_bone_name) = db.japanese_name(bone) {
                    if jp_bone_name == clean_name {
                        return Some(bone.to_string());
                    }
                }
            }
            None
        }) {
            return english_name;
        }

        clean_name.to_string()
    }
}
