use clap::Parser;

/// Arguments
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// Custom config file
    #[clap(short, long)]
    pub config: Option<String>,
}
