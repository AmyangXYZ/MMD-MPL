#[derive(Debug, Clone)]
pub struct MPLAnimationFrame {
    pub poses: Vec<String>,
    pub duration: f32,
}

impl MPLAnimationFrame {
    pub fn from_str(text: &str) -> Result<Self, String> {
        let text = text.trim();

        // Parse duration: "pose1 & pose2 1.5s" or just "pose1 & pose2" (default 1s)
        let (poses_text, duration) = if let Some(space_pos) = text.rfind(' ') {
            let potential_duration = &text[space_pos + 1..];
            if potential_duration.ends_with('s') {
                let dur_str = &potential_duration[..potential_duration.len() - 1];
                match dur_str.parse::<f32>() {
                    Ok(dur) => {
                        if dur <= 0.0 {
                            return Err("Duration must be positive".to_string());
                        }
                        (&text[..space_pos], dur)
                    }
                    Err(_) => (text, 1.0), // Default if parse fails
                }
            } else {
                (text, 1.0) // Default if no 's' suffix
            }
        } else {
            (text, 1.0) // Default if no space found
        };

        // Parse poses (split by &)
        let poses: Vec<String> = poses_text
            .split('&')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if poses.is_empty() {
            return Err("Animation statement must contain at least one pose".to_string());
        }

        Ok(Self { poses, duration })
    }
}

#[derive(Debug, Clone)]
pub struct MPLAnimation {
    pub name: String,
    pub frames: Vec<MPLAnimationFrame>,
}

impl MPLAnimation {
    pub fn new(name: String, frames: Vec<MPLAnimationFrame>) -> Self {
        Self { name, frames }
    }
}
