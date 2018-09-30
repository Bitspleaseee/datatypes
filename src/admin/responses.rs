//! The responses a admin will get from requests to the service

use crate::payloads::TokenPayload;
use crate::Token;

pub type IntAdminSuccess = TokenPayload<AdminSuccess, Token>;
pub type IntAdminError = TokenPayload<AdminError, Token>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum AdminSuccess {
    IpBanned,
    IpUnbanned,
}

#[derive(Fail, Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum AdminError {
    #[fail(display = "client (cookies) does not have token")]
    MissingTokenClient,
    #[fail(display = "server does not have the requested token registered")]
    UnassociatedToken,
    #[fail(display = "internal occured error")]
    InternalError,
}
