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
    #[get = "pub"]
    title: Title,
    #[get = "pub"]
    description: Description,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct EditCategoryPayload {
    #[get = "pub"]
    title: Option<Title>,
    #[get = "pub"]
    description: Option<Description>,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct HideCategoryPayload {
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
    title: Option<Title>,
    #[get = "pub"]
    description: Option<Description>,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct HideThreadPayload {
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
    content: CommentContent,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct HideCommentPayload {
    #[get = "pub"]
    hide: bool,
}

#[derive(Getters, Serialize, Deserialize, PartialEq, Debug)]
pub struct UploadAvatarPayload {
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
