use core::fmt;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::boxed::Box;

pub trait DebugDisplay: fmt::Debug + fmt::Display {}
impl<T: fmt::Debug + fmt::Display> DebugDisplay for T {}

#[derive(Debug)]
pub enum Error {
    InvalidSignedInteger(String),
    InvalidUnsignedInteger(String),
    InvalidFloat(String),
    InvalidBool(String),
    InvalidValue(String),

    UnrecognizedCommand(String),
    UnrecognizedArgument(String),

    OutOfData,
    EmptyShortList,
    MissingArguments(Vec<String>),
    MissingValue(String),

    Other(Box<dyn DebugDisplay>),
}

impl Error {
    pub fn other<E: DebugDisplay + 'static>(err: E) -> Self {
        Self::Other(Box::new(err))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSignedInteger(s) => write!(f, "invalid signed integer: {s}"),
            Self::InvalidUnsignedInteger(s) => write!(f, "invalid unsigned integer: {s}"),
            Self::InvalidFloat(s) => write!(f, "invalid float: {s}"),
            Self::InvalidBool(s) => write!(f, "invalid boolean: {s}"),
            Self::InvalidValue(s) => write!(f, "invalid value: {s}"),

            Self::UnrecognizedCommand(c) => write!(f, "unrecognized command: {c}"),
            Self::UnrecognizedArgument(a) => write!(f, "unrecognized argument: {a}"),

            Self::OutOfData => write!(f, "expected more arguments"),
            Self::EmptyShortList => write!(f, "found lone `-` without short args"),
            Self::MissingArguments(args) => {
                write!(f, "missing arguments: ")?;

                let first = args.first().ok_or(fmt::Error)?; // should never error, but...
                write!(f, "{first}")?;

                for arg in &args[1..] {
                    write!(f, ", {arg}")?;
                }

                Ok(())
            }
            Self::MissingValue(n) => write!(f, "argument {n} requires a value, found none"),

            Self::Other(e) => write!(f, "{e}"),
        }
    }
}
