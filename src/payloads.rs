//! Contains useful template payloads

use std::ops::{Deref, DerefMut};
use crate::valid::ids::UserId;
use rocket::http::Cookie;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

pub const USER_TOKEN_NAME: &str = "user_token";

/// A payload which must be present, but empty
///
/// # Examples
///
/// **For the examples it is assumed that the payload is used in a surrounding
/// type**
///
/// The following JSON object is parsed as a valid request because there is an
/// empty payload.
///
/// ```json
/// {
///     "type": "SOME_TYPE",
///     "payload": {}
/// }
/// ```
///
/// However this JSON object is parsed as a invalid request because `payload`
/// is missing.
///
/// ```json
/// {
///     "type": "SOME_TYPE"
/// }
/// ```
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug)]
pub struct EmptyPayloadStrict {}

/// A payload which can either be present and empty or not present
///
/// # Examples
///
/// **For the examples it is assumed that the payload is used in a surrounding
/// type**
///
/// The following JSON object is parsed as a valid request because there is an
/// empty payload.
///
/// ```json
/// {
///     "type": "SOME_TYPE",
///     "payload": {}
/// }
/// ```
///
/// This JSON object is also parsed as a valid request even though the field is
/// missing.
///
/// ```json
/// {
///     "type": "SOME_TYPE"
/// }
/// ```
pub type EmptyPayload = Option<EmptyPayloadStrict>;

/// Represents a payload that also contains a authorization token
///
/// This payload is generic for both the inner type and the token type, this
/// means that the consumer can provide the types they want. The sole purpose
/// of this structure is to provide a simple way of wrapping an existing type
/// with a token.
///
/// This wrapper is mainly intended for internal use as the token will be
/// stored in the session cookies.
///
/// NB! The type that is wrapped cannot contain a field named `token`
/// (`#[serde(rename="...")]` could be used to circument this)
///
/// # Example usage
///
/// ```
/// # use datatypes::payloads::TokenPayload;
/// # #[macro_use]
/// # extern crate serde_derive;
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// struct UserPayload<'a> {
///     name: &'a str,
///     email: &'a str,
/// }
///
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// struct UserToken(u32);
///
/// type AuthUserPayload<'a> = TokenPayload<UserPayload<'a>, UserToken>;
///
/// fn main() {
///
///     let user_payload = UserPayload {
///         name: "John Doe",
///         email: "john@doe.com"
///     };
///     let token = UserToken(123456789);
///
///     // Make a new authentication payload with a inner type and a token
///     let payload = AuthUserPayload::new(user_payload, token);
///
///     let json = r#"{
///                      "name": "John Doe",
///                      "email": "john@doe.com",
///                      "token": 123456789
///                   }"#;
///
///     let expt: AuthUserPayload = serde_json::from_str(json).unwrap();
///     assert_eq!(expt, payload);
/// }
/// ```
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug)]
pub struct TokenPayload<Inner, Token> {
    token: Token,
    #[serde(flatten)]
    inner: Inner,
}

impl<Inner, Token> TokenPayload<Inner, Token> {
    pub fn new(i: impl Into<Inner>, t: impl Into<Token>) -> TokenPayload<Inner, Token> {
        TokenPayload {
            inner: i.into(),
            token: t.into(),
        }
    }

    /// Get a reference to the token of the payload
    pub fn token(&self) -> &Token {
        &self.token
    }

    /// Set the token of a payload
    pub fn set_token(&mut self, t: impl Into<Token>) -> Token {
        std::mem::replace(&mut self.token, t.into())
    }

    /// Turn the payload into its inner type
    pub fn into_inner(self) -> (Inner, Token) {
        (self.inner, self.token)
    }
}

impl<Inner, Token> Deref for TokenPayload<Inner, Token> {
    type Target = Inner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<Inner, Token> DerefMut for TokenPayload<Inner, Token> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

<<<<<<< HEAD
=======
// Define token.
pub struct Token (String);

impl Token {
    pub fn new(token: String) -> Self {
        Token(token)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = ResponseError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let cookie = request.cookies().get_private(USER_TOKEN_NAME);

        match cookie {
            Some(cookie_content) => {
                // Found a token
                //info!("Getting request with token {:?}", cookie_content);
                Outcome::Success(Token::new(cookie_content.value().to_owned()))
            }
            None => {
                // Did not found any token
                //info!("Did not found any token.");
                Outcome::Failure((Status::BadRequest, ResponseError::Unauthenticated))
            }
        }
    }
}

/// Represents a payload that also contains a user id
///
/// This payload is generic for the inner type, this means that the consumer
/// can provide the type of payload they want. The sole purpose of this
/// structure is to provide a simple way of wrapping an existing payload with a
/// user id.
///
/// This wrapper is mainly intended for internal use to transport a payload
/// with a user id between the auth service to the security gate, and from the
/// security gate to the controller.
///
/// NB! The type that is wrapped cannot contain a field named `user_id`
/// (`#[serde(rename="...")]` could be used to circument this)
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug)]
pub struct UserIdPayload<Inner> {
    id: UserId,
    #[serde(flatten)]
    inner: Inner,
}

impl<Inner> UserIdPayload<Inner> {
    pub fn new(i: impl Into<Inner>, id: impl Into<UserId>) -> UserIdPayload<Inner> {
        UserIdPayload {
            inner: i.into(),
            id: id.into(),
        }
    }

    /// Get a reference to the token of the payload
    pub fn id(&self) -> &UserId {
        &self.id
    }

    /// Set the token of a payload
    pub fn set_id(&mut self, id: impl Into<UserId>) -> UserId {
        std::mem::replace(&mut self.id, id.into())
    }

    /// Turn the payload into its inner type
    pub fn into_inner(self) -> (Inner, UserId) {
        (self.inner, self.id)
    }
}

impl<Inner> Deref for UserIdPayload<Inner> {
    type Target = Inner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<Inner> DerefMut for UserIdPayload<Inner> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}


impl<'a, 'r> FromRequest<'a, 'r> for Token<'a> {
    type Error = ResponseError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let cookie = request.cookies().get_private(USER_TOKEN_NAME);

        match cookie {
            Some(cookie_content) => {
                // Found a token
                //info!("Getting request with token {:?}", cookie_content);
                Outcome::Success(Token::new(cookie_content.value().to_owned()))
            }
            None => {
                // Did not found any token
                //info!("Did not found any token.");
                Outcome::Failure((Status::BadRequest, ResponseError::Unauthenticated))
            }
        }
    }
}
