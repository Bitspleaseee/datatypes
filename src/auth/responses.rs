//! The responses a user will get from requests to the auth-service

use crate::payloads::TokenPayload;
use crate::Token;

pub type IntAuthSuccess = TokenPayload<AuthSuccess, Token>;
pub type IntAuthError = TokenPayload<AuthError, Token>;

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

#[derive(Fail, Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum AuthError {
    #[fail(display = "invalid username")]
    InvalidUsername,
    #[fail(display = "invalid password")]
    InvalidPassword,
    #[fail(display = "user already exists")]
    CannotRegisterExistingUser,
}
