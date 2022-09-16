mod timestamp;
mod timevalue;
mod timeinterval;
mod timeset;
mod format;

pub use timevalue::TimeValue;
pub use timestamp::{Timestamp,Timestamped};
pub use timeinterval::*;
pub use timeset::*;
pub use format::TimeFormat;
use crate::iter::TimeConvexIterator;


use crate::TimePoint;


/// # The envelope (the bounds) of a time window
///
/// A trait which describes the envelope of a time window.
pub trait TimeBounds {

    /// The type of the underlying time data.
    ///
    /// This is also the type of the element managed by a time window.
    /// Typically, the timepoint is [`Timestamp`] when dealing with dates and
    /// [`TimeValue`]  when dealing with durations.
    type TimePoint: TimePoint;

    /// Checks if this time window is empty
    fn is_empty(&self) -> bool;

    /// Checks if this time window contains exactly one value
    ///
    /// A singleton is not empty and its lower bound equals its upper bound.
    #[inline]
    fn is_singleton(&self) -> bool {
        !self.is_empty() && self.lower_bound() == self.upper_bound()
    }

    /// Checks if this time window is bounded
    ///
    /// It returns also `false` if this time window is empty.
    #[inline]
    fn is_bounded(&self) -> bool { self.is_low_bounded() && self.is_up_bounded() }

    /// Checks if this time window has a finite lower bound
    ///
    /// It returns also `false` if this time window is empty.
    fn is_low_bounded(&self) -> bool;

    /// Checks if this time window has a finite upper bound
    ///
    /// It returns also `false` if this time window is empty.
    fn is_up_bounded(&self) -> bool;

    /// The lower bound of the time window
    ///
    /// The behavior is undefined if the time window is empty
    fn lower_bound(&self) -> Self::TimePoint;

    /// The upper bound of the time window
    ///
    /// The behavior is undefined if the time window is empty
    fn upper_bound(&self) -> Self::TimePoint;
}

/// # An arbitrary set of timepoints
///
/// This traits describes the structure of the time window.
pub trait TimeWindow : TimeBounds {

    #[inline]
    fn is_all(&self) -> bool {
        self.is_convex() && !self.is_low_bounded() && !self.is_up_bounded()
    }

    /// Checks if this time window is an interval
    ///
    /// Note that the empty set is convex.
    #[inline] fn is_convex(&self) -> bool { self.convex_count() <= 1 }

    /// The number of convex parts
    ///
    /// An empty set has 0 convex part.
    /// A _non-empty_ interval has exactly 1 convex part.
    fn convex_count(&self) -> usize;

    /// Convex envelope of the time window
    ///
    /// Formally, the convex envelope is the smallest convex which
    /// contains the time window.
    /// In practise, it is the interval defined by the lower and upper
    /// bounds of the time window.
    ///
    /// If the time window is already convex, then the envelope is itself.
    #[inline] fn convex_envelope(&self) -> TimeInterval<Self::TimePoint> {
        TimeInterval { lower: self.lower_bound(), upper: self.upper_bound() }
    }


    type ConvexIter: TimeConvexIterator<TimePoint=Self::TimePoint>;
    fn iter(&self) -> Self::ConvexIter;
}


/// # A marker of convex (interval) time set
///
/// If a time window implements this trait, it is sure
/// that it is a time interval (bounded or not) or an empty set.
///
/// Some computations will be optimized for convex windows.
pub trait TimeConvex : TimeBounds+Sized+Into<TimeInterval<Self::TimePoint>> { }

impl<TW:TimeConvex> TimeWindow for TW {

    /// Checks if the time window is convex or not
    ///
    /// A time window is convex if it is a single time interval
    /// (or if it is empty)
    #[inline] fn is_convex(&self) -> bool { true }

    /// Gets the number of convex parts of the time window
    ///
    /// It returns `0` for empty time windows and `1` for non-empty interval.
    #[inline] fn convex_count(&self) -> usize {
        if self.is_empty() { 0 } else { 1 }
    }

    type ConvexIter = std::option::IntoIter<TimeInterval<Self::TimePoint>>;

    #[inline]
    fn iter(&self) -> Self::ConvexIter
    {
        if self.is_empty() {
            None.into_iter()
        } else {
            Some(TimeInterval {
                lower:self.lower_bound(),
                upper:self.upper_bound()
            }).into_iter()
        }
    }
}
