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
    #[fail(display = "client (cookies) does not have token")]
    MissingTokenClient,
    #[fail(display = "server does not have provided token")]
    MissingTokenServer,
    #[fail(display = "invalid username")]
    InvalidUsername,
    #[fail(display = "invalid password")]
    InvalidPassword,
}
