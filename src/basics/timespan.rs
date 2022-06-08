use std::ops::Neg;
use crate::TimeRange;

/// # A duration value or a timestamp
pub trait TimePoint : Clone+Copy+Eq+Ord+Neg<Output=Self> {

    /// The infinite time point (&infin;)
    const INFINITE: Self;

    /// Checks if this value is finite
    fn is_finite(&self) -> bool;

    /// Checks if this value is +&infin;
    fn is_future_infinite(&self) -> bool;

    /// Checks if this value is -&infin;
    fn is_past_infinite(&self) -> bool;

    /// Returns a value *just after*
    fn just_after(&self) -> Self;

    /// Returns a value *just before*
    fn just_before(&self) -> Self;
}

/// # A set of timepoint
pub trait TimeSpan {
    /// The type of the bounds.
    ///
    /// This is also the type of the element managed by this time window.
    type TimePoint: TimePoint;

    /// Checks if this time window is empty
    fn is_empty(&self) -> bool;

    /// Checks if this time window contains exactly one value
    ///
    /// A singleton is not empty, is convex, is bounded
    /// and its lower bound equals its upper bound.
    fn is_singleton(&self) -> bool;

    /// Checks if this time window is bounded
    ///
    /// It is also false if this time window is empty.
    fn is_bounded(&self) -> bool;

    /// Checks if this time window has a finite lower bound
    ///
    /// It is also false if this time window is empty.
    fn is_low_bounded(&self) -> bool;

    /// Checks if this time window has a finite upper bound
    ///
    /// It is also false if this time window is empty.
    fn is_up_bounded(&self) -> bool;

    /// Checks if this time window is an interval
    fn is_convex(&self) -> bool;

    /// The lower bound of the time window
    ///
    /// It panics if this time window is empty
    fn lower_bound(&self) -> Self::TimePoint;

    /// The upper bound of the time window
    ///
    /// It panics if this time window is empty
    fn upper_bound(&self) -> Self::TimePoint;
}


/// A convex (interval) time set
pub trait TimeConvex: TimeSpan {
    /// Convert to a time range structure
    ///
    /// Panics if the time convex is empty
    fn to_timerange(&self) -> TimeRange<Self::TimePoint>;
}
