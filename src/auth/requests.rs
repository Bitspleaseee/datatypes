//! The requests a user can send to the auth-service

use crate::payloads::EmptyPayload;
use crate::payloads::TokenPayload;
use crate::valid::fields::*;
use crate::Token;

pub type TokenAuthRequest = TokenPayload<AuthRequest, Token>;

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
    #[get = "pub"]
    username: Username,
    #[get = "pub"]
    password: PlainPassword,
}

#[derive(Getters, Serialize, Deserialize)]
pub struct RegisterUserPayload {
    #[get = "pub"]
    username: Username,
    #[get = "pub"]
    password: PlainPassword,
    #[get = "pub"]
    email: Email,
}
