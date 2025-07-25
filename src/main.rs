use mmd_mpl::{MPLCompiler, VMDWriter};

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
               head turn left 20;
               head sway left 20;
}

       @pose look_right {
               head turn right 20;
               head sway right 20;
}

       @pose bend_over {
               waist bend forward 90;
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
        let vmd = VMDWriter::new(key_frames);
        let vmd_data = vmd.create_vmd().unwrap();

        std::fs::write("output.vmd", vmd_data).unwrap();
        println!("VMD saved to output.vmd");
    }
}
