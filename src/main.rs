// src/main.rs
/*
 * Main executable for ArtificialMintMasterProtocolNext
 */

use clap::Parser;
use artificialmintmasterprotocolnext::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialMintMasterProtocolNext - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
