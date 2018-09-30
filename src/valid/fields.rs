//! Validated datafields

// TODO add tests which vertifies the `TryFrom` implementations

use super::ValidationError::*;
use serde::de::{self, Deserialize, Deserializer};
use std::convert::TryFrom;
use std::fmt::{self, Display};

use super::{EMAIL_REGEX, PASSWORD_REGEX, USERNAME_REGEX};
use regex::Regex;

/// A valid (well formatted) username
#[derive(Serialize, PartialEq, PartialOrd, Eq, Ord, Debug, Copy, Clone)]
pub struct Username<'a>(&'a str);

impl_try_from! {
    impl<'a> TryFrom<&'a str> for Username<'a> {
        @USERNAME_REGEX => Username | InvalidUsername
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for Username<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&'de str as Deserialize>::deserialize(deserializer)?;
        Username::try_from(s).map_err(de::Error::custom)
    }
}

impl<'a> Display for Username<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A valid (well formatted) plaintext password
///
/// NB This type does not implement `Debug` for the simple reason that a
/// plaintext passwords should **never** be printed.
#[derive(Serialize, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
#[serde(rename = "password")]
pub struct PlainPassword<'a>(&'a str);

impl_try_from! {
    impl<'a> TryFrom<&'a str> for PlainPassword<'a> {
        @PASSWORD_REGEX => PlainPassword | InvalidPassword
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for PlainPassword<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&'de str as Deserialize>::deserialize(deserializer)?;
        PlainPassword::try_from(s).map_err(de::Error::custom)
    }
}

/// A valid (well formatted) title
///
/// NB This type does **not** implement `Deserialize` because it should only be
/// constructed through `try_into`.
#[derive(Serialize, PartialEq, PartialOrd, Eq, Ord, Debug, Copy, Clone)]
pub struct Title<'a>(&'a str);

impl_try_from! {
    impl<'a> TryFrom<&'a str> for Title<'a> {
        |s: &'a str| s.len() > 4 && s.len() < 80 => Title | InvalidTitle
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for Title<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&'de str as Deserialize>::deserialize(deserializer)?;
        Title::try_from(s).map_err(de::Error::custom)
    }
}

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

impl_try_from! {
    impl<'a> TryFrom<&'a str> for Description<'a> {
        |s: &'a str| s.len() > 4 && s.len() < 80 => Description | InvalidDescription
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for Description<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&'de str as Deserialize>::deserialize(deserializer)?;
        Description::try_from(s).map_err(de::Error::custom)
    }
}
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

impl_try_from! {
    impl<'a> TryFrom<&'a str> for CommentContent<'a> {
        |s: &'a str| s.len() > 4 && s.len() < 80 => CommentContent | InvalidCommentContent
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for CommentContent<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&'de str as Deserialize>::deserialize(deserializer)?;
        CommentContent::try_from(s).map_err(de::Error::custom)
    }
}

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

impl_try_from! {
    impl<'a> TryFrom<&'a str> for Email<'a> {
        @EMAIL_REGEX => Email | InvalidEmail
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for Email<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&'de str as Deserialize>::deserialize(deserializer)?;
        Email::try_from(s).map_err(de::Error::custom)
    }
}

impl<'a> Display for Email<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
