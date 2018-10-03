//! The responses a admin will get from requests to the service

use crate::payloads::TokenPayload;
use crate::Token;

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
    #[fail(display = "internal occured error")]
    InternalError,
}
