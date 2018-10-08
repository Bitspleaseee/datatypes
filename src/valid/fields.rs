//! Validated datafields

// TODO add tests which vertifies the `TryFrom` implementations

use super::ValidationError;
use rocket::http::RawStr;
use rocket::request::FromFormValue;
use std::convert::TryFrom;
use std::fmt::{self, Display};
use htmlescape::encode_minimal;

use super::{EMAIL_REGEX, PASSWORD_REGEX, SEARCH_QUERY_REGEX, USERNAME_REGEX};
use regex::Regex;

/// A valid (well formatted) username
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct Username(String);

impl TryFrom<String> for Username {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex = USERNAME_REGEX.parse().expect("invalid username regex");
        }
        if RE.is_match(&s) {
            Ok(Username(s))
        } else {
            Err(ValidationError::InvalidUsername)
        }
    }
}

impl_deserialize_with_try_from!(Username);
impl_serialize!(Username);
impl_deref_and_as_ref!(Username => str);
impl_into_inner!(Username => String);

impl Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A valid (well formatted) plaintext password
///
/// NB This type does not implement `Debug` for the simple reason that a
/// plaintext passwords should **never** be printed.
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone)]
//#[serde(rename = "password")]
pub struct PlainPassword(String);

impl TryFrom<String> for PlainPassword {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex = PASSWORD_REGEX.parse().expect("invalid password regex");
        }
        if RE.is_match(&s)
            && s.chars().any(|c| c.is_ascii_lowercase())
            && s.chars().any(|c| c.is_ascii_uppercase())
            && s.chars().any(|c| c.is_numeric())
        {
            Ok(PlainPassword(s))
        } else {
            Err(ValidationError::InvalidPassword)
        }
    }
}

impl_deserialize_with_try_from!(PlainPassword);
impl_serialize!(PlainPassword);
impl_deref_and_as_ref!(PlainPassword => str);
impl_into_inner!(PlainPassword => String);

/// A valid (well formatted) title
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct Title(String);

impl TryFrom<String> for Title {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        if 4 < s.len() && s.len() < 80 {
            Ok(Title(htmlescape::encode_minimal(&s)))
        } else {
            Err(ValidationError::InvalidTitle)
        }
    }
}

impl_deserialize_with_try_from!(Title);
impl_serialize!(Title);
impl_deref_and_as_ref!(Title => str);
impl_into_inner!(Title => String);

impl Display for Title {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A valid (well formatted) description
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct Description(String);

impl TryFrom<String> for Description {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.len() < 255 {
            Ok(Description(htmlescape::encode_minimal(&s)))
        } else {
            Err(ValidationError::InvalidDescription)
        }
    }
}

impl_deserialize_with_try_from!(Description);
impl_serialize!(Description);
impl_deref_and_as_ref!(Description => str);
impl_into_inner!(Description => String);

impl Display for Description {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A valid (well formatted) comment-content
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct CommentContent(String);

impl TryFrom<String> for CommentContent {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.len() > 4 && s.len() < 80 {
            Ok(CommentContent(htmlescape::encode_minimal(&s)))
        } else {
            Err(ValidationError::InvalidCommentContent)
        }
    }
}

impl_deserialize_with_try_from!(CommentContent);
impl_serialize!(CommentContent);
impl_deref_and_as_ref!(CommentContent => str);
impl_into_inner!(CommentContent => String);

impl Display for CommentContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A valid (well formatted) email
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct Email(String);

impl TryFrom<String> for Email {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex = EMAIL_REGEX.parse().expect("invalid email regex");
        }
        if RE.is_match(&s) {
            Ok(Email(s))
        } else {
            Err(ValidationError::InvalidEmail)
        }
    }
}

impl_deserialize_with_try_from!(Email);
impl_serialize!(Email);
impl_deref_and_as_ref!(Email => str);
impl_into_inner!(Email => String);

impl Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A valid (well formatted) search query string
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
pub struct QueryStr(String);

impl TryFrom<String> for QueryStr {
    type Error = ValidationError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex = SEARCH_QUERY_REGEX.parse().expect("invalid query regex");
        }
        if RE.is_match(&s) {
            Ok(QueryStr(s))
        } else {
            Err(ValidationError::InvalidQuery)
        }
    }
}

impl_deserialize_with_try_from!(QueryStr);
impl_serialize!(QueryStr);
impl_deref_and_as_ref!(QueryStr => str);
impl_into_inner!(QueryStr => String);

