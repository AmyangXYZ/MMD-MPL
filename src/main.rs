use mmd_mpl::{MPLCompiler, VMDWriter};

fn main() {
    let compiler = MPLCompiler::new();
    let key_frames = compiler.compile(
        "
 @pose kick_left {
    leg_l bend forward 30;
    knee_l bend backward 0;
    leg_r bend backward 20;
    knee_r bend backward 15;
}

@pose kick_right {
    leg_r bend forward 30;
    knee_r bend backward 0;
    leg_l bend backward 20;
    knee_l bend backward 15;
}

@animation walk {
    0: kick_left;
    0.3: kick_right;
    0.6: kick_left;
    0.9: kick_right;
}

    
main {
    walk;
}
",
    );
    if let Ok(key_frames) = key_frames {
        let writer = VMDWriter::new(key_frames);
        let vmd_data = writer.write().unwrap();

        std::fs::write("vmd/output.vmd", &vmd_data).unwrap();
        println!("VMD saved to output.vmd");
    }

    // Read VMD data from file
    println!("=== VMD Reading Test ===");
    let vmd_data = std::fs::read("vmd/Stand.vmd").unwrap();

    let vmd_script = compiler.from_vmd(&vmd_data).unwrap();
    println!("{}", vmd_script);

    // // Test VPD reading
    // println!("=== VPD Reading Test ===");
    // let vpd_data = std::fs::read("vpd/1.vpd").unwrap();
    // let vpd_script = compiler.from_vpd(&vpd_data).unwrap();
    // println!("{}", vpd_script);
}
