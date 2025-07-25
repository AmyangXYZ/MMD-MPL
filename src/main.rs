use mmd_mpl::{MPLCompiler, MPLPoseStatement};

fn main() {
    let compiler = MPLCompiler::new();
    let bone_states = compiler.compile(
        "@pose bend {
       waist bend forward 40;
       waist turn right 20;

}
",
    );
    if let Ok(bone_states) = bone_states {
        println!("{:?}", bone_states[0]);
        println!(
            "{:?}",
            MPLPoseStatement::from_quaternion(
                &bone_states[0].bone_name_en(),
                bone_states[0].quaternion()
            )
        );
    }
}
