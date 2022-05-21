use std::error::Error;

pub mod conf;
pub mod flags;
pub mod req;
pub mod utils;

/// Entrance of the application
pub fn run() -> Result<(), Box<dyn Error>> {
    Ok(())
}
