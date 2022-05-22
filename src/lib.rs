use std::error::Error;

pub mod conf;
pub(crate) mod error;
pub mod flags;
pub mod req;
pub mod utils;

/// Entrance of the main application
pub fn run(conf: conf::Config) -> Result<(), Box<dyn Error>> {
    let client = req::init_default_client()?; // Global default client

    println!("Trying to get oauth code...");
    let oauth_code = req::auth::get_oauth_code(&client, &conf.info.id)?;
    println!("OAuth Code: {}", oauth_code);

    println!("Trying to login...");
    let auth_result = req::auth::authorize(&client, &oauth_code)?;
    println!("Authorized: {}", auth_result.session);

    Ok(())
}