impl Display for QueryStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> FromFormValue<'a> for QueryStr {
    type Error = <QueryStr as TryFrom<String>>::Error;
    fn from_form_value(search_str: &'a RawStr) -> Result<Self, Self::Error> {
        let s = search_str.url_decode().unwrap_or("".to_string());          // Decode string and if not working give empty string back.
        QueryStr::try_from(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_escaping {
        ($name:ident, $cons:ident, $strs:expr) => {
            #[test]
            fn $name() {
                for (s, expt) in $strs.into_iter() {
                    let res = $cons::try_from(String::from(s))
                        .expect(&format!("invalid {}", stringify!($cons)));
                    assert_eq!(&*res, expt, "expected '{}' to be equal to {}", s, expt);
                }
            }
        };
    }

    macro_rules! test_input {
        ($name:ident, $cons:ident, $strs:expr, $is_valid:expr) => {
            #[test]
            fn $name() {
                for s in $strs.into_iter() {
                    let res = $cons::try_from(String::from(s));
                    if $is_valid {
                        assert!(res.is_ok(), "expected '{}' to be valid input", s);
                    } else {
                        assert!(res.is_err(), "expected '{}' to be invalid input", s);
                    }
                }
            }
        };
    }

    macro_rules! doesnt_crash {
        ($name:ident, $cons:ident) => {
            proptest! {
                #[test]
                fn $name(s in "\\PC*") {
                    let _ = $cons::try_from(s);
                }
            }
        };
    }

    doesnt_crash!(username_doesnt_crash, Username);
    doesnt_crash!(plain_password_doesnt_crash, PlainPassword);
    doesnt_crash!(title_doesnt_crash, Title);
    doesnt_crash!(description_doesnt_crash, Description);
    doesnt_crash!(comment_content_doesnt_crash, CommentContent);
    doesnt_crash!(email_doesnt_crash, Email);
    doesnt_crash!(query_str_doesnt_crash, QueryStr);

    test_input!(valid_usernames, Username, vec!["john", "irene"], true);
    test_input!(
        valid_search_query,
        QueryStr,
        vec!["john", "irene", "a search with spaces"],
        true
    );
    test_input!(
        valid_emails,
        Email,
        vec![
            "john.theme@example.com",
            "irene@google.no",
            "post@tombarneby.com"
        ],
        true
    );
    test_input!(
        unvalid_emails,
        Email,
        vec!["john.theme.example.com", "irene@google", "tombarneby"],
        false
    );
    test_input!(
        valid_password,
        PlainPassword,
        vec!["helloAndWelcome123", "irene.Welcome1", "pOst@tom.barneby1"],
        true
    );
    test_input!(
        unvalid_password,
        PlainPassword,
        vec!["john", "irenetra", "John"],
        false
    );
    test_input!(
        valid_comments,
        CommentContent,
        vec!["Hello everyone!", "Hello", "Hello, I love you all guys."],
        true
    );
    test_input!(
        valid_title,
        Title,
        vec!["Just a question.", "Hello", "Hello, I love you all guys!"],
        true
    );
    test_input!(
        unvalid_title,
        Title,
        vec!["Just a questionyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy.",
        "Helloccccccccccccccccccccccccccccccccccccccccccccccccccccccvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvs fgfd fffdfffdfdfddfffffff",
        "Hello, I love you all guys! Or what Am fthd fgd rg srsrsd weesdef sdfeeeeeeeeeeeeeeeeeeeeef  esfsefsefds dgf  dfd    dfsdf"],
        false
    );
    test_input!(
        valid_desc,
        Description,
        vec![
            "Questions can be asked here.",
            "Hello",
            "Hello, I love you all guys!"
        ],
        true
    );

    test_escaping!(
        title_with_quotes,
        Title,
        vec![
            (
                "A title with \"quoutes\"",
                "A title with &quot;quoutes&quot;"
            ),
            ("A title with <script>", "A title with &lt;script&gt;")
        ]
    );
    test_escaping!(
        desc_with_quotes,
        Description,
        vec![
            (
                "A description with \"quoutes\"",
                "A description with &quot;quoutes&quot;"
            ),
            (
                "A description with <script>",
                "A description with &lt;script&gt;"
            )
        ]
    );

    test_escaping!(
        comment_with_quotes,
        CommentContent,
        vec![
            (
                "A comment with \"quoutes\"",
                "A comment with &quot;quoutes&quot;"
            ),
            ("A comment with <script>", "A comment with &lt;script&gt;")
        ]
    );
}
