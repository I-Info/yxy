use std::error::Error;

use super::url;

pub fn get_oauth_code(client: &reqwest::blocking::Client) -> Result<String, Box<dyn Error>> {
    let response = client.get(url::auth::OAUTH_URL).send()?;
    println!("{:?}", response);
    Ok(String::new())
}
