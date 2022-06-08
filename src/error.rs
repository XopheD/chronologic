//! # Time errors

use std::fmt;
use std::fmt::{Display, Formatter};

/// Time structures generation error
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TimeError {
    EmptyInterval,
    FutureOverflow,
    PastOverflow
}

impl std::error::Error for TimeError { }

impl Display for TimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            TimeError::EmptyInterval => "empty time interval",
            TimeError::FutureOverflow => "overflow in future (+oo)",
            TimeError::PastOverflow => "overflow in past (-oo)",
        })
    }
}
