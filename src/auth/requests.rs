//! The requests a user can send to the auth-service

use crate::payloads::EmptyPayload;
use crate::valid::ids::*;
use crate::valid::fields::*;
use crate::auth::responses::Role;

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
    SetUserRole(SetUserRolePayload),
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


#[derive(Serialize, Deserialize)]
pub struct SetUserRolePayload {
    pub id: UserId,
    pub role: Role,
}
