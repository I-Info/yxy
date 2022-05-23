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
    pub const QUERY_ELECTRIC: &'static str =
        concatcp!(BASE_URL, "/app/electric/queryISIMSRoomSurplus");
}
