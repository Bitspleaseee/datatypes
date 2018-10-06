//! Database IDs (a direct referece to a database item)

use std::hash::Hash;

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
id_impls!(CategoryId, CategoryId => u32);

/// A direct referece to a specific thread which is stored in the database
#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[serde(transparent)]
pub struct ThreadId(u32);
id_impls!(ThreadId, ThreadId => u32);

/// A direct referece to a specific comment which is stored in the database
#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[serde(transparent)]
pub struct CommentId(u32);
id_impls!(CommentId, CommentId => u32);

/// A direct referece to a specific user which is store in the database
#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[serde(transparent)]
pub struct UserId(u32);
id_impls!(UserId, UserId => u32);
