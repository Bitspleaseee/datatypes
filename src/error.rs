//! A collection of all errors

pub type ResponseResult<T> = Result<T, ResponseError>;

#[derive(Fail, Serialize, Deserialize, PartialEq, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum ResponseError {
    #[fail(display = "invalid username")]
    InvalidUsername,
    #[fail(display = "invalid password")]
    InvalidPassword,
    #[fail(display = "user already exists")]
    CannotRegisterExistingUser,
    #[fail(display = "content is missing or hidden")]
    MissingContent,
    #[fail(display = "passed invalid id")]
    InvalidId,
    #[fail(display = "user is not authenticated with the service")]
    Unauthenticated,
    #[fail(display = "user is not authorized to perform action")]
    Unauthorized,
    #[fail(display = "internal server error occured")]
    InternalServerError,
}
