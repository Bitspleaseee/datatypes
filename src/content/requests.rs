//! The requests a user can make to the content-database

use crate::payloads::TokenPayload;
use crate::valid::fields::*;
use crate::valid::ids::*;
use crate::Token;

pub type IntContentRequest<'a> = TokenPayload<ContentRequest<'a>, Token>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum ContentRequest<'a> {
    AddCategory(#[serde(borrow)] AddCategoryPayload<'a>),
    EditCategory(#[serde(borrow)] EditCategoryPayload<'a>),
    HideCategory(HideCategoryPayload),
    AddThread(#[serde(borrow)] AddThreadPayload<'a>),
    EditThread(#[serde(borrow)] EditThreadPayload<'a>),
    HideThread(HideThreadPayload),
    AddComment(#[serde(borrow)] AddCommentPayload<'a>),
    EditComment(#[serde(borrow)] EditCommentPayload<'a>),
    HideComment(HideCommentPayload),
    UploadAvatar(#[serde(borrow)] UploadAvatarPayload<'a>),
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct AddCategoryPayload<'a> {
    #[serde(borrow)] #[get]
    title: Title<'a>,
    #[serde(borrow)] #[get]
    description: Description<'a>,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct EditCategoryPayload<'a> {
    #[serde(borrow)] #[get]
    title: Option<Title<'a>>,
    #[serde(borrow)] #[get]
    description: Option<Description<'a>>,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct HideCategoryPayload {
    #[get]
    hide: bool,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct AddThreadPayload<'a> {
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

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct EditThreadPayload<'a> {
    #[serde(borrow)] #[get]
    title: Option<Title<'a>>,
    #[serde(borrow)] #[get]
    description: Option<Description<'a>>,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct HideThreadPayload {
    #[get]
    hide: bool,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct AddCommentPayload<'a> {
    #[get]
    thread_id: ThreadId,
    #[get]
    user_id: UserId,
    #[get]
    parent_id: Option<CommentId>,
    #[serde(borrow)] #[get]
    content: CommentContent<'a>,
    #[get]
    timestamp: i64, // TODO change to chrono type?
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct EditCommentPayload<'a> {
    #[serde(borrow)] #[get]
    content: CommentContent<'a>,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct HideCommentPayload {
    #[get]
    hide: bool,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct UploadAvatarPayload<'a> {
    #[serde(borrow)] #[get]
    avatar: &'a str,
}
