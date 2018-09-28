/// Takes a simple specification of two structs; one raw and one validated, and
/// generates a `TryFrom` based on if the field is required, optional or
/// doesn't need to be validated.
///
/// There can only be ONE macro innvocation per scope as it generates two
/// modules, `raw` and `valid` which will contain the types.
///
/// # Example usage
///
/// ```rust,ignore
/// raw_to_valid! {
///     #[derive(Serialize, Deserialize, PartialEq, Debug)]
///     pub struct RawAddCategoryPayload<'a>;
///
///     #[derive(Serialize, PartialEq, Debug)]
///     pub struct AddCategoryPayload<'a>;
///
///     impl<'a> TryFrom<RawAddCategoryPayload<'a>> for AddCategoryPayload<'a> {
///         > required
///             title: &'a str => Title<'a>,
///             description: &'a str => Description<'a>
///         > optional
///             subtitle: &'a str => Title<'a>
///     }
/// }
/// ```
#[macro_export]
macro_rules! raw_to_valid {
    (
        $(
        $( #[ $( $raw_atr:meta ),* ] )*
        pub struct $rawi:ident $( < $( $rlif:lifetime ),+ > )*;

        $( #[ $( $valid_atr:meta ),* ] )*
        pub struct $validi:ident $( < $( $vlif:lifetime ),+ > )*;

        impl $( < $( $lif:lifetime ),+ > )* TryFrom<$raw_ty:ty> for $valid_ty:ty {
            $(
                > required
                $( $field:ident: $raw_field_type:ty => $valid_field_type:ty ),*
                $(
                    >-> no-validate
                    $( $no_field:ident: $no_field_type:ty => $no_new_type:ty ),*
                )*
            )*
            $(
                > optional
                $( $opt_field:ident: $opt_raw_field_type:ty => $opt_valid_field_type:ty ),*
                $(
                    >-> no-validate
                    $( $opt_no_field:ident: $opt_no_field_type:ty => $opt_no_new_type:ty ),*
                )*
            )*
        }
        )+
    ) => {
        pub mod raw {
        $(
            $( #[ $( $raw_atr ),* ] )*
            pub struct $rawi $( < $( $lif ),+ > )* {
                $(
                    $( pub $field: $raw_field_type, )*
                    $(
                        $( pub $no_field: $no_field_type, )*
                    )*
                )*
                $(
                    $( pub $opt_field: Option<$opt_raw_field_type>, )*
                    $(
                        $( pub $opt_no_field: Option<$opt_no_field_type>, )*
                    )*
                )*
            }
        )+
        }

        pub mod valid {
            use super::*;
        $(
            $( #[ $( $valid_atr ),* ] )*
            /// NB this type should only be created with `try_into` because all the
            /// fields are automatically validated
            pub struct $validi $( < $( $lif ),+ > )* {
                $(
                    $( $field: $valid_field_type, )*
                    $(
                        $( $no_field: $no_new_type, )*
                    )*
                )*
                $(
                    $( $opt_field: Option<$opt_valid_field_type>, )*
                    $(
                        $( $opt_no_field: Option<$opt_no_new_type>, )*
                    )*
                )*
            }

            use super::raw::$rawi;
            impl $( < $( $lif ),+ > )* std::convert::TryFrom<$raw_ty> for $valid_ty {
                type Error = $crate::valid::ValidationError;
                fn try_from(p: raw::$rawi $( < $( $lif ),+ > )*) -> Result<Self, Self::Error> {
                    #[allow(unused_imports)]
                    use std::convert::TryInto;
                    $(
                        $( let $field: $valid_field_type = p.$field.try_into()?; )*
                        $(
                            $( let $no_field: $no_new_type = p.$no_field.into(); )*
                        )*
                    )*
                    $(
                        $( let $opt_field: Option<$opt_valid_field_type> = p.$opt_field.map(|i| i.try_into()).transpose()?; )*
                        $(
                            $( let $opt_no_field: Option<$opt_no_new_type> = p.$opt_no_field.map(|n| n.into()); )*
                        )*
                    )*
                    Ok(valid::$validi {
                    $(
                        $( $field, )*
                        $(
                            $( $no_field, )*
                        )*
                    )*
                    $(
                        $( $opt_field, )*
                        $(
                            $( $opt_no_field, )*
                        )*
                    )*
                    })
                }
            }
        )+
        }
    }
}

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
