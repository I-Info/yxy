use clap::Parser;
use std::error::Error;

/// Arguments
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Custom config file
    #[clap(short, long)]
    config: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    println!("{:?}", args);

    Ok(())
}
