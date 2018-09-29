//! The responses a user will get on requests to the content-database

use crate::payloads::TokenPayload;
use crate::valid::fields::*;
use crate::valid::ids::*;
use crate::Token;

pub type IntContentRequestSuccess<'a> = TokenPayload<ContentRequestSuccess<'a>, Token>;
pub type IntContentRequestError = TokenPayload<ContentRequestError, Token>;

/// All the successful responses to a `ContentRequest`
#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum ContentRequestSuccess<'a> {
    Category(#[serde(borrow)] CategoryPayload<'a>),
    Categories(#[serde(borrow)] Vec<CategoryPayload<'a>>),
    Thread(#[serde(borrow)] ThreadPayload<'a>),
    Threads(#[serde(borrow)] Vec<ThreadPayload<'a>>),
    Comment(#[serde(borrow)] CommentPayload<'a>),
    Comments(#[serde(borrow)] Vec<CommentPayload<'a>>),
    User(#[serde(borrow)] UserPayload<'a>),
    Users(#[serde(borrow)] Vec<UserPayload<'a>>),
}

/// All the unsuccessful responses to a `ContentRequest`
#[derive(Fail, Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum ContentRequestError {
    #[fail(display = "content is missing or hidden")]
    MissingContent,
    #[fail(display = "passed invalid id")]
    InvalidId,
    #[fail(display = "token missing from cookies")]
    MissingToken,
    #[fail(display = "passed invalid token")]
    InvalidToken,
    #[fail(display = "passed invalid query")]
    InvalidQuery,
    #[fail(display = "internal server error")]
    ServerError,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CategoryPayload<'a> {
    id: CategoryId,
    #[serde(borrow)]
    title: Title<'a>,
    #[serde(borrow)]
    description: Description<'a>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThreadPayload<'a> {
    id: ThreadId,
    category_id: CategoryId,
    user_id: UserId,
    #[serde(borrow)]
    title: Title<'a>,
    #[serde(borrow)]
    description: Description<'a>,
    timestamp: i64, // TODO change to chrono type?
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CommentPayload<'a> {
    id: CommentId,
    thread_id: ThreadId,
    parent_id: Option<CommentId>,
    user_id: UserId,
    #[serde(borrow)]
    title: Title<'a>,
    #[serde(borrow)]
    description: Description<'a>,
    timestamp: i64, // TODO change to chrono type?
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct UserPayload<'a> {
    id: CommentId,
    #[serde(borrow)]
    username: Username<'a>,
    #[serde(borrow)]
    description: Description<'a>,
    #[serde(borrow)]
    avatar: &'a str,
}
