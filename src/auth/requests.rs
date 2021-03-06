//! The requests a user can send to the auth-service

use crate::auth::responses::Role;
use crate::payloads::EmptyPayload;
use crate::valid::fields::*;
use crate::valid::ids::*;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct SetUserRolePayload {
    pub id: UserId,
    pub role: Role,
}
