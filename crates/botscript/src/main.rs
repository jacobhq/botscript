use anyhow::{Context, Result};
use botscript::compile_file;
use clap::Parser;
use std::path::PathBuf;

/// Compile botscript (.bs files) into Java OpModes for FTC robots.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// .bs file you want to compile
    #[arg(short, long)]
    input: PathBuf,

    /// .java output file for your robot
    #[arg(short, long, default_value = "default.java")]
    output: PathBuf,
}

fn default_output(input: &PathBuf) -> PathBuf {
    let mut output = input.clone();
    if let Some(stem) = input.file_stem() {
        output.set_file_name(format!("{}.java", stem.to_string_lossy()));
    }
    output
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let output = if args.output == PathBuf::from("default.java") {
        default_output(&args.input)
    } else {
        args.output
    };

    let content = std::fs::read_to_string(&args.input)
        .with_context(|| format!("could not read file `{}`", args.input.display()))?;

    let java_result = compile_file(content).join("\n");

    std::fs::write(&output, java_result)
        .with_context(|| format!("could not write to file `{}`", output.display()))?;

    println!("Successfully compiled {:?} to {:?}", args.input, output);

    Ok(())
}
