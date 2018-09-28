//! The responses a user will get on requests to the content-database

// TODO populate responses with proper payloads

#[derive(Serialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum ContentRequestSuccess {
    MultipleCategories,
    SingleCategory,
    MultipleThreads,
    SingleThread,
    MultipleComments,
    SingleComment,
}

#[derive(Fail, Serialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum ContentRequestError {
    #[fail(display = "content is hidden")]
    Hidden,
    #[fail(display = "passed invalid id")]
    InvalidId,
    #[fail(display = "token missing from cookies")]
    MissingToken,
    #[fail(display = "passed invalid token")]
    InvalidToken,
    #[fail(display = "passed invalid query")]
    InvalidQuery,
}

