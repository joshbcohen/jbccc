use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    c_filepath: String,
    #[arg(short, long)]
    lex: bool,
    #[arg(short, long)]
    parse: bool,
    #[arg(short, long)]
    codegen: bool,
    #[arg(short='S', long)]
    emit_assembly: bool,
}

fn main() {
    let cli = Cli::parse();

    println!("path: {:?}", cli.c_filepath);
    println!("lex: {:?}", cli.lex);
    println!("emit: {:?}", cli.emit_assembly);
}
