use std::fs::remove_file;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Result, bail, ensure};
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    c_filename: String,
    #[arg(short, long, help = "Run the lexer only.")]
    lex: bool,
    #[arg(short, long, help = "Run the lexer and parser.")]
    parse: bool,
    #[arg(
        short,
        long,
        help = "Run the lexer, parser, and assembly AST generator."
    )]
    codegen: bool,
    #[arg(
        short = 'S',
        long,
        help = "Run all compiler steps besides assembler, generating a file at <C_FILENAME>.s."
    )]
    emit_assembly: bool,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum CompilerMode {
    Lex,
    Parse,
    CodeGen,
    CodeEmit,
    Full,
}

fn compile(_preprocessed_path: &str, _assembly_path: &str, _mode: CompilerMode) -> Result<()> {
    Ok(())
}

/// Helper function to create path strings given the input C file path to the compiler and an extension
fn extract_path_str_from_pathbuf(mut path_buf: PathBuf, extension: &str) -> Result<String> {
    if !path_buf.add_extension(extension) {
        bail!(format!(
            "Unable to create filename from path: {:?}, {}",
            path_buf, extension
        ))
    }
    let Some(pb_path_str) = path_buf.to_str() else {
        bail!(format!("Unable to extract path string: {:?}", path_buf))
    };
    Ok(pb_path_str.to_string())
}

/// Helper function to run external commands that the compiler driver needs (i.e. preprocessor, linker)
fn run_command(cmd: &mut Command, cmd_str: &str) -> Result<()> {
    let cmd_out = cmd.output();
    match cmd_out {
        Ok(out) => {
            ensure!(
                out.status.success(),
                format!(
                    "Error running {}: {}",
                    cmd_str,
                    String::from_utf8_lossy(&out.stderr)
                )
            );
            return Ok(());
        }
        Err(err) => bail!(format!("Error running {}: {}", cmd_str, err)),
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let c_filename = &cli.c_filename;

    ensure!(
        c_filename.ends_with(".c"),
        format!("Invalid C filename passed to compiler: {}", c_filename)
    );

    // Boilerplate to determine paths for executable, preprocessed and assembly files
    let c_path = Path::new(c_filename);
    let Some(c_parent) = c_path.parent() else {
        bail!(format!(
            "Unable to determine parent of C filename: {}",
            c_filename
        ))
    };
    let Some(c_base) = c_path.file_stem() else {
        bail!(format!(
            "Unable to determine base of C filename: {}",
            c_filename
        ))
    };
    let c_executable_path = c_parent.join(c_base);
    let preprocessed_path = extract_path_str_from_pathbuf(c_executable_path.clone(), "i")?;
    let executable_path = extract_path_str_from_pathbuf(c_executable_path.clone(), "")?;
    let assembly_path = extract_path_str_from_pathbuf(c_executable_path.clone(), "s")?;

    // Run the preprocessor
    let mut preprocess_out = Command::new("gcc");
    preprocess_out.args(["-E", "-P", c_filename, "-o", &preprocessed_path]);
    run_command(&mut preprocess_out, "preprocessor")?;

    let compiler_mode = if cli.emit_assembly {
        CompilerMode::CodeEmit
    } else if cli.codegen {
        CompilerMode::CodeGen
    } else if cli.parse {
        CompilerMode::Parse
    } else if cli.lex {
        CompilerMode::Lex
    } else {
        CompilerMode::Full
    };

    compile(&preprocessed_path, &assembly_path, compiler_mode)?;
    remove_file(preprocessed_path)?;

    if let CompilerMode::Full = compiler_mode {
        let mut link_out = Command::new("gcc");
        link_out.args([&assembly_path, "-o", &executable_path]);
        run_command(&mut link_out, "linker")?;
        remove_file(assembly_path)?;
    }
    Ok(())
}
