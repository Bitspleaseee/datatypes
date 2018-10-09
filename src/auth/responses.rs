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
    #[fail(display = "username or password is invalid")]
    InvalidCredentials,
    #[fail(display = "user already exists")]
    ExistingUser,
    #[fail(display = "internal server error")]
    InternalServerError,
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug)]
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

impl Into<String> for Role {
    fn into(self) -> String {
        match self {
            Role::Admin => "admin".to_owned(),
            Role::Moderator => "moderator".to_owned(),
            Role::User => "user".to_owned(),
        }
    }
}

impl<'de> serde::de::Deserialize<'de> for Role {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        use serde::de::Deserialize;
        let s = String::deserialize(deserializer)?;
        Ok(Role::from(s.as_ref()))
    }
}
impl serde::Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let string: String = Role::into(*self);
        serializer.serialize_str(&string)
    }
}