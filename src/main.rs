use mmd_mpl::MPLCompiler;

fn main() {
    let compiler = MPLCompiler::new();
    let script = compiler.compile(
        "@pose bend {
    head bend forward 30;
    head turn right 15;
}

@pose look_right {
    head turn right 30;
}

@animation aa {
    bend 0.5s;
    look_right 1s;
}

@animation bb {
        look_right 1s;
    bend 0.5s;
}


main {
    aa;
    bb;
}",
    );
    let script = script.unwrap();
    println!("{:?}", script);
    println!("{:?}", script.poses["bend"].to_bone_states());
}
