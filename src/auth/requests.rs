//! The requests a user can send to the auth-service

use crate::payloads::EmptyPayload;
use crate::payloads::TokenPayload;
use crate::valid::fields::*;
use crate::Token;

pub type IntAuthRequest = TokenPayload<AuthRequest, Token>;

#[derive(Serialize, Deserialize)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum AuthRequest {
    Authenticate(AuthPayload),
    Deauthenticate(EmptyPayload),
    RegisterUser(RegisterUserPayload),
}

#[derive(Getters, Serialize, Deserialize)]
pub struct AuthPayload {
    #[get]
    username: Username,

    #[get]
    password: PlainPassword,
}

#[derive(Getters, Serialize, Deserialize)]
pub struct RegisterUserPayload {
    #[get]
    username: Username,

    #[get]
    password: PlainPassword,

    #[get]
    email: Email,
}
