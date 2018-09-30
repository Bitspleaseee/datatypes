//! The requests a admin can send to the service

use crate::payloads::TokenPayload;
use crate::valid::fields::*;
use crate::Token;
use std::net::IpAddr;

pub type IntAdminRequest = TokenPayload<AdminRequests, Token>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum AdminRequests {
    BanIp(BanUserPayload),
    UnbanIp(BanUserPayload),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BanUserPayload {
    ip: IpAddr,
}
