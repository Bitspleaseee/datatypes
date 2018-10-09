//! The requests a admin can send to the service

use crate::auth::requests::SetUserRolePayload;
use std::net::IpAddr;

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum AdminRequest {
    BanIp(IpAddrPayload),
    UnbanIp(IpAddrPayload),
    SetUserRole(SetUserRolePayload),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IpAddrPayload {
    pub ip: IpAddr,
}
