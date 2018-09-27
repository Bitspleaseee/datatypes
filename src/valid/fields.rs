//! Validated datafields

use super::ValidationError;
use std::convert::TryFrom;
use std::fmt::{self, Display};

use super::{
    COMMENT_CONTENT_REGEX, DESCRIPTION_REGEX, EMAIL_REGEX, PASSWORD_REGEX, TITLE_REGEX,
    USERNAME_REGEX,
};
use regex::Regex;

// A macro too generate `TryFrom` implementations based on a regex in a simple
// way
macro_rules! try_from_regex {
    ($ty:ty, $regex:expr => ($constructor:expr, $err:expr)) => {
        impl<'a> TryFrom<&'a str> for $ty {
            type Error = ValidationError;
            fn try_from(s: &'a str) -> Result<Self, Self::Error> {
                lazy_static! {
                    static ref RE: Regex =
                        Regex::new($regex).expect(&format!("regex '{}' is invalid", $regex));
                }
                if RE.is_match(s) {
                    Ok($constructor(s))
                } else {
                    Err($err)
                }
            }
        }
    };
}

/// A valid (well formatted) username
///
/// NB This type does **not** implement `Deserialize` because it should only be
/// constructed through `try_into`
#[derive(Serialize, PartialEq, PartialOrd, Eq, Ord, Debug, Copy, Clone)]
pub struct Username<'a>(&'a str);
try_from_regex!(Username<'a>,
                USERNAME_REGEX => (Username, ValidationError::InvalidUsername));

impl<'a> Display for Username<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A valid (well formatted) plaintext password
///
/// NB This type does **not** implement `Deserialize` because it should only be
/// constructed through `try_into`.
///
/// NB This type does not implement `Debug` for the simple reason that a
/// plaintext passwords should **never** be printed.
#[derive(Serialize, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
#[serde(rename = "password")]
pub struct PlainPassword<'a>(&'a str);
try_from_regex!(PlainPassword<'a>,
                PASSWORD_REGEX =>
                (PlainPassword, ValidationError::InvalidPassword));

/// A valid (well formatted) title
///
/// NB This type does **not** implement `Deserialize` because it should only be
/// constructed through `try_into`.
#[derive(Serialize, PartialEq, PartialOrd, Eq, Ord, Debug, Copy, Clone)]
pub struct Title<'a>(&'a str);
try_from_regex!(Title<'a>,
                TITLE_REGEX =>
                (Title, ValidationError::InvalidTitle));

impl<'a> Display for Title<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A valid (well formatted) description
///
/// NB This type does **not** implement `Deserialize` because it should only be
/// constructed through `try_into`.
#[derive(Serialize, PartialEq, PartialOrd, Eq, Ord, Debug, Copy, Clone)]
pub struct Description<'a>(&'a str);
try_from_regex!(Description<'a>,
                DESCRIPTION_REGEX =>
                (Description, ValidationError::InvalidDescription));

impl<'a> Display for Description<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A valid (well formatted) comment-content
///
/// NB This type does **not** implement `Deserialize` because it should only be
/// constructed through `try_into`.
#[derive(Serialize, PartialEq, PartialOrd, Eq, Ord, Debug, Copy, Clone)]
pub struct CommentContent<'a>(&'a str);
try_from_regex!(CommentContent<'a>,
                COMMENT_CONTENT_REGEX =>
                (CommentContent, ValidationError::InvalidCommentContent));

impl<'a> Display for CommentContent<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A valid (well formatted) email
///
/// NB This type does **not** implement `Deserialize` because it should only be
/// constructed through `try_into`.
#[derive(Serialize, PartialEq, PartialOrd, Eq, Ord, Debug, Copy, Clone)]
pub struct Email<'a>(&'a str);
try_from_regex!(Email<'a>,
                EMAIL_REGEX =>
                (Email, ValidationError::InvalidEmail));

impl<'a> Display for Email<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
