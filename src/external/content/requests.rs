//! The requests a user can make to the content-database

use crate::valid::fields::*;
use crate::valid::ids::*;
use crate::valid::ValidationError;
use std::convert::{TryFrom, TryInto};

use self::raw::*;
use self::valid::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum RawContentRequests<'a> {
    AddCategory(#[serde(borrow)] RawAddCategoryPayload<'a>),
    EditCategory(#[serde(borrow)] RawEditCategoryPayload<'a>),
    HideCategory(RawHideCategoryPayload),
    AddThread(#[serde(borrow)] RawAddThreadPayload<'a>),
    EditThread(#[serde(borrow)] RawEditThreadPayload<'a>),
    HideThread(RawHideThreadPayload),
    AddComment(#[serde(borrow)] RawAddCommentPayload<'a>),
    EditComment(#[serde(borrow)] RawEditCommentPayload<'a>),
    HideComment(RawHideCommentPayload),
    UploadAvatar(#[serde(borrow)] RawUploadAvatarPayload<'a>),
}

#[derive(Serialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum ContentRequests<'a> {
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

impl<'a> TryFrom<RawContentRequests<'a>> for ContentRequests<'a> {
    type Error = ValidationError;
    fn try_from(raw: RawContentRequests<'a>) -> Result<Self, Self::Error> {
        use self::ContentRequests as Valid;
        use self::RawContentRequests as Raw;
        match raw {
            Raw::AddCategory(p) => p.try_into().map(Valid::AddCategory),
            Raw::EditCategory(p) => p.try_into().map(Valid::EditCategory),
            Raw::HideCategory(p) => p.try_into().map(Valid::HideCategory),
            Raw::AddThread(p) => p.try_into().map(Valid::AddThread),
            Raw::EditThread(p) => p.try_into().map(Valid::EditThread),
            Raw::HideThread(p) => p.try_into().map(Valid::HideThread),
            Raw::AddComment(p) => p.try_into().map(Valid::AddComment),
            Raw::EditComment(p) => p.try_into().map(Valid::EditComment),
            Raw::HideComment(p) => p.try_into().map(Valid::HideComment),
            Raw::UploadAvatar(p) => p.try_into().map(Valid::UploadAvatar),
        }
    }
}

raw_to_valid! {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    #[serde(rename = "payload")]
    pub struct RawAddCategoryPayload<'a>;

    #[derive(Serialize, PartialEq, Debug)]
    #[serde(rename = "payload")]
    pub struct AddCategoryPayload<'a>;

    impl<'a> TryFrom<RawAddCategoryPayload<'a>> for AddCategoryPayload<'a> {
        > required
            title: &'a str => Title<'a>,
            description: &'a str => Description<'a>
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    #[serde(rename = "payload")]
    pub struct RawEditCategoryPayload<'a>;

    #[derive(Serialize, PartialEq, Debug)]
    #[serde(rename = "payload")]
    pub struct EditCategoryPayload<'a>;

    impl<'a> TryFrom<RawEditCategoryPayload<'a>> for EditCategoryPayload<'a> {
        > optional
            title: &'a str => Title<'a>,
            description: &'a str => Description<'a>
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    #[serde(rename = "payload")]
    pub struct RawHideCategoryPayload;

    #[derive(Serialize, PartialEq, Debug)]
    #[serde(rename = "payload")]
    pub struct HideCategoryPayload;

    impl TryFrom<RawHideCategoryPayload> for HideCategoryPayload {
        > required
        >-> no-validate
            hidden: bool => bool
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    #[serde(rename = "payload")]
    pub struct RawAddThreadPayload<'a>;

    #[derive(Serialize, PartialEq, Debug)]
    #[serde(rename = "payload")]
    pub struct AddThreadPayload<'a>;

    impl<'a> TryFrom<RawAddThreadPayload<'a>> for AddThreadPayload<'a> {
        > required
            title: &'a str => Title<'a>,
            description: &'a str => Description<'a>
        >-> no-validate
            category_id: u32 => CategoryId,
            user_id: u32 => UserId,
            timestamp: i64 => i64
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    #[serde(rename = "payload")]
    pub struct RawEditThreadPayload<'a>;

    #[derive(Serialize, PartialEq, Debug)]
    #[serde(rename = "payload")]
    pub struct EditThreadPayload<'a>;

    impl<'a> TryFrom<RawEditThreadPayload<'a>> for EditThreadPayload<'a> {
        > optional
            title: &'a str => Title<'a>,
            description: &'a str => Description<'a>
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    #[serde(rename = "payload")]
    pub struct RawHideThreadPayload;

    #[derive(Serialize, PartialEq, Debug)]
    #[serde(rename = "payload")]
    pub struct HideThreadPayload;

    impl TryFrom<RawHideThreadPayload> for HideThreadPayload {
        > required
        >-> no-validate
            hidden: bool => bool
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    #[serde(rename = "payload")]
    pub struct RawAddCommentPayload<'a>;

    #[derive(Serialize, PartialEq, Debug)]
    #[serde(rename = "payload")]
    pub struct AddCommentPayload<'a>;

    impl<'a> TryFrom<RawAddCommentPayload<'a>> for AddCommentPayload<'a> {
        > required
            title: &'a str => Title<'a>,
            content: &'a str => CommentContent<'a>
        >-> no-validate
            thread_id: u32 => ThreadId,
            user_id: u32 => UserId,
            timestamp: i64 => i64
        > optional
        >-> no-validate
            parent_id: u32 => CommentId
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    #[serde(rename = "payload")]
    pub struct RawEditCommentPayload<'a>;

    #[derive(Serialize, PartialEq, Debug)]
    #[serde(rename = "payload")]
    pub struct EditCommentPayload<'a>;

    impl<'a> TryFrom<RawEditCommentPayload<'a>> for EditCommentPayload<'a> {
        > optional
            title: &'a str => Title<'a>,
            content: &'a str => CommentContent<'a>
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    #[serde(rename = "payload")]
    pub struct RawHideCommentPayload;

    #[derive(Serialize, PartialEq, Debug)]
    #[serde(rename = "payload")]
    pub struct HideCommentPayload;

    impl TryFrom<RawHideCommentPayload> for HideCommentPayload {
        > required
        >-> no-validate
            hidden: bool => bool
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    #[serde(rename = "payload")]
    pub struct RawUploadAvatarPayload<'a>;

    #[derive(Serialize, PartialEq, Debug)]
    #[serde(rename = "payload")]
    pub struct UploadAvatarPayload<'a>;

    impl<'a> TryFrom<RawUploadAvatarPayload<'a>> for UploadAvatarPayload<'a> {
        > required
        >-> no-validate
            user_id: u32 => UserId,
            avatar: &'a str => &'a str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_to_valid_edit_category() {
        let raw = RawEditCategoryPayload {
            title: None,
            description: Some("a description"),
        };

        let valid = EditCategoryPayload {
            title: None,
            description: Some("a description".try_into().unwrap()),
        };

        let raw = raw.try_into().expect("raw should be valid");
        assert_eq!(valid, raw);
    }

    #[test]
    fn raw_to_valid_add_category() {
        let raw = RawAddCategoryPayload {
            title: "Raw title",
            description: "Raw description",
        };

        let valid = AddCategoryPayload {
            title: "Raw title".try_into().unwrap(),
            description: "Raw description".try_into().unwrap(),
        };

        let raw = raw.try_into().expect("raw should be valid");
        assert_eq!(valid, raw);
    }
    #[test]
    fn raw_to_valid_add_category_empty_title() {
        let raw = RawAddCategoryPayload {
            title: "",
            description: "Raw description",
        };

        let raw: Result<AddCategoryPayload, _> = raw.try_into();
        assert_eq!(ValidationError::InvalidTitle, raw.unwrap_err());
    }
    #[test]
    fn raw_to_valid_add_category_empty_description() {
        let raw = RawAddCategoryPayload {
            title: "Some title",
            description: "",
        };

        let raw: Result<AddCategoryPayload, _> = raw.try_into();
        assert_eq!(ValidationError::InvalidDescription, raw.unwrap_err());
    }
}
