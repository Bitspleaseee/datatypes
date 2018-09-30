//! The requests a user can send to the auth-service

use crate::payloads::EmptyPayload;
use crate::payloads::TokenPayload;
use crate::valid::fields::*;
use crate::Token;

pub type IntAuthRequest<'a> = TokenPayload<AuthRequest<'a>, Token>;

#[derive(Serialize, Deserialize)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum AuthRequest<'a> {
    Authenticate(#[serde(borrow)] AuthRequestPayload<'a>),
    Deauthenticate(EmptyPayload),
    RegisterUser(#[serde(borrow)] RegisterUserPayload<'a>),
}

#[derive(Getters, Serialize, Deserialize)]
pub struct AuthRequestPayload<'a> {
    #[serde(borrow)] #[get]
    username: Username<'a>,
    #[serde(borrow)] #[get]
    password: PlainPassword<'a>,
}

#[derive(Getters, Serialize, Deserialize)]
pub struct RegisterUserPayload<'a> {
    #[serde(borrow)] #[get]
    username: Username<'a>,
    #[serde(borrow)] #[get]
    password: PlainPassword<'a>,
    #[serde(borrow)] #[get]
    email: Email<'a>,
}
