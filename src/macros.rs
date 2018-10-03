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

/// Generates `TryFrom<&'a str>`, `FromParam`, `From<Inner>` and `Id` impls for ids
#[macro_export]
macro_rules! id_impls {
    ($ty:ty, $exp:expr, $inner:ty) => {
        impl Id for $ty {
            type I = $inner;
            fn inner(&self) -> Self::I {
                self.0
            }
        }
        impl TryFrom<&str> for $ty {
            type Error = ValidationError;
            fn try_from(s: &str) -> Result<Self, Self::Error> {
                s.parse::<$inner>()
                    .map($exp)
                    .map_err(|_| ValidationError::InvalidId)
            }
        }
        impl<'a> FromParam<'a> for $ty {
            type Error = ValidationError;
            fn from_param(s: &'a RawStr) -> Result<Self, Self::Error> {
                let s: &'a str = s.as_ref();
                s.try_into()
            }
        }
        impl From<$inner> for $ty {
            fn from(n: $inner) -> Self {
                $exp(n)
            }
        }
        impl Deref for $ty {
            type Target = $inner;
            fn deref(&self) -> &$inner {
                &self.0
            }
        }
    };
}

#[macro_export]
macro_rules! impl_deref_and_as_ref {
    ($ty:ty, $inner:ty) => {
        impl Deref for $ty {
            type Target = $inner;
            fn deref(&self) -> &$inner {
                &self.0
            }
        }

        impl AsRef<$inner> for $ty {
            fn as_ref(&self) -> &$inner {
                &self.0
            }
        }
    };
}

#[macro_export]
macro_rules! impl_get_string {
    ($ty:ty) => {
        impl $ty {
            pub fn get_string(&self) -> &String {
                &self.0
            }
        }
    };
}
