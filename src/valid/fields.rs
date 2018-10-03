//! Validated datafields

// TODO add tests which vertifies the `TryFrom` implementations

use super::ValidationError;
use rocket::http::RawStr;
use rocket::request::FromFormValue;
use std::convert::TryFrom;
use std::fmt::{self, Display};
use std::ops::Deref;

use super::{EMAIL_REGEX, PASSWORD_REGEX, SEARCH_QUERY_REGEX, USERNAME_REGEX};
use regex::Regex;

/// A valid (well formatted) username
#[derive(Serialize, PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct Username(String);

impl TryFrom<String> for Username {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex = USERNAME_REGEX.parse().expect("invalid username regex");
        }
        if RE.is_match(&s) {
            Ok(Username(s))
        } else {
            Err(ValidationError::InvalidUsername)
        }
    }
}

impl_deserialize_with_try_from!(Username);
impl_deref_and_as_ref!(Username, str);
impl_get_string!(Username);

impl Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A valid (well formatted) plaintext password
///
/// NB This type does not implement `Debug` for the simple reason that a
/// plaintext passwords should **never** be printed.
#[derive(Serialize, PartialEq, PartialOrd, Eq, Ord, Clone)]
#[serde(rename = "password")]
pub struct PlainPassword(String);

impl TryFrom<String> for PlainPassword {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex = PASSWORD_REGEX.parse().expect("invalid password regex");
        }
        if RE.is_match(&s) {
            Ok(PlainPassword(s))
        } else {
            Err(ValidationError::InvalidPassword)
        }
    }
}

impl_deserialize_with_try_from!(PlainPassword);
impl_deref_and_as_ref!(PlainPassword, str);

/// A valid (well formatted) title
#[derive(Serialize, PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct Title(String);

impl TryFrom<String> for Title {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        if 4 < s.len() && s.len() < 80 {
            Ok(Title(s))
        } else {
            Err(ValidationError::InvalidTitle)
        }
    }
}

impl_deserialize_with_try_from!(Title);
impl_deref_and_as_ref!(Title, str);
impl_get_string!(Title);

impl Display for Title {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A valid (well formatted) description
#[derive(Serialize, PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct Description(String);

impl TryFrom<String> for Description {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.len() < 255 {
            Ok(Description(s))
        } else {
            Err(ValidationError::InvalidDescription)
        }
    }
}

impl_deserialize_with_try_from!(Description);
impl_deref_and_as_ref!(Description, str);
impl_get_string!(Description);

impl Display for Description {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A valid (well formatted) comment-content
#[derive(Serialize, PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct CommentContent(String);

impl TryFrom<String> for CommentContent {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.len() > 4 && s.len() < 80 {
            Ok(CommentContent(s))
        } else {
            Err(ValidationError::InvalidCommentContent)
        }
    }
}

impl_deserialize_with_try_from!(CommentContent);
impl_deref_and_as_ref!(CommentContent, str);
impl_get_string!(CommentContent);

impl Display for CommentContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A valid (well formatted) email
#[derive(Serialize, PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct Email(String);

impl TryFrom<String> for Email {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex = EMAIL_REGEX.parse().expect("invalid email regex");
        }
        if RE.is_match(&s) {
            Ok(Email(s))
        } else {
            Err(ValidationError::InvalidEmail)
        }
    }
}

impl_deserialize_with_try_from!(Email);
impl_deref_and_as_ref!(Email, str);
impl_get_string!(Email);

impl Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A valid (well formatted) search query string
#[derive(Serialize, PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct QueryStr(String);

impl TryFrom<String> for QueryStr {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex = SEARCH_QUERY_REGEX.parse().expect("invalid email regex");
        }
        if RE.is_match(&s) {
            Ok(QueryStr(s))
        } else {
            Err(ValidationError::InvalidQuery)
        }
    }
}

impl_deserialize_with_try_from!(QueryStr);
impl_deref_and_as_ref!(QueryStr, str);

impl Display for QueryStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> FromFormValue<'a> for QueryStr {
    type Error = <QueryStr as TryFrom<String>>::Error;
    fn from_form_value(search_str: &'a RawStr) -> Result<Self, Self::Error> {
        let s: &'a str = search_str.as_ref();
        QueryStr::try_from(s.to_string())
    }
}
