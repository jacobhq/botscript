use anyhow::{Context, Result};
use botscript::compile_file;
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

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

fn default_output(input: &Path) -> PathBuf {
    let mut output = input.to_path_buf();
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

    let content = fs::read_to_string(&args.input)
        .with_context(|| format!("could not read file `{}`", args.input.display()))?;

    let java_lines = compile_file(content)
        .with_context(|| format!("failed to compile the file `{}`", args.input.display()))?
        .join("\n");

    let template_path = PathBuf::from("src")
        .join("..")
        .join("templates")
        .join("BasicOpModeLinear.java");
    let template_content = fs::read_to_string(&template_path)
        .with_context(|| format!("could not read template file `{}`", template_path.display()))?;

    let mut template_lines: Vec<String> = template_content.lines().map(String::from).collect();

    let insert_line = 51.min(template_lines.len());
    template_lines.splice(insert_line..insert_line, vec![java_lines]);

    let final_output = template_lines.join("\n");

    fs::write(&output, final_output)
        .with_context(|| format!("could not write to file `{}`", output.display()))?;

    println!("Successfully compiled {:?} to {:?}", args.input, output);

    Ok(())
}
