//! The requests a user can make to the content-database

// TODO populate these requests with proper payload

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum ContentRequests {
    AddThread,
    EditThread,
    HideThread,
    AddCategory,
    EditCategory,
    HideCategory,
    AddComment,
    EditComment,
    HideComment,
    UploadAvatar,
}
