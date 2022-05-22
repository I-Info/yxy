use clap::{ArgEnum, Parser};

/// Arguments
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// Custom config file
    #[clap(short, long)]
    pub config: Option<String>,

    /// Query
    #[clap(arg_enum, short, long)]
    pub query: Option<Query>,
}

#[derive(ArgEnum, Clone, Debug)]
pub enum Query {
    #[clap(name = "electric")]
    Electric,
}
