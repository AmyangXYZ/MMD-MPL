use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::utils::{Quaternion, Vector3};

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MPLBoneFrame {
    name_en: String,
    name_jp: String,
    position: Vector3,
    rotation: Quaternion,
}

#[wasm_bindgen]
impl MPLBoneFrame {
    #[wasm_bindgen(constructor)]
    pub fn new(name_en: String, name_jp: String, position: Vector3, rotation: Quaternion) -> Self {
        Self {
            name_en,
            name_jp,
            position,
            rotation,
        }
    }
    #[wasm_bindgen(getter)]
    pub fn name_en(&self) -> String {
        self.name_en.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn name_jp(&self) -> String {
        self.name_jp.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn position(&self) -> Vector3 {
        self.position.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn rotation(&self) -> Quaternion {
        self.rotation.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MPLMorphFrame {
    pub name_en: String,
    pub name_jp: String,
    pub weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MPLKeyFrame {
    pub time: f32,
    pub bone_frames: Vec<MPLBoneFrame>,
    pub morph_frames: Vec<MPLMorphFrame>,
}

impl MPLKeyFrame {
    pub fn new(
        time: f32,
        bone_frames: Vec<MPLBoneFrame>,
        morph_frames: Vec<MPLMorphFrame>,
    ) -> Self {
        Self {
            time,
            bone_frames,
            morph_frames,
        }
    }
}
