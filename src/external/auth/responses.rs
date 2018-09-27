//! The responses a user will get from requests to the auth-service

// TODO decide if the responses should have a payload

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum AuthSuccess {
    Authenticated,
    Deauthenticated,
    UserRegistered,
}

#[derive(Fail, Serialize, Deserialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum AuthError {
    #[fail(display = "token missing from cookies")]
    RequestMissingToken,
    #[fail(display = "server did not have the provided token")]
    ServerMissingToken,
    #[fail(display = "invalid username")]
    InvalidUsername,
    #[fail(display = "invalid password")]
    InvalidPassword,
}
