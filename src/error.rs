use crate::auth::responses::AuthError;
use crate::admin::responses::AdminError;
use crate::content::responses::ContentError;

#[derive(Fail, Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum ResponseError {
    #[fail(display = "authentication request error")]
    AuthenticationError(AuthError),
    #[fail(display = "content request error")]
    ContentRequestError(ContentError),
    #[fail(display = "admin request error")]
    AdminRequestError(AdminError),
}

impl From<AuthError> for ResponseError {
    fn from(e: AuthError) -> Self {
        ResponseError::AuthenticationError(e)
    }
}

impl From<ContentError> for ResponseError {
    fn from(e: ContentError) -> Self {
        ResponseError::ContentRequestError(e)
    }
}

impl From<AdminError> for ResponseError {
    fn from(e: AdminError) -> Self {
        ResponseError::AdminRequestError(e)
    }
}

