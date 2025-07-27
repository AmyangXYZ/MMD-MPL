use mmd_mpl::{MPLCompiler, MPLPose, VMDWriter};

fn main() {
    let compiler = MPLCompiler::new();
    let key_frames = compiler.compile(
        "
        @pose default {
               head turn right 0;
               head sway right 0;
               waist bend forward 0;
}

        @pose look_left {
               base move forward 2;
               base move left 2;
               base move up 2;
               head turn left 20;
               head sway left 20;
}

       @pose look_right {
               head turn right 20;
               head sway right 20;
}

       @pose bend_over {
                      base turn right 90;

               base move forward 10;
}
                      @pose stand {
               waist bend forward 0;
}
               

       @animation look_around {
      
  0.8: look_right;
  0.9: look_left;
  1.2: look_right;
}

@animation bow {
    1.4: stand & look_left;
  2.0: bend_over;
  2.3: stand;
    }

    @animation default1 {
        0.5: bend_over;
        1.0: look_left;
    }

  main {
        default1;
    
  }
",
    );
    if let Ok(key_frames) = key_frames {
        for key_frame in key_frames.iter() {
            println!("{:?}", key_frame);
        }
        println!(
            "{:?}",
            MPLPose::from_bone_frames("default", key_frames[0].bone_frames.clone()).to_string()
        );
        let vmd = VMDWriter::new(key_frames);
        let vmd_data = vmd.create_vmd().unwrap();

        std::fs::write("output.vmd", vmd_data).unwrap();
        println!("VMD saved to output.vmd");
    }
}
