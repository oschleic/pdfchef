#![allow(unused)]
use clap::Parser;

#[derive(Parser)]
struct Cli{
    /// The pdf output file (replaces the input file by default)
    #[clap(parse(from_os_str))]
    #[clap(short, long)]
    output: Option<std::path::PathBuf>,
    /// The input pdf file
    #[clap(parse(from_os_str))]
    file: std::path::PathBuf,

}

fn main() {
    let args = Cli::parse();
    println!("Input file {}", args.file.display());
    println!("Output file {}", args.output.unwrap().display());
}
