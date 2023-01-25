use clap::Parser;
use std::process;
mod parse;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[arg(short, long)]
    input_file: String,
        
    #[arg(short, long)]
    output_file: String,
}

fn main() {
    let arguments = Arguments::parse();
    if let Err(error) = parse::run(&arguments.input_file, &arguments.output_file) {
        eprintln!("{}", error);
        process::exit(1);
    }
}