//! The requests a admin can send to the service

use crate::payloads::TokenPayload;
use crate::Token;
use std::net::IpAddr;

pub type TokenAdminRequest = TokenPayload<AdminRequest, Token>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum AdminRequest {
    BanIp(IpAddrPayload),
    UnbanIp(IpAddrPayload),
}

#[derive(Getters, Serialize, Deserialize, Debug)]
pub struct IpAddrPayload {
    #[get = "pub"]
    ip: IpAddr,
}
