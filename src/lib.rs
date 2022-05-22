use std::error::Error;

pub mod conf;
pub(crate) mod error;
pub mod flags;
pub mod req;
pub mod utils;

/// Entrance of the application
pub fn run(conf: conf::Config) -> Result<(), Box<dyn Error>> {
    let client = req::init_default_client()?; // Global default client
    println!("Trying to get oauth code...");
    let result = req::auth::get_oauth_code(&client, &conf.info.id)?;
    println!("OAuth Code: {}", result);

    Ok(())
}
