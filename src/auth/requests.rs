//! The requests a user can send to the auth-service

use crate::payloads::EmptyPayload;
use crate::payloads::TokenPayload;
use crate::valid::fields::*;
use crate::valid::token::Token;

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

#[derive(Serialize, Deserialize)]
pub struct AuthPayload {
    pub username: Username,
    pub password: PlainPassword,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterUserPayload {
    pub username: Username,
    pub password: PlainPassword,
    pub email: Email,
}
