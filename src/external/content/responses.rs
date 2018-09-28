//! The responses a user will get on requests to the content-database

use std::convert::TryFrom;

use crate::valid::fields::*;
use crate::valid::ids::*;
use crate::valid::ValidationError;

use self::raw::*;
use self::valid::*;

/// All the successful responses to a `ContentRequest`
#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum RawContentRequestSuccess<'a> {
    Category(#[serde(borrow)] RawCategoryPayload<'a>),
    Categories(#[serde(borrow)] Vec<RawCategoryPayload<'a>>),
    Thread(#[serde(borrow)] RawThreadPayload<'a>),
    Threads(#[serde(borrow)] Vec<RawThreadPayload<'a>>),
    Comment(#[serde(borrow)] RawCommentPayload<'a>),
    Comments(#[serde(borrow)] Vec<RawCommentPayload<'a>>),
    User(#[serde(borrow)] RawUserPayload<'a>),
    Users(#[serde(borrow)] Vec<RawUserPayload<'a>>),
}

/// All the successful responses to a `ContentRequest`
#[derive(Serialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum ContentRequestSuccess<'a> {
    Category(CategoryPayload<'a>),
    Categories(Vec<CategoryPayload<'a>>),
    Thread(ThreadPayload<'a>),
    Threads(Vec<ThreadPayload<'a>>),
    Comment(CommentPayload<'a>),
    Comments(Vec<CommentPayload<'a>>),
    User(UserPayload<'a>),
    Users(Vec<UserPayload<'a>>),
}

impl<'a> TryFrom<RawContentRequestSuccess<'a>> for ContentRequestSuccess<'a> {
    type Error = ValidationError;
    fn try_from(p: RawContentRequestSuccess<'a>) -> Result<Self, Self::Error> {
        use self::ContentRequestSuccess as Valid;
        use self::RawContentRequestSuccess as Raw;
        use std::convert::TryInto;
        match p {
            Raw::Category(p) => p.try_into().map(Valid::Category),
            Raw::Thread(p) => p.try_into().map(Valid::Thread),
            Raw::Comment(p) => p.try_into().map(Valid::Comment),
            Raw::User(p) => p.try_into().map(Valid::User),
            Raw::Categories(p) => p
                .into_iter()
                .map(|e| e.try_into())
                .collect::<Result<_, _>>()
                .map(Valid::Categories),
            Raw::Threads(p) => p
                .into_iter()
                .map(|e| e.try_into())
                .collect::<Result<_, _>>()
                .map(Valid::Threads),
            Raw::Comments(p) => p
                .into_iter()
                .map(|e| e.try_into())
                .collect::<Result<_, _>>()
                .map(Valid::Comments),
            Raw::Users(p) => p
                .into_iter()
                .map(|e| e.try_into())
                .collect::<Result<_, _>>()
                .map(Valid::Users),
        }
    }
}

/// All the unsuccessful responses to a `ContentRequest`
#[derive(Fail, Serialize, Deserialize, PartialEq, PartialOrd, Debug)]
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

raw_to_valid! {
    #[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug)]
    pub struct RawCategoryPayload<'a>;

    #[derive(Serialize, PartialEq, PartialOrd, Debug)]
    pub struct CategoryPayload<'a>;

    impl<'a> TryFrom<RawCategoryPayload<'a>> for CategoryPayload<'a> {
        > required
            title: &'a str => Title<'a>,
            description: &'a str => Description<'a>
        >-> no-validate
            id: u32 => CategoryId
    }
    #[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug)]
    pub struct RawThreadPayload<'a>;

    #[derive(Serialize, PartialEq, PartialOrd, Debug)]
    pub struct ThreadPayload<'a>;

    impl<'a> TryFrom<RawThreadPayload<'a>> for ThreadPayload<'a> {
        > required
            title: &'a str => Title<'a>,
            description: &'a str => Description<'a>
        >-> no-validate
            id: u32 => ThreadId,
            category_id: u32 => CategoryId,
            timestamp: i64 => i64
    }
    #[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug)]
    pub struct RawCommentPayload<'a>;

    #[derive(Serialize, PartialEq, PartialOrd, Debug)]
    pub struct CommentPayload<'a>;

    impl<'a> TryFrom<RawCommentPayload<'a>> for CommentPayload<'a> {
        > required
            title: &'a str => Title<'a>,
            content: &'a str => CommentContent<'a>
        >-> no-validate
            id: u32 => CommentId,
            thread_id: u32 => ThreadId,
            timestamp: i64 => i64
        > optional
        >-> no-validate
            parent_id: u32 => CommentId
    }

    #[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug)]
    pub struct RawUserPayload<'a>;

    #[derive(Serialize, PartialEq, PartialOrd, Debug)]
    pub struct UserPayload<'a>;

    impl<'a> TryFrom<RawUserPayload<'a>> for UserPayload<'a> {
        > required
            username: &'a str => Username<'a>,
            description: &'a str => Description<'a>
        >-> no-validate
            id: u32 => UserId
    }

}
