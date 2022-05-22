use clap::{ArgEnum, Parser};

/// Arguments
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Options {
    /// Custom config file
    #[clap(short, long)]
    pub config: Option<String>,

    /// Query
    #[clap(arg_enum, short, long)]
    pub query: Option<Query>,

    /// Force fresh session cache
    #[clap(short, long)]
    pub fresh: bool,
}

#[derive(ArgEnum, Clone, Debug)]
pub enum Query {
    #[clap(name = "electric")]
    Electric,
}
