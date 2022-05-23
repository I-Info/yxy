use std::error::Error;

use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let opts = yxy::arg::Options::parse();
    // println!("{:?}", args);

    let conf_path = match &opts.config {
        Some(c) => c,
        None => "./conf.yaml",
    };

    let conf = yxy::conf::Config::parse(&conf_path)?;
    // println!("{:?}", conf);

    yxy::run(conf, opts)?;

    Ok(())
}
