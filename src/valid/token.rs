use std::convert::{AsRef, From};
use std::borrow::Cow;
use crate::error::ResponseError;
use rocket::http::{Status, Cookie};
use rocket::request::{FromRequest, Outcome as RequestOutcome, Request};
use rocket::Outcome;

pub const USER_TOKEN_NAME: &str = "user_token";

#[derive(Serialize, Deserialize)]
pub struct Token<'a>(Cow<'a, str>);

impl<'a> Token<'a> {
    pub fn new(token: impl Into<Cow<'a, str>>) -> Self {
        Token(token.into())
    }
}

impl AsRef<str> for Token<'_> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'a> From<&'a Cookie<'a>> for Token<'a> {
    fn from(c: &'a Cookie<'a>) -> Self {
        Token::new(c.value())
    }
}

impl<'a> From<Cookie<'a>> for Token<'a> {
    fn from(c: Cookie<'a>) -> Self {
        Token::new(c.value().to_owned())
    }
}

impl<'a> Into<Cookie<'a>> for Token<'a> {
    fn into(self) -> Cookie<'a> {
        Cookie::new(USER_TOKEN_NAME, self.0.into_owned())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Token<'a> {
    type Error = ResponseError;

    fn from_request(req: &'a Request<'r>) -> RequestOutcome<Self, Self::Error> {
        req.cookies().get_private(USER_TOKEN_NAME)
            .map(|cookie|
                 Outcome::Success(cookie.into())
            )
            .unwrap_or_else(||
                Outcome::Failure((Status::BadRequest, ResponseError::Unauthenticated))
            )
    }
}
