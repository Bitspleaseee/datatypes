//! The responses a user will get on requests to the content-database

use crate::payloads::TokenPayload;
use crate::valid::fields::*;
use crate::valid::ids::*;
use crate::Token;

pub type IntContentRequestSuccess<'a> = TokenPayload<ContentRequestSuccess<'a>, Token>;
pub type IntContentRequestError = TokenPayload<ContentError, Token>;

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
pub enum ContentError {
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

#[derive(Getters, Serialize, Deserialize, Debug, PartialEq)]
pub struct CategoryPayload<'a> {
    #[get]
    id: CategoryId,
    #[serde(borrow)] #[get]
    title: Title<'a>,
    #[serde(borrow)] #[get]
    description: Description<'a>,
}

#[derive(Getters, Serialize, Deserialize, Debug, PartialEq)]
pub struct ThreadPayload<'a> {
    #[get]
    id: ThreadId,
    #[get]
    category_id: CategoryId,
    #[get]
    user_id: UserId,
    #[serde(borrow)] #[get]
    title: Title<'a>,
    #[serde(borrow)] #[get]
    description: Description<'a>,
    #[get]
    timestamp: i64, // TODO change to chrono type?
}

#[derive(Getters, Serialize, Deserialize, Debug, PartialEq)]
pub struct CommentPayload<'a> {
    #[get]
    id: CommentId,
    #[get]
    thread_id: ThreadId,
    #[get]
    parent_id: Option<CommentId>,
    #[get]
    user_id: UserId,
    #[serde(borrow)] #[get]
    title: Title<'a>,
    #[serde(borrow)] #[get]
    description: Description<'a>,
    #[get]
    timestamp: i64, // TODO change to chrono type?
}

#[derive(Getters, Serialize, Deserialize, Debug, PartialEq)]
pub struct UserPayload<'a> {
    #[get]
    id: CommentId,
    #[serde(borrow)] #[get]
    username: Username<'a>,
    #[serde(borrow)] #[get]
    description: Description<'a>,
    #[serde(borrow)] #[get]
    avatar: &'a str,
}
