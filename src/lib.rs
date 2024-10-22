mod macros;

#[doc(hidden)]
pub use const_format::{map_ascii_case as __map_ascii_case, Case as __Case};

pub use std::iter::Peekable;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidInteger(String),
    InvalidFloat(String),
    InvalidBool(String),

    UnrecognizedCommand(String),
    UnrecognizedArgument(String),

    OutOfData,
    EmptyShortList,
    MissingArguments(Vec<String>),
    MissingValue(String),

    Other(Box<dyn DebugDisplay>),
}

pub trait DebugDisplay: std::fmt::Debug + std::fmt::Display {}
impl<T: std::fmt::Debug + std::fmt::Display> DebugDisplay for T {}

pub trait Parsable: Sized {
    type Output;
    type FullOutput<T>;

    fn make_full_output<T>(output: Self::Output, rest: T) -> Self::FullOutput<T>;

    fn parse_from<S>(stream: &mut Peekable<S>) -> Result<Self::Output>
    where
        S: Iterator,
        S::Item: AsRef<str> + Into<String>;

    fn parse() -> Result<Self::FullOutput<Vec<String>>> {
        let mut args = std::env::args().skip(1).peekable();
        let out = Self::parse_from(&mut args)?;
        Ok(Self::make_full_output(out, args.collect()))
    }
}

pub trait Value: Sized {
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

impl Parsable for () {
    type Output = ();
    type FullOutput<T> = T;

    fn make_full_output<T>((): (), rest: T) -> Self::FullOutput<T> {
        rest
    }

    fn parse_from<S>(_: &mut Peekable<S>) -> Result<()>
    where
        S: Iterator,
        S::Item: AsRef<str> + Into<String>,
    {
        Ok(())
    }
}

impl Value for String {
    fn parse(data: &str) -> Result<Self> {
        Ok(data.to_string())
    }
}

impl Value for i32 {
    fn parse(data: &str) -> Result<Self> {
        data.parse()
            .map_err(|_| Error::InvalidInteger(data.to_string()))
    }
}

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
    T: Value,
{
    fn parse(data: &str) -> Result<Self> {
        data.split(',').map(|s| T::parse(s.trim())).collect()
    }
}
