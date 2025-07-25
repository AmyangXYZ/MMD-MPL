mod animation;
mod bone;
mod compiler;
mod pose;
mod utils;

pub use bone::*;
pub use compiler::MPLCompiler;
pub use pose::{MPLPose, MPLPoseStatement};
pub use utils::{Quaternion, Vector3};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmMPLCompiler {
    compiler: MPLCompiler,
}

#[wasm_bindgen]
impl WasmMPLCompiler {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            compiler: MPLCompiler::new(),
        }
    }

    #[wasm_bindgen]
    pub fn compile(&self, script: &str) -> Result<Vec<MPLBoneState>, String> {
        let result = self.compiler.compile(script)?;
        Ok(result)
    }

    #[wasm_bindgen]
    pub fn reverse_compile(&self, name: &str, states: Vec<MPLBoneState>) -> String {
        MPLPose::from_bone_states(name, states).to_string()
    }

    #[wasm_bindgen]
    pub fn get_all_bones(&self) -> Vec<String> {
        with_bone_db(|db| db.bones().to_vec())
    }

    #[wasm_bindgen]
    pub fn get_bone_actions(&self, bone: &str) -> Option<Vec<String>> {
        with_bone_db(|db| db.actions(bone).map(|actions| actions.to_vec()))
    }

    #[wasm_bindgen]
    pub fn get_bone_directions(&self, bone: &str, action: &str) -> Option<Vec<String>> {
        with_bone_db(|db| {
            db.directions(bone, action)
                .map(|directions| directions.to_vec())
        })
    }

    #[wasm_bindgen]
    pub fn get_bone_degree_limit(&self, bone: &str, action: &str, direction: &str) -> Option<f32> {
        with_bone_db(|db| db.get_rule(bone, action, direction).map(|rule| rule.limit))
    }

    #[wasm_bindgen]
    pub fn get_bone_japanese_name(&self, bone: &str) -> Option<String> {
        with_bone_db(|db| db.japanese_name(bone).map(|name| name.to_string()))
    }

    #[wasm_bindgen]
    pub fn get_bone_english_name(&self, bone: &str) -> Option<String> {
        with_bone_db(|db| db.english_name(bone).map(|name| name.to_string()))
    }
}
