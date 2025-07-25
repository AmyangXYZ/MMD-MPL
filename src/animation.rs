use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MPLAnimationStatement {
    pub time: f32,
    pub poses: Vec<String>,
}

impl MPLAnimationStatement {
    pub fn from_str(text: &str) -> Result<Self, String> {
        let text = text.trim();

        let text = if text.ends_with(';') {
            &text[..text.len() - 1]
        } else {
            text
        };

        // Parse keyframe: "0.5: pose1 & pose2"
        let colon_pos = text
            .find(':')
            .ok_or("Keyframe must have format 'time: poses'")?;

        let time_str = text[..colon_pos].trim();
        let poses_text = text[colon_pos + 1..].trim();

        // Parse time
        let time = time_str
            .parse::<f32>()
            .map_err(|_| format!("Invalid time value: '{}'", time_str))?;

        if time < 0.0 {
            return Err("Time must be non-negative".to_string());
        }

        // Parse poses (split by &)
        let poses: Vec<String> = poses_text
            .split('&')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if poses.is_empty() {
            return Err("Keyframe must contain at least one pose".to_string());
        }

        Ok(Self { time, poses })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MPLAnimation {
    pub name: String,
    pub statements: Vec<MPLAnimationStatement>,
}

impl MPLAnimation {
    pub fn new(name: String, statements: Vec<MPLAnimationStatement>) -> Self {
        Self { name, statements }
    }
}
