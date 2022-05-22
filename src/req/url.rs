//! Define public API urls

pub mod auth {
    use const_format::concatcp;

    pub const BASE_URL: &'static str = "https://auth.xiaofubao.com";
    pub const OAUTH_URL: &'static str = concatcp!(BASE_URL, "/authoriz/getCodeV2");
}

pub mod application {
    pub const BASE_URL: &'static str = "https://application.xiaofubao.com";
}
