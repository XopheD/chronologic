//! # Time errors

use std::fmt;
use std::fmt::{Display, Formatter};

/// Time calculus error
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TimeError {

    /// An empty interval was met
    ///
    /// It will generated when an empty interval is computed
    /// in a context which refuses it (e.g. when computing
    /// an [`TimeInterval`] which can’t be empty or when
    /// propagating time constraints in a [`TimeGraph`]
    EmptyInterval,

    /// A future (+&infin;) overflow was met
    ///
    /// Typically, this error occurs when the _lower_ bound of
    /// an interval becomes `+&infin;`, so the interval is undefined
    FutureOverflow,

    /// A past (`-oo`) overflow was met
    ///
    /// Typically, this error occurs when the _upper_ bound of
    /// an interval becomes `-oo`, so the interval is undefined
    PastOverflow,

    /// Undefined value was met during time computation
    ///
    /// Typically, this error occurs when operations are undefined
    /// such as following:
    /// * opposite infinite addition: `+oo + -oo`
    /// * multiplying `0` with infinite: `+oo * 0`
    /// * dividing infinite values: `+oo / +oo`
    /// * and so on...
    UndefinedValue
}

impl std::error::Error for TimeError { }

impl Display for TimeError {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            TimeError::EmptyInterval => "empty time interval",
            TimeError::FutureOverflow => "overflow in future (+oo)",
            TimeError::PastOverflow => "overflow in past (-oo)",
            TimeError::UndefinedValue => "undefined value"
        })
    }
}
