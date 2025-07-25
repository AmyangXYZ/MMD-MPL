use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::utils::{Quaternion, Vector3};
use std::cell::OnceCell;
use std::collections::HashMap;

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MPLBoneState {
    bone_name_en: String,
    bone_name_jp: String,
    position: Vector3,
    quaternion: Quaternion,
}

#[wasm_bindgen]
impl MPLBoneState {
    #[wasm_bindgen(constructor)]
    pub fn new(
        bone_name_en: String,
        bone_name_jp: String,
        position: Vector3,
        quaternion: Quaternion,
    ) -> Self {
        Self {
            bone_name_en,
            bone_name_jp,
            position,
            quaternion,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn bone_name_en(&self) -> String {
        self.bone_name_en.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn bone_name_jp(&self) -> String {
        self.bone_name_jp.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn position(&self) -> Vector3 {
        self.position
    }

    #[wasm_bindgen(getter)]
    pub fn quaternion(&self) -> Quaternion {
        self.quaternion
    }
}

#[derive(Debug, Clone)]
pub struct ActionRule {
    pub axis: Vector3,
    pub limit: f32,
}

pub struct BoneActionDatabase {
    rules: HashMap<String, HashMap<String, HashMap<String, ActionRule>>>,
    all_bones: Vec<String>,
    bone_actions: HashMap<String, Vec<String>>,
    bone_action_directions: HashMap<String, Vec<String>>,
    bone_translations: HashMap<String, String>, // English -> Japanese
}

impl BoneActionDatabase {
    fn new() -> Self {
        let rules = Self::build_rules();
        Self::from_rules(rules)
    }

    fn from_rules(rules: HashMap<String, HashMap<String, HashMap<String, ActionRule>>>) -> Self {
        let all_bones: Vec<String> = rules.keys().cloned().collect();
        let mut bone_actions = HashMap::new();
        let mut bone_action_directions = HashMap::new();

        for (bone, actions) in &rules {
            bone_actions.insert(bone.clone(), actions.keys().cloned().collect());

            for (action, directions) in actions {
                let key = format!("{}_{}", bone, action);
                bone_action_directions.insert(key, directions.keys().cloned().collect());
            }
        }

        let bone_translations = Self::build_translations();

        Self {
            rules,
            all_bones,
            bone_actions,
            bone_action_directions,
            bone_translations,
        }
    }

    // Public API
    pub fn bones(&self) -> &[String] {
        &self.all_bones
    }

    pub fn actions(&self, bone: &str) -> Option<&[String]> {
        self.bone_actions.get(bone).map(|v| v.as_slice())
    }

    pub fn directions(&self, bone: &str, action: &str) -> Option<&[String]> {
        let key = format!("{}_{}", bone, action);
        self.bone_action_directions.get(&key).map(|v| v.as_slice())
    }

    pub fn validate(
        &self,
        bone: &str,
        action: &str,
        direction: &str,
        degrees: f32,
    ) -> Result<(), String> {
        match self
            .rules
            .get(bone)
            .and_then(|actions| actions.get(action))
            .and_then(|directions| directions.get(direction))
        {
            Some(rule) if degrees <= rule.limit => Ok(()),
            Some(rule) => Err(format!(
                "Max {} degrees for {} {} {}",
                rule.limit, bone, action, direction
            )),
            None => Err(format!(
                "Invalid combination: {} {} {}",
                bone, action, direction
            )),
        }
    }

    pub fn get_rule(&self, bone: &str, action: &str, direction: &str) -> Option<&ActionRule> {
        self.rules
            .get(bone)
            .and_then(|actions| actions.get(action))
            .and_then(|directions| directions.get(direction))
    }

    pub fn japanese_name(&self, bone_en: &str) -> Option<&str> {
        self.bone_translations.get(bone_en).map(|s| s.as_str())
    }

    pub fn english_name(&self, bone_jp: &str) -> Option<&str> {
        self.bone_translations
            .iter()
            .find(|(_, v)| v == &&bone_jp.to_string())
            .map(|(k, _)| k.as_str())
    }

    fn build_translations() -> HashMap<String, String> {
        macro_rules! translations {
            { $( $en:literal => $jp:literal ),* $(,)? } => {{
                let mut map = HashMap::new();
                $( map.insert($en.to_string(), $jp.to_string()); )*
                map
            }};
        }

        translations! {
            "base" => "全ての親",
            "center" => "センター",
            "upper_body" => "上半身",
            "upper_body2" => "上半身2",
            "lower_body" => "下半身",
            "waist" => "腰",
            "neck" => "首",
            "head" => "頭",
            "shoulder_l" => "左肩",
            "shoulder_r" => "右肩",
            "arm_l" => "左腕",
            "arm_r" => "右腕",
            "arm_twist_l" => "左腕捩",
            "arm_twist_r" => "右腕捩",
            "elbow_l" => "左ひじ",
            "elbow_r" => "右ひじ",
            "wrist_l" => "左手首",
            "wrist_r" => "右手首",
            "wrist_twist_l" => "左手捩",
            "wrist_twist_r" => "右手捩",
            "leg_l" => "左足",
            "leg_r" => "右足",
            "knee_l" => "左ひざ",
            "knee_r" => "右ひざ",
            "ankle_l" => "左足首",
            "ankle_r" => "右足首",
            "toe_l" => "左足先EX",
            "toe_r" => "右足先EX",
            "thumb_0_l" => "左親指０",
            "thumb_1_l" => "左親指１",
            "thumb_2_l" => "左親指２",
            "index_0_l" => "左人指１",
            "index_1_l" => "左人指２",
            "index_2_l" => "左人指３",
            "middle_0_l" => "左中指０",
            "middle_1_l" => "左中指２",
            "middle_2_l" => "左中指３",
            "ring_0_l" => "左薬指１",
            "ring_1_l" => "左薬指２",
            "ring_2_l" => "左薬指３",
            "pinky_0_l" => "左小指１",
            "pinky_1_l" => "左小指２",
            "pinky_2_l" => "左小指３",
            "thumb_0_r" => "右親指０",
            "thumb_1_r" => "右親指１",
            "thumb_2_r" => "右親指２",
            "index_0_r" => "右人指１",
            "index_1_r" => "右人指２",
            "index_2_r" => "右人指３",
            "middle_0_r" => "右中指１",
            "middle_1_r" => "右中指２",
            "middle_2_r" => "右中指３",
            "ring_0_r" => "右薬指１",
            "ring_1_r" => "右薬指２",
            "ring_2_r" => "右薬指３",
            "pinky_0_r" => "右小指１",
            "pinky_1_r" => "右小指２",
            "pinky_2_r" => "右小指３",
        }
    }

    fn build_rules() -> HashMap<String, HashMap<String, HashMap<String, ActionRule>>> {
        macro_rules! rules {
            {
                $( $bone:literal => {
                    $( $action:literal => {
                        $( $direction:literal => [$x:expr, $y:expr, $z:expr], $limit:expr ),* $(,)?
                    } ),* $(,)?
                } ),* $(,)?
            } => {{
                let mut rules = HashMap::new();
                $(
                    let mut actions = HashMap::new();
                    $(
                        let mut directions = HashMap::new();
                        $(
                            directions.insert($direction.to_string(), ActionRule {
                                axis: Vector3::new($x, $y, $z),
                                limit: $limit,
                            });
                        )*
                        actions.insert($action.to_string(), directions);
                    )*
                    rules.insert($bone.to_string(), actions);
                )*
                rules
            }};
        }

        rules! {
            "base" => {
                "bend" => {
                    "forward" => [-1.0, 0.0, 0.0], 90.0,
                    "backward" => [1.0, 0.0, 0.0], 90.0,
                },
                "turn" => {
                    "left" => [0.0, -1.0, 0.0], 180.0,
                    "right" => [0.0, 1.0, 0.0], 180.0,
                },
                "sway" => {
                    "left" => [0.0, 0.0, -1.0], 180.0,
                    "right" => [0.0, 0.0, 1.0], 180.0,
                },
            },
            "center" => {
                "bend" => {
                    "forward" => [-1.0, 0.0, 0.0], 180.0,
                    "backward" => [1.0, 0.0, 0.0], 180.0,
                },
                "turn" => {
                    "left" => [0.0, -1.0, 0.0], 180.0,
                    "right" => [0.0, 1.0, 0.0], 180.0,
                },
                "sway" => {
                    "left" => [0.0, 0.0, -1.0], 180.0,
                    "right" => [0.0, 0.0, 1.0], 180.0,
                }
            },
            "head" => {
                "bend" => {
                    "forward" => [-1.0, 0.0, 0.0], 60.0,
                    "backward" => [1.0, 0.0, 0.0], 90.0,
                },
                "turn" => {
                    "left" => [0.0, -1.0, 0.0], 90.0,
                    "right" => [0.0, 1.0, 0.0], 90.0,
                },
                "sway" => {
                    "left" => [0.0, 0.0, -1.0], 30.0,
                    "right" => [0.0, 0.0, 1.0], 30.0,
                },
            },
            "neck" => {
                "bend" => {
                    "forward" => [-1.0, 0.0, 0.0], 45.0,
                    "backward" => [1.0, 0.0, 0.0], 60.0,
                },
                "turn" => {
                    "left" => [0.0, -1.0, 0.0], 75.0,
                    "right" => [0.0, 1.0, 0.0], 75.0,
                },
                "sway" => {
                    "left" => [0.0, 0.0, -1.0], 30.0,
                    "right" => [0.0, 0.0, 1.0], 30.0,
                },
            },
            "upper_body" => {
                "bend" => {
                    "forward" => [-1.0, 0.0, 0.0], 45.0,
                    "backward" => [1.0, 0.0, 0.0], 45.0,
                },
                "turn" => {
                    "left" => [0.0, -1.0, 0.0], 45.0,
                    "right" => [0.0, 1.0, 0.0], 45.0,
                },
                "sway" => {
                    "left" => [0.0, 0.0, -1.0], 45.0,
                    "right" => [0.0, 0.0, 1.0], 45.0,
                },
            },
            "upper_body2" => {
                "bend" => {
                    "forward" => [-1.0, 0.0, 0.0], 45.0,
                    "backward" => [1.0, 0.0, 0.0], 45.0,
                },
                "turn" => {
                    "left" => [0.0, -1.0, 0.0], 45.0,
                    "right" => [0.0, 1.0, 0.0], 45.0,
                },
                "sway" => {
                    "left" => [0.0, 0.0, -1.0], 45.0,
                    "right" => [0.0, 0.0, 1.0], 45.0,
                },
            },
            "lower_body" => {
                "bend" => {
                    "forward" => [-1.0, 0.0, 0.0], 45.0,
                    "backward" => [1.0, 0.0, 0.0], 45.0,
                },
                "turn" => {
                    "left" => [0.0, -1.0, 0.0], 45.0,
                    "right" => [0.0, 1.0, 0.0], 45.0,
                },
                "sway" => {
                    "left" => [0.0, 0.0, -1.0], 45.0,
                    "right" => [0.0, 0.0, 1.0], 45.0,
                },
            },
            "waist" => {
                "bend" => {
                    "forward" => [-1.0, 0.0, 0.0], 90.0,
                    "backward" => [1.0, 0.0, 0.0], 90.0,
                },
                "turn" => {
                    "left" => [0.0, -1.0, 0.0], 45.0,
                    "right" => [0.0, 1.0, 0.0], 45.0,
                },
                "sway" => {
                    "left" => [0.0, 0.0, -1.0], 30.0,
                    "right" => [0.0, 0.0, 1.0], 30.0,
                },
            },

            "shoulder_l" => {
                "bend" => {
                    "forward" => [0.0, 0.0, -1.0], 90.0,
                    "backward" => [0.0, 0.0, 1.0], 90.0,
                },
                "sway" => {
                    "left" => [0.0, -1.0, 0.0], 90.0,
                    "right" => [0.0, 1.0, 0.0], 90.0,
                },
            },
            "shoulder_r" => {
                "bend" => {
                    "forward" => [0.0, 0.0, 1.0], 90.0,
                    "backward" => [0.0, 0.0, -1.0], 90.0,
                },
                "sway" => {
                    "left" => [0.0, 1.0, 0.0], 90.0,
                    "right" => [0.0, -1.0, 0.0], 90.0,
                },
            },
            "arm_l" => {
                "bend" => {
                    "forward" => [0.0, 0.0, -1.0], 90.0,
                    "backward" => [0.0, 0.0, 1.0], 90.0,
                },
                "sway" => {
                    "left" => [0.0, -1.0, 0.0], 90.0,
                    "right" => [0.0, 1.0, 0.0], 90.0,
                },
            },
            "arm_r" => {
                "bend" => {
                    "forward" => [0.0, 0.0, 1.0], 45.0,
                    "backward" => [0.0, 0.0, -1.0], 180.0,
                },
                "sway" => {
                    "left" => [0.0, -1.0, 0.0], 90.0,
                    "right" => [0.0, 1.0, 0.0], 90.0,
                },
            },
            "arm_twist_l" => {
                "turn" => {
                    "left" => [0.0, -1.0, 0.0], 90.0,
                    "right" => [0.0, 1.0, 0.0], 90.0,
                },
            },
            "arm_twist_r" => {
                "turn" => {
                    "left" => [0.0, -1.0, 0.0], 90.0,
                    "right" => [0.0, 1.0, 0.0], 90.0,
                },
            },
            "elbow_l" => {
                "bend" => {
                    "forward" => [1.0, 1.0, 0.0], 135.0,
                },
            },
            "elbow_r" => {
                "bend" => {
                    "forward" => [1.0, -1.0, 0.0], 135.0,
                },
            },
            "wrist_l" => {
                "bend" => {
                    "forward" => [0.0, 0.0, -1.0], 60.0,
                    "backward" => [1.0, 0.0, -1.0], 30.0,
                },
                "sway" => {
                    "left" => [-1.0, 1.0, 0.0], 15.0,
                    "right" => [1.0, 1.0, 0.0], 15.0,
                },
            },
            "wrist_r" => {
                "bend" => {
                    "forward" => [0.0, 0.0, 1.0], 60.0,
                    "backward" => [-1.0, 0.0, -1.0], 30.0,
                },
                "sway" => {
                    "left" => [-1.0, -1.0, 0.0], 15.0,
                    "right" => [1.0, -1.0, 0.0], 15.0,
                },
            },
            "wrist_twist_l" => {
                "turn" => {
                    "left" => [0.0, -1.0, 0.0], 90.0,
                    "right" => [0.0, 1.0, 0.0], 90.0,
                },
            },
            "wrist_twist_r" => {
                "turn" => {
                    "left" => [0.0, -1.0, 0.0], 90.0,
                    "right" => [0.0, 1.0, 0.0], 90.0,
                },
            },
            "wrist_twist_l" => {
                "turn" => {
                    "left" => [0.0, -1.0, 0.0], 90.0,
                    "right" => [0.0, 1.0, 0.0], 90.0,
                },
            },
            "wrist_twist_r" => {
                "turn" => {
                    "left" => [0.0, -1.0, 0.0], 90.0,
                    "right" => [0.0, 1.0, 0.0], 90.0,
                },
            },
            "wrist_twist_l" => {
                "turn" => {
                    "left" => [0.0, -1.0, 0.0], 90.0,
                    "right" => [0.0, 1.0, 0.0], 90.0,
                },
            },
            "wrist_twist_r" => {
                "turn" => {
                    "left" => [0.0, -1.0, 0.0], 90.0,
                    "right" => [0.0, 1.0, 0.0], 90.0,
                },
            },
            "leg_l" => {
                "bend" => {
                    "forward" => [1.0, 0.0, 0.0], 90.0,
                    "backward" => [-1.0, 0.0, 0.0], 90.0,
                },
            },
            "leg_l" => {
                "bend" => {
                    "forward" => [1.0, 0.0, 0.0], 90.0,
                    "backward" => [-1.0, 0.0, 0.0], 90.0,
                },
                "turn" => {
                    "left" => [0.0, -1.0, 0.0], 90.0,
                    "right" => [0.0, 1.0, 0.0], 90.0,
                },
                "sway" => {
                    "left" => [0.0, 0.0, 1.0], 180.0,
                    "right" => [0.0, 0.0, -1.0], 30.0,
                },
            },
            "leg_r" => {
                "bend" => {
                    "forward" => [1.0, 0.0, 0.0], 90.0,
                    "backward" => [-1.0, 0.0, 0.0], 90.0,
                },
                "turn" => {
                    "left" => [0.0, -1.0, 0.0], 90.0,
                    "right" => [0.0, 1.0, 0.0], 90.0,
                },
                "sway" => {
                    "left" => [0.0, 0.0, 1.0], 30.0,
                    "right" => [0.0, 0.0, -1.0], 180.0,
                },
            },
            "knee_l" => {
                      "bend" => {
                    "backward" => [-1.0, 0.0, 0.0], 135.0,
                },
            },
            "knee_r" => {
                "bend" => {
                    "backward" => [-1.0, 0.0, 0.0], 135.0,
                },
            },
            "ankle_l" => {
                "bend" => {
                    "forward" => [-1.0, 0.0, 0.0], 60.0,
                    "backward" => [1.0, 0.0, 0.0], 60.0,
                },
                "turn" => {
                    "left" => [0.0, -1.0, 0.0], 90.0,
                    "right" => [0.0, 1.0, 0.0], 90.0,
                },
                "sway" => {
                    "left" => [0.0, 0.0, 1.0], 30.0,
                    "right" => [0.0, 0.0, -1.0], 30.0,
                },
            },
            "ankle_r" => {
                "bend" => {
                    "forward" => [-1.0, 0.0, 0.0], 60.0,
                    "backward" => [1.0, 0.0, 0.0], 60.0,
                },
                "turn" => {
                    "left" => [0.0, -1.0, 0.0], 90.0,
                    "right" => [0.0, 1.0, 0.0], 90.0,
                },
                "sway" => {
                    "left" => [0.0, 0.0, 1.0], 30.0,
                    "right" => [0.0, 0.0, -1.0], 30.0,
                },
            },
            "toe_l" => {
                "bend" => {
                    "forward" => [-1.0, 0.0, 0.0], 30.0,
                    "backward" => [1.0, 0.0, 0.0], 30.0,
                },
            },
            "toe_r" => {
                "bend" => {
                    "forward" => [-1.0, 0.0, 0.0], 30.0,
                    "backward" => [1.0, 0.0, 0.0], 30.0,
                },
            },
            "thumb_0_l" => {
                "bend" => {
                    "forward" => [-1.0, -1.0, 0.0], 90.0,
                    "backward" => [1.0, 1.0, 0.0], 15.0,
                },
            },
            "thumb_1_l" => {
                "bend" => {
                    "forward" => [-1.0, -1.0, 0.0], 90.0,
                    "backward" => [1.0, 1.0, 0.0], 15.0,
                },
            },
            "thumb_2_l" => {
                "bend" => {
                    "forward" => [-1.0, -1.0, 0.0], 90.0,
                    "backward" => [1.0, 1.0, 0.0], 15.0,
                },
            },
            "index_0_l" => {
                "bend" => {
                    "forward" => [0.0, 0.0, -1.0], 90.0,
                    "backward" => [0.0, 0.0, 1.0], 15.0,
                },
                "sway" => {
                    "left" => [0.0, 0.0, 1.0], 45.0,
                    "right" => [0.0, 0.0, -1.0], 45.0,
                },
            },
            "index_1_l" => {
                "bend" => {
                    "forward" => [0.0, 0.0, -1.0], 90.0,
                    "backward" => [0.0, 0.0, 1.0], 15.0,
                },
            },
            "index_2_l" => {
                "bend" => {
                    "forward" => [0.0, 0.0, -1.0], 90.0,
                    "backward" => [0.0, 0.0, 1.0], 15.0,
                },
            },
            "middle_0_l" => {
                "bend" => {
                    "forward" => [0.0, 0.0, -1.0], 90.0,
                    "backward" => [0.0, 0.0, 1.0], 15.0,
                },
                "sway" => {
                    "left" => [0.0, 0.0, 1.0], 45.0,
                    "right" => [0.0, 0.0, -1.0], 45.0,
                },
            },
            "middle_0_l" => {
                "bend" => {
                    "forward" => [0.0, 0.0, -1.0], 90.0,
                    "backward" => [0.0, 0.0, 1.0], 15.0,
                },
                "sway" => {
                    "left" => [0.0, 0.0, 1.0], 45.0,
                    "right" => [0.0, 0.0, -1.0], 45.0,
                },
            },
            "middle_1_l" => {
                "bend" => {
                    "forward" => [0.0, 0.0, -1.0], 90.0,
                    "backward" => [0.0, 0.0, 1.0], 15.0,
                },
            },
            "middle_2_l" => {
                "bend" => {
                    "forward" => [0.0, 0.0, -1.0], 90.0,
                    "backward" => [0.0, 0.0, 1.0], 15.0,
                },
            },
            "ring_0_l" => {
                "bend" => {
                    "forward" => [0.0, 0.0, -1.0], 90.0,
                    "backward" => [0.0, 0.0, 1.0], 15.0,
                },
                "sway" => {
                    "left" => [0.0, 0.0, 1.0], 45.0,
                    "right" => [0.0, 0.0, -1.0], 45.0,
                },
            },
            "ring_1_l" => {
                "bend" => {
                    "forward" => [0.0, 0.0, -1.0], 90.0,
                    "backward" => [0.0, 0.0, 1.0], 15.0,
                },
            },
            "ring_2_l" => {
                "bend" => {
                    "forward" => [0.0, 0.0, -1.0], 90.0,
                    "backward" => [0.0, 0.0, 1.0], 15.0,
                },
            },
            "pinky_0_l" => {
                "bend" => {
                    "forward" => [0.0, 0.0, -1.0], 90.0,
                    "backward" => [0.0, 0.0, 1.0], 15.0,
                },
                "sway" => {
                    "left" => [0.0, 0.0, 1.0], 45.0,
                    "right" => [0.0, 0.0, -1.0], 45.0,
                },
            },
            "pinky_1_l" => {
                "bend" => {
                    "forward" => [0.0, 0.0, -1.0], 90.0,
                    "backward" => [0.0, 0.0, 1.0], 15.0,
                },
            },
            "pinky_2_l" => {
                "bend" => {
                    "forward" => [0.0, 0.0, -1.0], 90.0,
                    "backward" => [0.0, 0.0, 1.0], 15.0,
                },
            },

            "thumb_0_r" => {
                "bend" => {
                    "forward" => [-1.0, 1.0, 0.0], 90.0,
                    "backward" => [1.0, -1.0, 0.0], 15.0,
                },
                "sway" => {
                    "left" => [0.0, 0.0, 1.0], 45.0,
                    "right" => [0.0, 0.0, -1.0], 45.0,
                },
            },
            "thumb_1_r" => {
                "bend" => {
                    "forward" => [-1.0, 1.0, 0.0], 90.0,
                    "backward" => [1.0, -1.0, 0.0], 15.0,
                },
            },
            "thumb_2_r" => {
                "bend" => {
                    "forward" => [-1.0, 1.0, 0.0], 90.0,
                    "backward" => [1.0, -1.0, 0.0], 15.0,
                },
            },
            "index_0_r" => {
                "bend" => {
                    "forward" => [0.0, 0.0, 1.0], 90.0,
                    "backward" => [0.0, 0.0, -1.0], 15.0,
                },
                "sway" => {
                    "left" => [1.0, 0.0, 0.0], 15.0,
                    "right" => [-1.0, 0.0, 0.0], 15.0,
                },
            },
            "index_1_r" => {
                "bend" => {
                    "forward" => [0.0, 0.0, 1.0], 90.0,
                    "backward" => [0.0, 0.0, -1.0], 15.0,
                },
            },
            "index_2_r" => {
                "bend" => {
                    "forward" => [0.0, 0.0, 1.0], 90.0,
                    "backward" => [0.0, 0.0, -1.0], 15.0,
                },
            },
            "middle_0_r" => {
                "bend" => {
                    "forward" => [0.0, 0.0, 1.0], 90.0,
                    "backward" => [0.0, 0.0, -1.0], 15.0,
                },
                "sway" => {
                    "left" => [1.0, 0.0, 0.0], 45.0,
                    "right" => [-1.0, 0.0, 0.0], 45.0,
                },
            },
            "middle_1_r" => {
                "bend" => {
                    "forward" => [0.0, 0.0, 1.0], 90.0,
                    "backward" => [0.0, 0.0, -1.0], 15.0,
                },
            },
            "middle_2_r" => {
                "bend" => {
                    "forward" => [0.0, 0.0, 1.0], 90.0,
                    "backward" => [0.0, 0.0, -1.0], 15.0,
                },
            },
            "ring_0_r" => {
                "bend" => {
                    "forward" => [0.0, 0.0, 1.0], 90.0,
                    "backward" => [0.0, 0.0, -1.0], 15.0,
                },
                "sway" => {
                    "left" => [1.0, 0.0, 0.0], 45.0,
                    "right" => [-1.0, 0.0, 0.0], 45.0,
                },
            },
            "ring_1_r" => {
                "bend" => {
                    "forward" => [0.0, 0.0, 1.0], 90.0,
                    "backward" => [0.0, 0.0, -1.0], 15.0,
                },
            },
            "ring_2_r" => {
                "bend" => {
                    "forward" => [0.0, 0.0, 1.0], 90.0,
                    "backward" => [0.0, 0.0, -1.0], 15.0,
                },
            },
            "pinky_0_r" => {
                "bend" => {
                    "forward" => [0.0, 0.0, 1.0], 90.0,
                    "backward" => [0.0, 0.0, -1.0], 15.0,
                },
                "sway" => {
                    "left" => [1.0, 0.0, 0.0], 45.0,
                    "right" => [-1.0, 0.0, 0.0], 45.0,
                },
            },
            "pinky_1_r" => {
                "bend" => {
                    "forward" => [0.0, 0.0, 1.0], 90.0,
                    "backward" => [0.0, 0.0, -1.0], 15.0,
                },
            },
            "pinky_2_r" => {
                "bend" => {
                    "forward" => [0.0, 0.0, 1.0], 90.0,
                    "backward" => [0.0, 0.0, -1.0], 15.0,
                },
            },
        }
    }
}

thread_local! {
    static BONE_DB: OnceCell<BoneActionDatabase> = OnceCell::new();
}

pub fn with_bone_db<T>(f: impl FnOnce(&BoneActionDatabase) -> T) -> T {
    BONE_DB.with(|db| f(db.get_or_init(|| BoneActionDatabase::new())))
}
