use std::fs::remove_file;
use std::path::Path;
use std::process::Command;

use anyhow::{Result, bail, ensure};
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    c_filename: String,
    #[arg(short, long)]
    lex: bool,
    #[arg(short, long)]
    parse: bool,
    #[arg(short, long)]
    codegen: bool,
    #[arg(short = 'S', long)]
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
    let mut preprocessed_path = c_executable_path.clone();
    if !preprocessed_path.add_extension("i") {
        bail!(format!(
            "Unable to create preprocessed filename: {:?}",
            preprocessed_path
        ))
    }
    let mut assembly_path = c_executable_path.clone();
    if !assembly_path.add_extension("s") {
        bail!(format!(
            "Unable to create assembly filename: {:?}",
            assembly_path
        ))
    }
    let Some(preprocessed_path_str) = preprocessed_path.to_str() else {
        bail!(format!("Unable to extract preprocessed_path string: {:?}", preprocessed_path))
    };
    let Some(assembly_path_str) = assembly_path.to_str() else {
        bail!(format!(
            "Unable to extract assembly_path string: {:?}",
            assembly_path
        ))
    };
    let Some(executable_path_str) = c_executable_path.to_str() else {
        bail!(format!(
            "Unable to extract executable_path string: {:?}",
            c_executable_path
        ))       
    };

    // Run the preprocessor
    let preprocess_out = Command::new("gcc")
        .args(["-E", "-P", c_filename, "-o", preprocessed_path_str])
        .output();
    match preprocess_out {
        Ok(out) => ensure!(
            out.status.success(),
            format!(
                "Error running preprocessor: {}",
                String::from_utf8_lossy(&out.stderr)
            )
        ),
        Err(err) => bail!(format!("Error running preprocessor: {}", err)),
    }

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

    compile(preprocessed_path_str, assembly_path_str, compiler_mode)?;
    remove_file(preprocessed_path_str)?;

    if let CompilerMode::Full = compiler_mode {
        let link_out = Command::new("gcc")
            .args([assembly_path_str, "-o", executable_path_str])
            .output();
        match link_out {
            Ok(out) => ensure!(
                out.status.success(),
                format!(
                    "Error running linker: {}",
                    String::from_utf8_lossy(&out.stderr)
                )
            ),
            Err(err) => bail!(format!("Error running preprocessor: {}", err)),
        }
        remove_file(assembly_path_str)?;
    }
    Ok(())
}
