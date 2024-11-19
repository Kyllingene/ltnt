//! A simple, efficient, and flexible arg parsing library.
//!
//! Most users will simply want [`ltnt!`] and [`Parsable::parse`]; for advanced
//! usage, see [`Parsable`] and [`Error`].
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

mod error;
mod macros;

#[doc(hidden)]
pub use const_format::{map_ascii_case as __map_ascii_case, Case as __Case};

pub use core::iter::Peekable;
pub use error::Error;

/// The result of a parsing operation.
pub type Result<T> = core::result::Result<T, Error>;

/// Returns the path to the current executable.
///
/// Note that this just returns the first argument, which is the executable path
/// only by convention. There's no strict guarantee that this function will
/// return the correct path, or any path at all.
///
/// Only available on feature `std`.
#[cfg(feature = "std")]
pub fn executable() -> String {
    std::env::args()
        .next()
        .expect("first argument should always be the executable path")
}

/// Anything that can be parsed from a stream of arguments.
///
/// If you're just trying to parse CLI arguments, see
/// [`parse`](Parsable::parse).
///
/// Generally, these fall into three categories:
///  - "Command-style" (e.g. `attach-session` in `tmux attach-session`.)
///  - "Argument-style", a set of arguments (e.g. `--help`, `-h`, `-al`)
///  - "Value-style", a value used by another `Parsable` (e.g. `1.23`, `"foo"`)
///
/// ```sh
/// manager dev --kinds keyboard,monitor
/// #       \_/                          A "command-style" Parsable
/// #           \______________________/ An "argument-style" Parsable with..
/// #                   \______________/ a "value-style" Parsable, composed of..
/// #                   \______/ \_____/ two other "value-style" Parsables
/// ```
pub trait Parsable: Sized {
    /// The basic output of parsing this type.
    ///
    /// Usually used to pair arguments with the output, e.g.
    /// `type Output = (Self, SelfArgs);`.
    type Output;

    /// The result when this type is parsed as the root of an invocation.
    ///
    /// Should usually be `(Self::Output, T)`, `(Self::Output, MyArguments, T)`,
    /// or something similar.
    type FullOutput<T>;

    /// Transform the result of [`parse_from`](Parsable::parse_from) into the
    /// result of [`parse_all`](Parsable::parse_all).
    fn make_full_output<T>(output: Self::Output, rest: T) -> Self::FullOutput<T>;

    /// The default value. What this means is dependent on how/when this is
    /// parsed.
    ///
    /// Most things should leave this at it's default implementation (`None`).
    fn default() -> Option<Self> {
        None
    }

    /// Parse this item from a stream.
    ///
    /// Note that this *must not* consume values it doesn't use.
    fn parse_from<S>(stream: &mut Peekable<S>) -> Result<Self::Output>
    where
        S: Iterator,
        S::Item: AsRef<str> + Into<String>;

    /// Parse this item as the root of an invocation.
    ///
    /// Returns any leftovers (i.e. arguments that weren't consumed).
    fn parse_all<A>(args: A) -> Result<Self::FullOutput<Vec<String>>>
    where
        A: IntoIterator,
        A::Item: AsRef<str> + Into<String>,
    {
        let mut args = args.into_iter().skip(1).peekable();
        let out = Self::parse_from(&mut args)?;
        Ok(Self::make_full_output(
            out,
            args.map(|s| s.into()).collect(),
        ))
    }

    /// Parse all the command-line arguments with this as the root.
    ///
    /// Returns any leftovers (i.e. arguments that weren't consumed).
    #[cfg(feature = "std")]
    fn parse() -> Result<Self::FullOutput<Vec<String>>> {
        Self::parse_all(std::env::args())
    }
}

impl<T> Parsable for Box<T>
where
    T: Parsable,
{
    type Output = Box<T::Output>;
    type FullOutput<Q> = (Self::Output, Q);

    fn make_full_output<Q>(output: Self::Output, rest: Q) -> Self::FullOutput<Q> {
        (output, rest)
    }

    #[cfg(feature = "std")]
    fn parse() -> Result<Self::FullOutput<Vec<String>>> {
        let mut args = std::env::args().skip(1).peekable();
        let out = Self::parse_from(&mut args)?;
        Ok(Self::make_full_output(out, args.collect()))
    }

    fn parse_from<S>(stream: &mut Peekable<S>) -> Result<Self::Output>
    where
        S: Iterator,
        S::Item: AsRef<str> + Into<String>,
    {
        T::parse_from(stream).map(Box::new)
    }
}

mod value {
    extern crate alloc;

    use alloc::ffi::CString;
    use alloc::string::{String, ToString};
    use alloc::vec::Vec;

    #[cfg(feature = "std")]
    use std::ffi::OsString;

    use crate::{Error, Parsable, Peekable, Result};

