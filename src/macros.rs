/// A simple macro to make implementations of `TryFrom` easier to user for
/// either closures or regexes
///
/// Use a '@' infront of a regex and the closure will recieve the `&'a str` as
/// the first and only argument
#[macro_export]
macro_rules! impl_try_from {
    (
        impl<'a $(,)* $( $lif:lifetime ),* > TryFrom<&'a str> for $ty:ty {
           $cls:expr => $cons:ident | $err:expr
        }
    ) => {
        impl<'a, $( $lif ),* > TryFrom<&'a str> for $ty {
            type Error = crate::valid::ValidationError;
            fn try_from(s: &'a str) -> Result<Self, Self::Error> {
                if ($cls)(s) {
                    Ok($cons(s))
                } else {
                    Err($err)
                }
            }
        }
    };
    (
        impl<'a $(,)* $( $lif:lifetime ),* > TryFrom<&'a str> for $ty:ty {
            @$regex:expr => $cons:ident | $err:expr
        }
    ) => {
        impl<'a, $( $lif ),* > TryFrom<&'a str> for $ty {
            type Error = crate::valid::ValidationError;
            fn try_from(s: &'a str) -> Result<Self, Self::Error> {
                lazy_static! {
                    static ref RE: Regex =
                        Regex::new($regex).expect(&format!("regex '{}' is invalid", $regex));
                }
                if RE.is_match(s) {
                    Ok($cons(s))
                } else {
                    Err($err)
                }
            }
        }
    }

}

/// Generates `TryFrom<&'a str>` and `Id` impls for ids
#[macro_export]
macro_rules! id_impls {
    ($ty:ty, $exp:expr, $inner:ty) => {
        impl Id for $ty {
            type I = $inner;
            fn inner(&self) -> Self::I {
                self.0
            }
        }
        impl<'a> TryFrom<&'a str> for $ty {
            type Error = ValidationError;
            fn try_from(s: &'a str) -> Result<Self, Self::Error> {
                s.parse::<$inner>()
                    .map($exp)
                    .map_err(|_| ValidationError::InvalidId)
            }
        }
        impl From<$inner> for $ty {
            fn from(n: $inner) -> Self {
                $exp(n)
            }
        }
    };
}
