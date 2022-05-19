use base64;
use rsa::{pkcs8::DecodePublicKey, PaddingScheme, PublicKey, RsaPublicKey};
use std::error::Error;

pub mod utils;

/// Encrypt password by PKCS1v15
///
/// Return Base64 String
pub fn encrypt_password(pass: &str, raw_pub: &str) -> Result<String, Box<dyn Error>> {
    let pem = utils::parse_public_key_pem(raw_pub);
    println!("{}", pem);
    let public_key = RsaPublicKey::from_public_key_pem(&pem)?;

    let mut rng = rand::thread_rng();

    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let encrypted = public_key.encrypt(&mut rng, padding, &utils::md5(pass))?;

    let result = base64::encode(encrypted);
    Ok(result)
}
