//! The responses a user will get on requests to the content-database

use crate::payloads::TokenPayload;
use crate::valid::fields::*;
use crate::valid::ids::*;
use crate::Token;

pub type IntContentSuccess = TokenPayload<ContentSuccess, Token>;
pub type IntContentError = TokenPayload<ContentError, Token>;

/// All the successful responses to a `ContentRequest`
#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum ContentSuccess {
    Category(CategoryPayload),
    Categories(Vec<CategoryPayload>),
    Thread(ThreadPayload),
    Threads(Vec<ThreadPayload>),
    Comment(CommentPayload),
    Comments(Vec<CommentPayload>),
    User(UserPayload),
    Users(Vec<UserPayload>),
    SearchResult(SearchResultsPayload),
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
    #[fail(display = "invalid query string")]
    InvalidQuery,
}

#[derive(Getters, Serialize, Deserialize, Debug, PartialEq)]
pub struct CategoryPayload {
    #[get]
    id: CategoryId,
    #[get]
    title: Title,
    #[get]
    description: Description,
}

#[derive(Getters, Serialize, Deserialize, Debug, PartialEq)]
pub struct ThreadPayload {
    #[get]
    id: ThreadId,
    #[get]
    category_id: CategoryId,
    #[get]
    user_id: UserId,
    #[get]
    title: Title,
    #[get]
    description: Description,
    #[get]
    timestamp: i64, // TODO change to chrono type?
}

#[derive(Getters, Serialize, Deserialize, Debug, PartialEq)]
pub struct CommentPayload {
    #[get]
    id: CommentId,
    #[get]
    thread_id: ThreadId,
    #[get]
    parent_id: Option<CommentId>,
    #[get]
    user_id: UserId,
    #[get]
    title: Title,
    #[get]
    description: Description,
    #[get]
    timestamp: i64, // TODO change to chrono type?
}

#[derive(Getters, Serialize, Deserialize, Debug, PartialEq)]
pub struct UserPayload {
    #[get]
    id: CommentId,
    #[get]
    username: Username,
    #[get]
    description: Description,
    #[get]
    avatar: String,
}

/// A search result which contains all the elements that matched the search
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SearchResultsPayload {
    categories: Vec<CategoryPayload>,
    threads: Vec<ThreadPayload>,
    comments: Vec<CommentPayload>,
    users: Vec<UserPayload>,
}
