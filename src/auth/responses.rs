//! The responses a user will get from requests to the auth-service

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum AuthSuccess {
    Authenticated,
    Deauthenticated,
    UserRegistered,
}

#[derive(Fail, Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum AuthError {
    #[fail(display = "invalid token")]
    InvalidToken,
    #[fail(display = "invalid username")]
    InvalidUsername,
    #[fail(display = "invalid password")]
    InvalidPassword,
    #[fail(display = "user already exists")]
    ExistingUser,
    #[fail(display = "internal server error")]
    InternalServerError,
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub enum Role {
    Admin = 30,
    Moderator = 20,
    User = 10,
}

impl<'a> From<&'a str> for Role {
    fn from(s: &'a str) -> Self {
        match s {
            "admin" => Role::Admin,
            "moderator" => Role::Moderator,
            "user" => Role::User,
            _ => Role::User,
        }
    }
}
