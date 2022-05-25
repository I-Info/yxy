use std::error::Error;

use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let opts = yxy::arg::Options::parse();

    let conf_path = match &opts.config {
        Some(c) => c,
        None => "./conf.yaml",
    };

    let conf = yxy::conf::Config::parse(&conf_path)?;

    if let Some(v) = opts.command {
        match v {
            yxy::arg::Commands::Query { query: q, arg: a } => match q {
                yxy::arg::Query::Uid => {
                    yxy::query_uid(&a)?;
                }
                _ => {
                    todo!()
                }
            },
        }
    } else {
        // Default query electricity
        let result = yxy::query_ele(conf, opts)?;
        println!("Electricity balance: {}", result.soc);
    }

    Ok(())
}
