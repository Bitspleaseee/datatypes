//! Raw datatypes which can be deserialized

/// A raw representation of a category
#[derive(Serialize, Deserialize, Debug)]
pub struct RawCategory<'a> {
    id: u32,
    title: &'a str,
    description: &'a str,
}

/// A raw representation of a thread
#[derive(Serialize, Deserialize, Debug)]
pub struct RawThread<'a> {
    id: u32,
    category_id: u32,
    user_id: u32,
    title: &'a str,
    description: &'a str,
    /// UNIX timestamp
    timestamp: i64, // TODO perhaps change to a `DateTime`?
}

/// A raw representation of a comment
#[derive(Serialize, Deserialize, Debug)]
pub struct RawComment<'a> {
    id: u32,
    thread_id: u32,
    user_id: u32,
    parent_id: Option<u32>,
    content: &'a str,
}

/// A raw representation of a user (content-database)
#[derive(Serialize, Deserialize, Debug)]
pub struct RawContentUser<'a> {
    id: u32,
    username: &'a str,
    description: &'a str,
    avatar: &'a str,
}

/// A raw representation of a user (auth-database)
#[derive(Serialize, Deserialize, Debug)]
pub struct RawAuthUser<'a> {
    id: u32,
    email: &'a str,
    username: &'a str,
    password: &'a str,
}
