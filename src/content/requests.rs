//! The requests a user can make to the content-database

use crate::payloads::TokenPayload;
use crate::valid::fields::*;
use crate::valid::ids::*;
use crate::Token;

pub type IntContentRequest = TokenPayload<ContentRequest, Token>;

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
    UploadAvatar(UploadAvatarPayload),
    Search(SearchPayload),
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct AddCategoryPayload {
    #[get]
    title: Title,
    #[get]
    description: Description,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct EditCategoryPayload {
    #[get]
    title: Option<Title>,
    #[get]
    description: Option<Description>,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct HideCategoryPayload {
    #[get]
    hide: bool,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct AddThreadPayload {
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

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct EditThreadPayload {
    #[get]
    title: Option<Title>,
    #[get]
    description: Option<Description>,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct HideThreadPayload {
    #[get]
    hide: bool,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct AddCommentPayload {
    #[get]
    thread_id: ThreadId,
    #[get]
    user_id: UserId,
    #[get]
    parent_id: Option<CommentId>,
    #[get]
    content: CommentContent,
    #[get]
    timestamp: i64, // TODO change to chrono type?
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct EditCommentPayload {
    #[get]
    content: CommentContent,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct HideCommentPayload {
    #[get]
    hide: bool,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct UploadAvatarPayload {
    #[get]
    avatar: String,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct SearchPayload {
    #[get]
    query: QueryStr,
}
