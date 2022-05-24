//! Define public API urls

pub mod auth {
    use const_format::concatcp;

    pub const BASE_URL: &'static str = "https://auth.xiaofubao.com";
    pub const OAUTH_URL: &'static str = concatcp!(BASE_URL, "/authoriz/getCodeV2");
}

pub mod application {
    use const_format::concatcp;

    pub const BASE_URL: &'static str = "https://application.xiaofubao.com";

    pub const GET_USER_FOR_AUTHORIZE: &'static str =
        concatcp!(BASE_URL, "/app/login/getUser4Authorize");

    pub const QUERY_BIND: &'static str = concatcp!(BASE_URL, "/app/electric/queryBind");

    pub const QUERY_ELECTRICITY: &'static str =
        concatcp!(BASE_URL, "/app/electric/queryISIMSRoomSurplus");
}

pub mod app {
    use const_format::concatcp;

    pub const BASE_URL: &'static str = "https://compus.xiaofubao.com";

    pub const GET_SECURITY_TOKEN: &'static str = concatcp!(BASE_URL, "/common/security/token");

    pub const SEND_VERIFICATION_CODE: &'static str =
        concatcp!(BASE_URL, "/compus/user/sendLoginVerificationCode");

    pub const GET_IMAGE_CAPTCHA: &'static str =
        concatcp!(BASE_URL, "/common/security/imageCaptcha");

    pub const DO_LOGIN_BY_CODE: &'static str =
        concatcp!(BASE_URL, "/login/doLoginByVerificationCode");
}
