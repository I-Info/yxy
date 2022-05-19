use std::{env, error::Error, process};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("invalid count of arguments");
        process::exit(1);
    }
    println!("{}", yxy::encrypt_password(&args[1], &args[2])?);

    Ok(())
}
