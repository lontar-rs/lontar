//! Lontar CLI — command-line interface for document generation.

use clap::Parser;

#[derive(Parser)]
#[command(name = "lontar")]
#[command(about = "Comprehensive document generation for Rust", long_about = None)]
struct Cli {
    /// Input file
    input: String,
    
    /// Output format
    #[arg(short, long)]
    format: String,
}

fn main() {
    let _cli = Cli::parse();
    println!("Lontar CLI placeholder");
}
