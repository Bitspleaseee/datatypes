//! The responses a admin will get from requests to the service

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
