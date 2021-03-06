//! The responses a user will get on requests to the content-database

use chrono::naive::NaiveDateTime;
use crate::valid::fields::*;
use crate::valid::ids::*;

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
pub enum ContentError {
    #[fail(display = "content was not found")]
    MissingContent,
    #[fail(display = "content is hidden")]
    HiddenContent,
    #[fail(display = "internal server error")]
    InternalServerError,
    #[fail(display = "invalid search query")]
    InvalidSearchQuery,
    #[fail(display = "invalid id")]
    InvalidId,
    #[fail(display = "invalid content")]
    InvalidContent,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct UserPayload {
    pub id: UserId,
    pub username: Username,
    pub description: Option<Description>,
    pub avatar: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CategoryPayload {
    pub id: CategoryId,
    pub title: Title,
    pub description: Description,
    pub hidden: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThreadPayload {
    pub id: ThreadId,
    pub category_id: CategoryId,
    pub user_id: UserId,
    pub title: Title,
    pub description: Description,
    pub timestamp: NaiveDateTime,
    pub hidden: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CommentPayload {
    pub id: CommentId,
    pub thread_id: ThreadId,
    pub parent_id: Option<CommentId>,
    pub user_id: UserId,
    pub content: CommentContent,
    pub timestamp: NaiveDateTime,
    pub hidden: bool,
}

/// A search result which contains all the elements that matched the search
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SearchResultsPayload {
    pub categories: Vec<CategoryPayload>,
    pub threads: Vec<ThreadPayload>,
    pub comments: Vec<CommentPayload>,
    pub users: Vec<UserPayload>,
}
