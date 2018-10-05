use crate::error::ResponseError;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome as RequestOutcome, Request};
use rocket::Outcome;

pub const USER_TOKEN_NAME: &str = "user_token";

#[derive(Serialize, Deserialize)]
pub struct Token(String);

impl Token {
    pub fn new(token: impl Into<String>) -> Self {
        Token(token.into())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = ResponseError;

    fn from_request(request: &'a Request<'r>) -> RequestOutcome<Self, Self::Error> {
        let cookie = request.cookies().get_private(USER_TOKEN_NAME);

        match cookie {
            // Found user token
            Some(cookie_content) => Outcome::Success(Token::new(cookie_content.value())),
            // Did not found user token
            None => Outcome::Failure((Status::BadRequest, ResponseError::Unauthenticated)),
        }
    }
}
