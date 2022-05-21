use std::error::Error;

use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let args = yxy::flags::Args::parse();
    println!("{:?}", args);

    let conf_path = match args.config {
        Some(c) => c,
        None => String::from("./conf.yaml"),
    };

    let conf = yxy::conf::Config::parse(conf_path)?;

    println!("{:?}", conf);

    Ok(())
}
