//! The requests a user can make to the content-database

use crate::payloads::{TokenPayload, UserIdPayload};
use crate::valid::fields::*;
use crate::valid::ids::*;
use crate::valid::token::Token;

pub type TokenContentRequest = TokenPayload<ContentRequest, Token>;
pub type UserIdContentRequest = UserIdPayload<ContentRequest>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum ContentRequest {
    AddCategory(AddCategoryPayload),
    EditCategory(EditCategoryPayload),
    HideCategory(HideCategoryPayload),
    AddThread(AddThreadPayload),
    EditThread(EditThreadPayload),
    HideThread(HideThreadPayload),
    AddComment(AddCommentPayload),
    EditComment(EditCommentPayload),
    HideComment(HideCommentPayload),
    EditUser(EditUserPayload),
    UploadAvatar(UploadAvatarPayload),
    Search(SearchPayload),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AddCategoryPayload {
    pub title: Title,
    pub description: Description,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct EditCategoryPayload {
    pub id: CategoryId,
    pub title: Option<Title>,
    pub description: Option<Description>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct HideCategoryPayload {
    pub id: CategoryId,
    pub hide: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AddThreadPayload {
    pub category_id: CategoryId,
    pub user_id: UserId,
    pub title: Title,
    pub description: Description,
    pub timestamp: i64, // TODO change to chrono type?
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct EditThreadPayload {
    pub id: ThreadId,
    pub title: Option<Title>,
    pub description: Option<Description>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct HideThreadPayload {
    pub id: ThreadId,
    pub hide: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AddCommentPayload {
    pub thread_id: ThreadId,
    pub user_id: UserId,
    pub parent_id: Option<CommentId>,
    pub content: CommentContent,
    pub timestamp: i64, // TODO change to chrono type?
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct EditCommentPayload {
    pub id: CommentId,
    pub content: CommentContent,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct HideCommentPayload {
    pub id: CommentId,
    pub hide: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AddUserPayload {
    pub id: UserId,
    pub username: Username,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct EditUserPayload {
    pub id: UserId,
    pub description: Description,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct UploadAvatarPayload {
    pub id: UserId,
    pub avatar: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SearchPayload {
    pub query: QueryStr,
}
