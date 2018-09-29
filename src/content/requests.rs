//! The requests a user can make to the content-database

use crate::valid::fields::*;
use crate::valid::ids::*;

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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AddCategoryPayload<'a> {
    #[serde(borrow)]
    title: Title<'a>,
    #[serde(borrow)]
    description: Description<'a>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct EditCategoryPayload<'a> {
    #[serde(borrow)]
    title: Option<Title<'a>>,
    #[serde(borrow)]
    description: Option<Description<'a>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct HideCategoryPayload {
    hide: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AddThreadPayload<'a> {
    category_id: CategoryId,
    user_id: UserId,
    #[serde(borrow)]
    title: Title<'a>,
    #[serde(borrow)]
    description: Description<'a>,
    timestamp: i64, // TODO change to chrono type?
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct EditThreadPayload<'a> {
    #[serde(borrow)]
    title: Option<Title<'a>>,
    #[serde(borrow)]
    description: Option<Description<'a>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct HideThreadPayload {
    hide: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AddCommentPayload<'a> {
    thread_id: ThreadId,
    user_id: UserId,
    parent_id: Option<CommentId>,
    #[serde(borrow)]
    content: CommentContent<'a>,
    timestamp: i64, // TODO change to chrono type?
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct EditCommentPayload<'a> {
    #[serde(borrow)]
    content: CommentContent<'a>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct HideCommentPayload {
    hide: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct UploadAvatarPayload<'a> {
    #[serde(borrow)]
    avatar: &'a str,
}
