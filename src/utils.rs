use crypto::{digest::Digest, md5};

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

pub fn md5<T: Into<String>>(input: T) -> [u8; 16] {
    let mut md5 = md5::Md5::new();
    md5.input_str(&input.into());
    let mut result: [u8; 16] = [0; 16];
    md5.result_str();
    md5.result(&mut result);

    result
}
