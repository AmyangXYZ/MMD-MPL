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
}
