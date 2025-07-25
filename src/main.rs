use mmd_mpl::MPLCompiler;

fn main() {
    let compiler = MPLCompiler::new();
    let bone_states = compiler.compile(
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
    println!("{:?}", bone_states);
}
