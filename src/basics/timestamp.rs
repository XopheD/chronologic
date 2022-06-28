use std::ops::Neg;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use std::fmt;
use std::time::SystemTime;

use crate::*;

/// # A UTC timestamp (date + time)
#[derive(Copy, Clone, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Timestamp(pub(crate) TimeValue);

/// A trait for marking timestamped data
pub trait Timestamped {
    /// Gets the timestamp
    fn timestamp(&self) -> Timestamp;
}

impl fmt::Debug for Timestamp
{
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "t={:?}", self.0)
    }
}

impl fmt::Display for Timestamp
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    {
        if self.0.is_positive() {
            if self.is_future_infinite() {
                write!(formatter, "+oo")
            } else {
                write!(formatter, "{}", self.to_datetime())
            }
        } else {
            if self.is_past_infinite() {
                write!(formatter, "-oo")
            } else {
                write!(formatter, "1970-01-01 00:00:00 UTC - {:?}", -self.0)
            }
        }
    }
}

impl Timestamp {

    /// Creates a timepoint relative to the origin
    ///
    /// Equivalent to `Timestamp::ORIGIN + t`
    #[inline]
    pub fn from_origin(t: TimeValue) -> Self { Self(t) }

    #[inline]
    pub fn now() -> Self {
        Self(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().into())
    }

    #[inline]
    pub fn to_datetime(&self) -> DateTime<Utc>
    {
        DateTime::<Utc>::from_utc((*self).into(), Utc)
    }

    #[inline]
    pub fn floor(self, period:TimeValue) -> Self
    {
        Self(self.0.floor(period))
    }

    #[inline]
    pub fn ceil(self, period:TimeValue) -> Self
    {
        Self(self.0.ceil(period))
    }


    /// Duration since origin
    ///
    /// Equivalent to `self - Timestamp::ORIGIN`
    #[inline]
    pub fn since_origin(self) -> TimeValue { self.0 }
}


impl Neg for Timestamp {
    type Output = Self;
    #[inline] fn neg(self) -> Self::Output { Self(-self.0) }
}

impl TimePoint for Timestamp
{
    const INFINITE: Self = Self(TimeValue::INFINITE);
    #[inline] fn is_finite(&self) -> bool { self.0.is_finite() }
    #[inline] fn is_future_infinite(&self) -> bool { self.0.is_future_infinite() }
    #[inline] fn is_past_infinite(&self) -> bool { self.0.is_past_infinite() }
    #[inline] fn just_after(&self) -> Self { Self(self.0.just_after()) }
    #[inline] fn just_before(&self) -> Self { Self(self.0.just_before()) }
}


impl Timestamped for Timestamp
{
    #[inline]
    fn timestamp(&self) -> Timestamp { *self }
}

impl TimeTranslation for Timestamp
{
    #[inline]
    fn translate(&self, t: TimeValue) -> TimeResult<Self> {
        self.0.translate(t).map(|t| Self(t))
    }
}

impl Into<NaiveDateTime> for Timestamp
{
    #[inline]
    fn into(self) -> NaiveDateTime {
        NaiveDateTime::from_timestamp( self.0.as_secs(), self.0.subsec_nanos() as u32)
    }
}

impl From<NaiveDateTime> for Timestamp
{
    #[inline]
    fn from(t: NaiveDateTime) -> Self {
        Self(TimeValue::from_nanos(t.timestamp_nanos()))
    }
}

impl<Tz:TimeZone> From<DateTime<Tz>> for Timestamp
{
    #[inline]
    fn from(t: DateTime<Tz>) -> Self {
        Self(TimeValue::from_nanos(t.timestamp_nanos()))
    }
}

