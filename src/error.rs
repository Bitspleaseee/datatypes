//! A collection of all common errors

use crate::admin::responses::AdminError;
use crate::auth::responses::AuthError;
use crate::content::responses::ContentError;

#[derive(Fail, Serialize, Deserialize, PartialEq, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum Error {
    #[fail(display = "error specific to auth requests")]
    AuthRequestError(#[cause] AuthError),
    #[fail(display = "error specific to content requests")]
    ContentRequestError(#[cause] ContentError),
    #[fail(display = "error specific to admin requests")]
    AdminRequestError(#[cause] AdminError),
    #[fail(display = "user is not authenticated with the service")]
    Unauthenticated,
    #[fail(display = "user is not authorized to perform action")]
    Unauthorized,
}

impl From<AuthError> for Error {
    fn from(e: AuthError) -> Self {
        Error::AuthRequestError(e)
    }
}

impl From<ContentError> for Error {
    fn from(e: ContentError) -> Self {
        Error::ContentRequestError(e)
    }
}

impl From<AdminError> for Error {
    fn from(e: AdminError) -> Self {
        Error::AdminRequestError(e)
    }
}
