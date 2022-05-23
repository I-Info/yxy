//! Application APIs

use std::error::Error;

use super::Handler;

impl Handler {
    pub fn query_bind(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn query_electric(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
