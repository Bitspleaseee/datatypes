//! The requests a user can make to the content-database

use crate::valid::fields::*;
use crate::valid::ids::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum ContentRequest {
    AddUser(AddUserPayload),
    EditUser(EditUserPayload),

    AddCategory(AddCategoryPayload),
    EditCategory(EditCategoryPayload),
    HideCategory(HideCategoryPayload),

    AddThread(AddThreadPayload),
    EditThread(EditThreadPayload),
    HideThread(HideThreadPayload),

    AddComment(AddCommentPayload),
    EditComment(EditCommentPayload),
    HideComment(HideCommentPayload),
}

// Users

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct GetUserPayload {
    pub id: UserId,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AddUserPayload {
    pub id: UserId,
    pub username: Username,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct EditUserPayload {
    pub id: UserId,
    pub description: Option<Description>,
    pub avatar: Option<String>,
}

// Categories

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct GetCategoryPayload {
    pub id: CategoryId,
    pub include_hidden: bool,
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

// Threads

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct GetThreadPayload {
    pub id: ThreadId,
    pub include_hidden: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct GetThreadsPayload {
    pub id: CategoryId,
    pub include_hidden: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AddThreadPayload {
    pub category_id: CategoryId,
    pub user_id: UserId,
    pub title: Title,
    pub description: Description,
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

// Comments

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct GetCommentPayload {
    pub id: CommentId,
    pub include_hidden: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct GetCommentsPayload {
    pub id: ThreadId,
    pub include_hidden: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AddCommentPayload {
    pub thread_id: ThreadId,
    pub user_id: UserId,
    pub parent_id: Option<CommentId>,
    pub content: CommentContent,
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

// Search

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SearchPayload {
    pub query: QueryStr,
    pub include_hidden: bool,
}

// Hidden

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct GetHiddenPayload {
    pub include_hidden: bool,
}
