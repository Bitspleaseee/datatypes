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
    #[get = "pub"]
    id: CategoryId,
    #[get = "pub"]
    title: Title,
    #[get = "pub"]
    description: Description,
}

impl CategoryPayload {
    pub fn new(id: impl Into<CategoryId>, title: Title, description: Description) -> Self {
        CategoryPayload {
            id: id.into(),
            title,
            description,
        }
    }
}

#[derive(Getters, Serialize, Deserialize, Debug, PartialEq)]
pub struct ThreadPayload {
    #[get = "pub"]
    id: ThreadId,
    #[get = "pub"]
    category_id: CategoryId,
    #[get = "pub"]
    user_id: UserId,
    #[get = "pub"]
    title: Title,
    #[get = "pub"]
    description: Description,
    #[get = "pub"]
    timestamp: i64, // TODO change to chrono type?
}

#[derive(Getters, Serialize, Deserialize, Debug, PartialEq)]
pub struct CommentPayload {
    #[get = "pub"]
    id: CommentId,
    #[get = "pub"]
    thread_id: ThreadId,
    #[get = "pub"]
    parent_id: Option<CommentId>,
    #[get = "pub"]
    user_id: UserId,
    #[get = "pub"]
    title: Title,
    #[get = "pub"]
    description: Description,
    #[get = "pub"]
    timestamp: i64, // TODO change to chrono type?
}

#[derive(Getters, Serialize, Deserialize, Debug, PartialEq)]
pub struct UserPayload {
    #[get = "pub"]
    id: UserId,
    #[get = "pub"]
    username: Username,
    #[get = "pub"]
    description: Option<Description>,
    #[get = "pub"]
    avatar: Option<String>,
}
impl UserPayload {
    pub fn new(
        id: impl Into<UserId>,
        username: Username,
        description: Option<Description>,
        avatar: Option<String>,
    ) -> Self {
        UserPayload {
            id: id.into(),
            username,
            description,
            avatar,
        }
    }
}

/// A search result which contains all the elements that matched the search
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SearchResultsPayload {
    categories: Vec<CategoryPayload>,
    threads: Vec<ThreadPayload>,
    comments: Vec<CommentPayload>,
    users: Vec<UserPayload>,
}
