use base64;
use rsa::{pkcs8::DecodePublicKey, PaddingScheme, PublicKey, RsaPublicKey};
use std::error::Error;

pub mod conf;
pub mod utils;

/// Encrypt password by `PKCS1v15(MD5(<password>))`
///
/// Return Base64 String
pub fn encrypt_password(pass: &str, raw_pub: &str) -> Result<String, Box<dyn Error>> {
    let pem = utils::parse_public_key_pem(raw_pub);
    let public_key = RsaPublicKey::from_public_key_pem(&pem)?;

    let mut rng = rand::thread_rng(); // random generator

    let pass_md5 = utils::md5(pass);

    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let encrypted = public_key.encrypt(&mut rng, padding, pass_md5.as_bytes())?;

    let result = base64::encode(encrypted);
    Ok(result)
}
