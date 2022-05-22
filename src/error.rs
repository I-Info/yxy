//! Application Error

#[derive(Debug)]
pub struct Error {
    pub code: u32,
    pub msg: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error {{ code: {}, msg: {}}}", self.code, self.msg)
    }
}

impl std::error::Error for Error {}
