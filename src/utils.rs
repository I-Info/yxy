use base64;
use crypto::{digest::Digest, md5};
use rsa::{pkcs8::DecodePublicKey, PaddingScheme, PublicKey, RsaPublicKey};
use std::io::Write;

use crate::error::Error;

/// Parse unformatted pure Base64 public key to PEM format
///
pub fn parse_public_key_pem(raw: &str) -> String {
    let bytes = raw.as_bytes();
    let mut result = String::from("-----BEGIN PUBLIC KEY-----\n");
    for i in 0..bytes.len() {
        result.push(bytes[i] as char);
        if (i + 1) % 64 == 0 {
            result.push('\n')
        }
    }
    result.push_str("\n-----END PUBLIC KEY-----");

    result
}

/// Return MD5 Hex string
pub fn md5<T: Into<String>>(input: T) -> String {
    let mut md5 = md5::Md5::new();
    md5.input_str(&input.into());

    md5.result_str()
}

/// Encrypt password by `PKCS1v15(MD5(<password>))`
///
/// Return Base64 String
pub fn encrypt_password(pass: &str, raw_pub: &str) -> Result<String, Error> {
    let pem = parse_public_key_pem(raw_pub);
    let public_key = RsaPublicKey::from_public_key_pem(&pem)?;

    let mut rng = rand::thread_rng(); // random generator

    let pass_md5 = md5(pass);

    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let encrypted = public_key.encrypt(&mut rng, padding, pass_md5.as_bytes())?;

    let result = base64::encode(encrypted);
    Ok(result)
}

/// Write string to file
///
/// Auto create file
pub fn file_write(path: &str, s: &str) -> Result<(), Error> {
    let mut f = std::fs::File::create(path)?;
    f.write(s.as_bytes())?;
    Ok(())
}

/// PKCS#7 Padding method
pub fn pkcs7_padding(message: &str, block_size: usize) -> String {
    let padding_size = block_size - message.len() % block_size;
    let padding_char = padding_size as u8 as char;
    let padding: String = (0..padding_size).map(|_| padding_char).collect();
    format!("{}{}", message, padding)
}
