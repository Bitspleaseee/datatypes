use crate::auth::responses::AuthError;
use crate::content::responses::ContentRequestError;

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
    ContentRequestError(ContentRequestError),
}

impl From<AuthError> for ResponseError {
    fn from(e: AuthError) -> Self {
        ResponseError::AuthenticationError(e)
    }
}

impl From<ContentRequestError> for ResponseError {
    fn from(e: ContentRequestError) -> Self {
        ResponseError::ContentRequestError(e)
    }
}
