/// A convenience macro to implement deserialize for a item which validates the
/// contents of the string with `TryInto`
#[macro_export]
macro_rules! impl_deserialize_with_try_from {
    ($ident:ident) => {
        impl<'de> serde::de::Deserialize<'de> for $ident {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                use serde::de::Deserialize;
                let s = String::deserialize(deserializer)?;
                $ident::try_from(s).map_err(serde::de::Error::custom)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_serialize {
    ($ident:ident) => {
        impl serde::Serialize for $ident {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let encoded = encode_minimal(self.as_ref());
                serializer.serialize_str(&encoded)
            }
        }
    };
}

/// Generates relevant impls for ids
#[macro_export]
macro_rules! id_impls {
    ($outer_ty:ty, $outer_cons:expr => $inner:ty) => {
        impl Id for $outer_ty {
            type I = $inner;
            fn inner(&self) -> Self::I {
                self.0
            }
        }
        impl std::convert::TryFrom<&str> for $outer_ty {
            type Error = crate::valid::ValidationError;
            fn try_from(s: &str) -> Result<Self, Self::Error> {
                s.parse::<$inner>()
                    .map($outer_cons)
                    .map_err(|_| crate::valid::ValidationError::InvalidId)
            }
        }
        impl<'a> rocket::request::FromParam<'a> for $outer_ty {
            type Error = crate::valid::ValidationError;
            fn from_param(s: &'a rocket::http::RawStr) -> Result<Self, Self::Error> {
                use std::convert::TryInto;
                <rocket::http::RawStr as std::convert::AsRef<str>>::as_ref(s).try_into()
            }
        }
        impl std::convert::From<$inner> for $outer_ty {
            fn from(n: $inner) -> Self {
                $outer_cons(n)
            }
        }

        impl_deref_and_as_ref!($outer_ty => $inner);

        impl std::fmt::Display for $outer_ty {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_into_inner {
    ($outer:ty => $inner:ty) => {
        impl $outer {
            #[allow(unused)]
            pub fn into_inner(self) -> $inner {
                self.0
            }
        }
    };
}

#[macro_export]
macro_rules! impl_deref_and_as_ref {
    ($outer:ty => $inner:ty) => {
        impl std::ops::Deref for $outer {
            type Target = $inner;
            fn deref(&self) -> &$inner {
                &self.0
            }
        }
        impl std::convert::AsRef<$inner> for $outer {
            fn as_ref(&self) -> &$inner {
                &self.0
            }
        }
    };
}
