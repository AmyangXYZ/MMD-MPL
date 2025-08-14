use mmd_mpl::{MPLCompiler, MPLPose, VMDReader, VMDWriter};

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

        // Read VMD data from file
        println!("Reading VMD data from file...");
        let file_vmd_data = std::fs::read("vmd/Stand.vmd").unwrap();
        println!("Read {} bytes from file", file_vmd_data.len());

        // Test VMD reading from file
        let reader = VMDReader::new();
        let read_key_frames = reader.read(&file_vmd_data).unwrap();
        println!("Read {} keyframes from VMD file", read_key_frames.len());

        // Demonstrate VMD to MPL animation conversion
        println!("\n=== VMD to MPL Animation Conversion ===");
        let mut poses = Vec::new();
        let mut animation_statements = Vec::new();

        for (i, keyframe) in read_key_frames.iter().enumerate() {
            let pose_name = format!("pose_{}", i);
            let pose = MPLPose::from_bone_frames(&pose_name, keyframe.bone_frames.clone());
            poses.push(pose);

            // Create animation statement
            animation_statements.push(format!("    {:.2}: {};", keyframe.time, pose_name));
        }

        // Generate MPL script
        println!("Generated MPL script:");

        for pose in poses {
            println!("{}", pose.to_block());
        }

        println!("@animation extracted_animation {{");
        for statement in animation_statements {
            println!("{}", statement);
        }
        println!("}}\n");

        println!("main {{");
        println!("    extracted_animation;");
        println!("}}");
    }
}
