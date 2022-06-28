use std::fmt;
use std::ops::{Neg, Not};
use crate::*;
use crate::error::TimeError;

/// A union of [`TimeSpan`] (aliased to [`TimeSet<TimeValue>`])
pub type TimeSpans = TimeSet<TimeValue>;

/// A union of [`TimeSlot`] (aliased to [`TimeSet<Timestamp>`])
pub type TimeSlots = TimeSet<Timestamp>;

/// # A union of time intervals
///
/// This is the more generic structure to keep a set of time points.
/// It could be empty, convex or defined by pieces.
///
/// The inner list of time intervals is chronological sorted
/// and all the inner intervals are disjoint. If, when added,
/// two intervals overlaps, then they are merged.
#[derive(Clone, Eq, PartialEq, Hash, Default)]
pub struct TimeSet<T:TimePoint>(pub(crate) Vec<TimeInterval<T>>);

impl<T:TimePoint> TimeSet<T>
{
    /// The full interval `]-oo,+oo[`
    ///
    /// Returns a timeset composed of the full interval `]-oo,+oo[`
    #[inline]
    pub fn all() -> Self { Self(vec![TimeInterval::all()]) }

    /// A convex interval `[a,b]`
    ///
    /// Returns a timeset composed of one convex interval.
    #[inline]
    pub fn convex(lower: T, upper: T) -> Result<Self,TimeError>
    {
        match TimeInterval::new(lower, upper) {
            Ok(tw) => { Ok(Self(vec![tw])) }
            Err(TimeError::EmptyInterval) => { Ok(Self::empty()) }
            Err(err) => Err(err)
        }
    }

    /// A singleton `{t}`
    ///
    /// Retuns a timeset composed of the convex interval `[t,t]`
    #[inline]
    pub fn singleton(t: T) -> Result<Self,TimeError> {
        TimeInterval::singleton(t).map(|tw| Self(vec![tw]))
    }

    /// The empty set
    #[inline]
    pub fn empty() -> Self { Self(vec![]) }

    /// Inner intervals ordered access
    ///
    /// This method gives an access to the inner intervals describing this set.
    /// The intervals are all distints, without any overlapping nor meeting
    /// (i.e. each pair of intervals are separated by, at least, one time point).
    #[inline]
    pub fn as_slice(&self) -> &[TimeInterval<T>] { self.0.as_slice() }

}


impl<T:TimePoint> TimeWindow for TimeSet<T>
{
    type TimePoint = T;

    #[inline]
    fn is_empty(&self) -> bool { self.0.is_empty() }

    #[inline]
    fn is_singleton(&self) -> bool {
        self.is_convex() && unsafe { self.0.get_unchecked(0).is_singleton() }
    }

    #[inline]
    fn is_bounded(&self) -> bool {
        self.is_low_bounded() && self.is_up_bounded()
    }

    #[inline]
    fn is_low_bounded(&self) -> bool {
        self.0.first().map(|s| s.is_low_bounded()).unwrap_or(false)
    }

    #[inline]
    fn is_up_bounded(&self) -> bool {
        self.0.last().map(|s| s.is_up_bounded()).unwrap_or(false)
    }

    #[inline]
    fn is_convex(&self) -> bool { self.0.len() == 1 }

    #[inline]
    fn lower_bound(&self) -> Self::TimePoint {
        self.0.first().expect("empty interval").lower_bound()
    }

    #[inline]
    fn upper_bound(&self) -> Self::TimePoint {
        self.0.last().expect("empty interval").upper_bound()
    }
}

impl<T:TimePoint+TimeTranslation> TimeTranslation for TimeSet<T>
{
    fn translate(&self, t: TimeValue) -> TimeResult<Self>
    {
        // adding a constant preserves the structure (order and distance
        // between successive intervals -> nothing to manage)
        self.0.iter()
            .map(|i| i.translate(t))
            .collect::<TimeResult<Vec<_>>>()
            .map(|v| Self(v))
    }
}


impl<T:TimePoint> Neg for TimeSet<T>
{
    type Output = Self;
    #[inline] fn neg(self) -> Self {
        // negate each intervals AND reverse the list
        Self(self.0.iter().rev().map(|&t| -t).collect())
    }
}

impl<T:TimePoint> IntoIterator for TimeSet<T>
{
    type Item = TimeInterval<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T,TW> FromIterator<TW> for TimeSet<T>
    where
        T:TimePoint,
        TW:TimeWindow<TimePoint=T>+Not<Output=Self>
{
    fn from_iter<I: IntoIterator<Item=TW>>(iter: I) -> Self {
        iter.into_iter().fold(Self::empty(), |r,i| r|i)
    }
}

impl<I,T> From<I> for TimeSet<T>
    where
        T:TimePoint,
        I:Into<TimeInterval<T>>
{
    #[inline] fn from(tw: I) -> Self { Self( vec![tw.into()]) }
}


impl<T:TimePoint+fmt::Debug> fmt::Debug for TimeSet<T>
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    {
        let mut iter = self.0.iter();
        if let Some(first) = iter.next() {
            write!(formatter, "{:?}", first)?;
            iter.try_for_each(|tw| write!(formatter, "U{:?}", tw))
        } else {
            write!(formatter, "{{}}") /* empty set */
        }
    }
}

impl<T:TimePoint+fmt::Display> fmt::Display for TimeSet<T>
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    {
        let mut iter = self.0.iter();
        if let Some(first) = iter.next() {
            write!(formatter, "{}", first)?;
            iter.try_for_each(|tw| write!(formatter, "U{}", tw))
        } else {
            write!(formatter, "{{}}") /* empty set */
        }
    }
}
