//! Datatypes with validation
//!
//! These datatypes can be used to compose requests and responses

pub mod fields;
pub mod ids;

// TODO update regexes or change validation to fit our need
// I just threw together some regexs to test out the functionality

/// The regex which vertifies that a username is formatted correctly
const USERNAME_REGEX: &str = "^[a-zA-Z0-9_-]{4,10}$";

/// The regex which vertifies that a password is formatted correctly
const PASSWORD_REGEX: &str = "^[\\w]{8,64}$";

/// The regex which vertifies that a username is formatted correctly
const TITLE_REGEX: &str = "^\\w{4,80}$";

/// The regex which vertifies that a password is formatted correctly
const DESCRIPTION_REGEX: &str = "^\\w{0,255}$";

/// The regex which vertifies that a username is formatted correctly
const COMMENT_CONTENT_REGEX: &str = "^\\w{1,255}$";

/// The regex which vertifies that a password is formatted correctly
const EMAIL_REGEX: &str = "^[A-Za-z0-9._%+-]+@[A-Za-z0-9-]+\\.[A-Za-z]{2,}$";

#[derive(Fail, Serialize, Deserialize, Debug)]
pub enum ValidationError {
    #[fail(display = "invalid (badly formatted) username")]
    InvalidUsername,
    #[fail(display = "invalid (badly formatted) password")]
    InvalidPassword,
    #[fail(display = "invalid (badly formatted) id")]
    InvalidId,
    #[fail(display = "invalid (badly formatted) title")]
    InvalidTitle,
    #[fail(display = "invalid (badly formatted) description")]
    InvalidDescription,
    #[fail(display = "invalid (badly formatted) comment")]
    InvalidCommentContent,
    #[fail(display = "invalid (badly formatted) email")]
    InvalidEmail,
}
