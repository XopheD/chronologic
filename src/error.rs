//! # Time errors

use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

/// Time calculus error
///
/// Enumeration of different errors which could
/// be encountered in this crate.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TimeError {

    /// An empty interval was met
    ///
    /// It will generated when an empty interval is computed
    /// in a context which refuses it (e.g. when computing
    /// an [`TimeInterval`] which can’t be empty or when
    /// propagating time constraints in a [`TimeGraph`])
    EmptyInterval,

    /// A future (&plus;&infin;) overflow was met
    ///
    /// Typically, this error occurs when the _lower_ bound of
    /// an interval becomes &plus;&infin;, so the interval is undefined
    FutureOverflow,

    /// A past (&minus;&infin;) overflow was met
    ///
    /// Typically, this error occurs when the _upper_ bound of
    /// an interval becomes &minus;&infin;, so the interval is undefined
    PastOverflow,

    /// Undefined value was met during time computation
    ///
    /// Typically, this error occurs when operations are undefined
    /// such as following:
    /// * opposite infinite addition: &plus;&infin; &plus; &minus;&infin;
    /// * multiplying zero with infinite: &pm;&infin; &times; 0
    /// * dividing infinite values: &pm;&infin; &div; &pm;&infin;
    /// * dividing by zero...
    UndefinedValue
}

impl Error for TimeError { }

impl Display for TimeError {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            TimeError::EmptyInterval => "empty time interval (inconsistency)",
            TimeError::FutureOverflow => "overflow in future (+oo)",
            TimeError::PastOverflow => "overflow in past (-oo)",
            TimeError::UndefinedValue => "undefined value",

        })
    }
}

/// A convenient alias for `Result<...,TimeError>`
pub type TimeResult<X> = Result<X,TimeError>;
