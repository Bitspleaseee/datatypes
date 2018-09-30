//! Database IDs (a direct referece to a database item)

use super::ValidationError;
use rocket::http::RawStr;
use rocket::request::FromParam;
use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Deref;

/// Any value which is used as an ID should implement this trait
///
/// This trait forces an ID to implement some traits which are relevant for
/// the use of an ID.
pub trait Id: Copy + Eq + Ord + Hash {
    type I: Copy + Eq + Ord + Hash;

    /// Get the underlying representation of the ID
    fn inner(&self) -> Self::I;
}

/// A direct referece to a specific category which is stored in the database
#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[serde(transparent)]
pub struct CategoryId(u32);
id_impls!(CategoryId, CategoryId, u32);

/// A direct referece to a specific thread which is stored in the database
#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[serde(transparent)]
pub struct ThreadId(u32);
id_impls!(ThreadId, ThreadId, u32);

/// A direct referece to a specific comment which is stored in the database
#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[serde(transparent)]
pub struct CommentId(u32);
id_impls!(CommentId, CommentId, u32);

/// A direct referece to a specific user which is store in the database
#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[serde(transparent)]
pub struct UserId(u32);
id_impls!(UserId, UserId, u32);

/// Optional wrapper for any type which implements Id
///
/// The reason for this wrapper is to be able to implement a custom `TryFrom`
/// and `FromParam` for `OptId` which takes into account if the value is empty.
pub struct OptId<I: Id>(Option<I>);

impl<'a, I> TryFrom<&'a str> for OptId<I>
where
    I: Id + TryFrom<&'a str>,
{
    type Error = <I as TryFrom<&'a str>>::Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        if s.is_empty() {
            return Ok(OptId(None));
        }
        s.try_into().map(|id| OptId(Some(id)))
    }
}

impl<'a, I> FromParam<'a> for OptId<I>
where
    I: Id + TryFrom<&'a str>,
    I::Error: Debug,
{
    type Error = <I as TryFrom<&'a str>>::Error;

    fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
        let s: &'a str = param.as_ref();
        s.try_into()
    }
}

impl<I: Id> Deref for OptId<I> {
    type Target = Option<I>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
