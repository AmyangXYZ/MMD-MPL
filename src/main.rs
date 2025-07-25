use mmd_mpl::{MPLCompiler, MPLPose};

fn main() {
    let compiler = MPLCompiler::new();
    let bone_states = compiler.compile(
        "@pose bend {
               waist turn right 20;

       waist bend forward 20;
       head turn right 20;

}
",
    );
    if let Ok(bone_states) = bone_states {
        println!("{:?}", bone_states[0]);
        println!(
            "{:}",
            MPLPose::from_bone_states("a", bone_states.clone()).to_string()
        );
    }
}
