use clap::{Parser, Subcommand};
use mmd_mpl::MPLCompiler;
use std::path::Path;

#[derive(Parser)]
#[command(name = "mpl")]
#[command(about = "MPL - Rule-based Domain-Specific Language for MMD poses and animations")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile MPL script to VMD file
    #[command(short_flag = 'c')]
    Compile {
        /// Input MPL file
        input: String,
        /// Output VMD file (optional, auto-detected from input)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Reverse compile VMD/VPD file to MPL script
    #[command(short_flag = 'r')]
    ReverseCompile {
        /// Input VMD or VPD file
        input: String,
        /// Output MPL file (optional, auto-detected from input)
        #[arg(short, long)]
        output: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    let compiler = MPLCompiler::new();

    match cli.command {
        Commands::Compile { input, output } => {
            compile(&compiler, &input, output);
        }
        Commands::ReverseCompile { input, output } => {
            reverse_compile(&compiler, &input, output);
        }
    }
}

fn compile(compiler: &MPLCompiler, input: &str, output: Option<String>) {
    // Read MPL script from file
    let mpl_script = match std::fs::read_to_string(input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading input file '{}': {}", input, e);
            std::process::exit(1);
        }
    };

    // Compile MPL to key frames
    let vmd_bytes = match compiler.compile(&mpl_script) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Compilation error: {}", e);
            std::process::exit(1);
        }
    };

    // Determine output filename
    let output_path = output.unwrap_or_else(|| {
        let input_path = Path::new(input);
        let stem = input_path.file_stem().unwrap_or_default();
        format!("{}.vmd", stem.to_string_lossy())
    });

    // Write output file
    if let Err(e) = std::fs::write(&output_path, &vmd_bytes) {
        eprintln!("Error writing output file '{}': {}", output_path, e);
        std::process::exit(1);
    }

    println!("Successfully compiled '{}' to '{}'", input, output_path);
}

fn reverse_compile(compiler: &MPLCompiler, input: &str, output: Option<String>) {
    // Read input file
    let input_data = match std::fs::read(input) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading input file '{}': {}", input, e);
            std::process::exit(1);
        }
    };

    // Determine file type and reverse compile using WASM API pattern
    let input_path = Path::new(input);
    let extension = input_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    let mpl_script = match extension.as_str() {
        "vmd" => match compiler.from_vmd(&input_data) {
            Ok(script) => script,
            Err(e) => {
                eprintln!("Error reverse compiling VMD: {}", e);
                std::process::exit(1);
            }
        },
        "vpd" => match compiler.from_vpd(&input_data) {
            Ok(script) => script,
            Err(e) => {
                eprintln!("Error reverse compiling VPD: {}", e);
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!(
                "Unknown file extension '{}'. Please use .vmd or .vpd files.",
                extension
            );
            std::process::exit(1);
        }
    };

    // Determine output filename
    let output_path = output.unwrap_or_else(|| {
        let input_path = Path::new(input);
        let stem = input_path.file_stem().unwrap_or_default();
        format!("{}.mpl", stem.to_string_lossy())
    });

    // Write output file
    if let Err(e) = std::fs::write(&output_path, mpl_script) {
        eprintln!("Error writing output file '{}': {}", output_path, e);
        std::process::exit(1);
    }

    println!("Successfully reversed '{}' to '{}'", input, output_path);
}
