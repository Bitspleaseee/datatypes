//! Validated IDs (a direct referece to a database item)

use super::ValidationError;
use std::convert::TryFrom;
use std::hash::Hash;

macro_rules! id_impls {
    ($ty:ty, $exp:expr, $inner:ty) => {
        impl Id for $ty {
            type I = $inner;
            fn inner(&self) -> Self::I {
                self.0
            }
        }
        impl<'a> TryFrom<&'a str> for $ty {
            type Error = ValidationError;
            fn try_from(s: &'a str) -> Result<Self, Self::Error> {
                s.parse::<$inner>()
                    .map($exp)
                    .map_err(|_| ValidationError::InvalidId)
            }
        }
    };
}

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
///
/// NB This type does **not** implement `Deserialize` because it should only be
/// constructed through `try_into`
#[derive(Serialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[serde(transparent)]
pub struct CategoryId(u32);
id_impls!(CategoryId, CategoryId, u32);

/// A direct referece to a specific thread which is stored in the database
///
/// NB This type does **not** implement `Deserialize` because it should only be
/// constructed through `try_into`
#[derive(Serialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[serde(transparent)]
pub struct ThreadId(u32);
id_impls!(ThreadId, ThreadId, u32);

/// A direct referece to a specific comment which is stored in the database
///
/// NB This type does **not** implement `Deserialize` because it should only be
/// constructed through `try_into`
#[derive(Serialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[serde(transparent)]
pub struct CommentId(u32);
id_impls!(CommentId, CommentId, u32);

/// A direct referece to a specific user which is store in the database
///
/// NB This type does **not** implement `Deserialize` because it should only be
/// constructed through `try_into`
#[derive(Serialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[serde(transparent)]
pub struct UserId(u32);
id_impls!(UserId, UserId, u32);
