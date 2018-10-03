//! The requests a user can make to the content-database

use crate::payloads::{TokenPayload, UserIdPayload};
use crate::valid::fields::*;
use crate::valid::ids::*;
use crate::Token;

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

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct AddCategoryPayload {
    #[get = "pub"]
    title: Title,
    #[get = "pub"]
    description: Description,
}

impl AddCategoryPayload {
    pub fn new(title: Title, description: Description) -> Self {
        AddCategoryPayload { title, description }
    }
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct EditCategoryPayload {
    #[get = "pub"]
    id: CategoryId,
    #[get = "pub"]
    title: Option<Title>,
    #[get = "pub"]
    description: Option<Description>,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct HideCategoryPayload {
    #[get = "pub"]
    id: CategoryId,
    #[get = "pub"]
    hide: bool,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct AddThreadPayload {
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

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct EditThreadPayload {
    #[get = "pub"]
    id: ThreadId,
    #[get = "pub"]
    title: Option<Title>,
    #[get = "pub"]
    description: Option<Description>,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct HideThreadPayload {
    #[get = "pub"]
    id: ThreadId,
    #[get = "pub"]
    hide: bool,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct AddCommentPayload {
    #[get = "pub"]
    thread_id: ThreadId,
    #[get = "pub"]
    user_id: UserId,
    #[get = "pub"]
    parent_id: Option<CommentId>,
    #[get = "pub"]
    content: CommentContent,
    #[get = "pub"]
    timestamp: i64, // TODO change to chrono type?
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct EditCommentPayload {
    #[get = "pub"]
    id: CommentId,
    #[get = "pub"]
    content: CommentContent,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct HideCommentPayload {
    #[get = "pub"]
    id: CommentId,
    #[get = "pub"]
    hide: bool,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct AddUserPayload {
    #[get = "pub"]
    id: UserId,
    #[get = "pub"]
    username: Username,
}
impl AddUserPayload {
    pub fn new(id: impl Into<UserId>, username: Username) -> Self {
        AddUserPayload {
            id: id.into(),
            username,
        }
    }
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct EditUserPayload {
    #[get = "pub"]
    id: UserId,
    #[get = "pub"]
    description: Description,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct UploadAvatarPayload {
    #[get = "pub"]
    id: UserId,
    #[get = "pub"]
    avatar: String,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct SearchPayload {
    #[get = "pub"]
    query: QueryStr,
}
impl SearchPayload {
    pub fn new(query: QueryStr) -> Self {
        SearchPayload { query }
    }
}