    macro_rules! impl_for {
        ($($t:ident $(<$gen:ident $(: $bound:ident $([$($gen_bound:tt)*])?)?>)?),*) => {$(
            impl $(<$gen $(: $bound $(<$($gen_bound)*>)?)?>)? Parsable for $t $(<$gen>)? {
                type Output = Self;
                type FullOutput<T> = (Self, T);

                fn make_full_output<T>(output: Self::Output, rest: T) -> Self::FullOutput<T> {
                    (output, rest)
                }

                /// The default value. What this means is dependent on how/when this is
                /// parsed.
                ///
                /// Most things should leave this at it's default implementation (`None`).
                fn default() -> Option<Self> {
                    <Self as Value>::default()
                }

                fn parse_from<S>(stream: &mut Peekable<S>) -> Result<Self::Output>
                where
                    S: Iterator,
                    S::Item: AsRef<str> + Into<String>
                {
                    stream.next()
                        .ok_or(Error::OutOfData)
                        .and_then(|s| <Self as Value>::parse(s.as_ref()))
                }
            }
            )*};
    }
    impl_for!(String, f32, f64, bool, CString, Vec<Q: Parsable[Output = Q]>);

    #[cfg(feature = "std")]
    impl_for!(OsString);

    trait Value: Sized {
        /// The default value used when the argument is given, but without a value.
        ///
        /// Defaults to `None`.
        ///
        /// Should rarely be `Some`; the only reason they exists is for boolean
        /// arguments, e.g. `--foo` -> `foo == true`.
        fn default() -> Option<Self> {
            None
        }

        fn parse(data: &str) -> Result<Self>;
    }

    impl Value for String {
        fn parse(data: &str) -> Result<Self> {
            Ok(data.to_string())
        }
    }

    #[cfg(feature = "std")]
    impl Value for OsString {
        fn parse(data: &str) -> Result<Self> {
            Ok(OsString::from(data))
        }
    }

    impl Value for CString {
        fn parse(data: &str) -> Result<Self> {
            CString::new(data).map_err(Error::other)
        }
    }

    macro_rules! impl_for_int {
        (signed = [$($signed:ident),*] unsigned = [$($unsigned:ident),*]) => {
            $(
                impl Value for $signed {
                    fn parse(data: &str) -> Result<Self> {
                        data.parse()
                            .map_err(|_| Error::InvalidSignedInteger(data.to_string()))
                    }
                }
            )*

            $(
                impl Value for $unsigned {
                    fn parse(data: &str) -> Result<Self> {
                        data.parse()
                            .map_err(|_| Error::InvalidUnsignedInteger(data.to_string()))
                    }
                }
            )*

            impl_for!($($signed,)* $($unsigned),*);
        };
    }
    impl_for_int!(
        signed = [i8, i16, i32, i64, i128, isize]
        unsigned = [u8, u16, u32, u64, u128, usize]
    );

    impl Value for f32 {
        fn parse(data: &str) -> Result<Self> {
            data.parse()
                .map_err(|_| Error::InvalidFloat(data.to_string()))
        }
    }

    impl Value for f64 {
        fn parse(data: &str) -> Result<Self> {
            data.parse()
                .map_err(|_| Error::InvalidFloat(data.to_string()))
        }
    }

    impl Value for bool {
        fn default() -> Option<Self> {
            Some(true)
        }

        fn parse(data: &str) -> Result<Self> {
            Ok(match data {
                "0" | "false" => false,
                "1" | "true" => true,
                _ => return Err(Error::InvalidBool(data.to_string())),
            })
        }
    }

    impl<T> Value for Vec<T>
    where
        T: Parsable<Output = T>,
    {
        fn parse(data: &str) -> Result<Self> {
            let mut split = data.split(',').peekable();
            let mut vals = Vec::new();

            while split.peek().is_some() {
                vals.push(T::parse_from(&mut split)?);
            }

            Ok(vals)
        }
    }
}

macro_rules! impl_tuples {
    () => {};
    (@impl) => {};

    (@impl $($i:ident)*) => {
        impl<$($i,)*> Parsable for ($($i,)*)
        where
            $($i: Parsable,)*
        {
            type Output = (
                $(<$i as Parsable>::Output,)*
            );
            type FullOutput<T2> = (Self::Output, T2);

            fn make_full_output<T2>(output: Self::Output, rest: T2) -> Self::FullOutput<T2> {
                (output, rest)
            }

            fn default() -> Option<Self> {
                Some((
                    $(<$i as Parsable>::default()?,)*
                ))
            }

            fn parse_from<S2>(stream: &mut Peekable<S2>) -> Result<Self::Output>
            where
                S2: Iterator,
                S2::Item: AsRef<str> + Into<String>
            {
                Ok((
                    $(<$i as Parsable>::parse_from(stream)?,)*
                ))
            }
        }
    };

    ($head:ident $($tail:ident)*) => {
        impl_tuples!([$head] $($tail)*);
    };

    ([$($curr:ident)*]) => {
        impl_tuples!(@impl $($curr)*);
    };
    ([$($curr:ident)*] $head:ident $($tail:ident)*) => {
        impl_tuples!(@impl $($curr)*);
        impl_tuples!([$($curr)* $head] $($tail)*);
    };
}
impl_tuples!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);
