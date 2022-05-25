use clap::{ArgEnum, Parser, Subcommand};

/// Arguments
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Options {
    /// Custom config file
    #[clap(short, long)]
    pub config: Option<String>,

    /// Query
    #[clap(subcommand)]
    pub command: Option<Commands>,

    /// Force fresh session cache
    #[clap(short, long)]
    pub fresh: bool,

    /// Verbose
    #[clap(short, long)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// API Queries
    Query {
        /// Selection
        #[clap(arg_enum)]
        query: Query,
        /// Argument
        arg: String,
    },
}

#[derive(ArgEnum, Clone, Debug)]
pub enum Query {
    #[clap(name = "electric")]
    Electric,

    #[clap(name = "uid")]
    Uid,
}
