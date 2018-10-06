//! A collection of all common errors

use crate::admin::responses::AdminError;
use crate::auth::responses::AuthError;
use crate::content::responses::ContentError;

pub type ResponseResult<T> = Result<T, ResponseError>;

#[derive(Fail, Serialize, Deserialize, PartialEq, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum ResponseError {
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
    #[fail(display = "internal server error occured")]
    InternalServerError,
}

impl From<tarpc::Error<ContentError>> for ResponseError {
    fn from(e: tarpc::Error<ContentError>) -> ResponseError {
        let ee: ContentError = e.into();
        match ee {
            ContentError::InternalServerError => ResponseError::InternalServerError,
            eee => ResponseError::ContentRequestError(eee),
        }
    }
}

impl From<tarpc::Error<AuthError>> for ResponseError {
    fn from(e: tarpc::Error<AuthError>) -> ResponseError {
        let ee: AuthError = e.into();
        match ee {
            AuthError::InternalServerError => ResponseError::InternalServerError,
            eee => ResponseError::AuthRequestError(eee),
        }
    }
}

impl From<tarpc::Error<ContentError>> for ContentError {
    fn from(e: tarpc::Error<ContentError>) -> ContentError {
        use tarpc::Error::*;
        match e {
            App(ee) => ee,
            _ => ContentError::InternalServerError,
        }
    }
}

impl From<tarpc::Error<AuthError>> for AuthError {
    fn from(e: tarpc::Error<AuthError>) -> AuthError {
        use tarpc::Error::*;
        match e {
            App(ee) => ee,
            _ => AuthError::InternalServerError,
        }
    }
}

impl From<AuthError> for ResponseError {
    fn from(e: AuthError) -> Self {
        ResponseError::AuthRequestError(e)
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
