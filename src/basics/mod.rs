mod timevalue;
mod timestamp;
mod timeinterval;
mod timeset;

mod translation;
mod scaling;
mod setops;

use std::ops::Neg;

pub use timevalue::*;
pub use timestamp::*;
pub use timeinterval::*;
pub use timeset::*;
pub use translation::*;

use crate::TimeResult;

// Inner value to represent infinite
const INFINITE_TIME_VALUE : i64 = i64::MAX;

const SUBSEC_BITLEN: usize = 30; // more than nanosecond precision
// could be set to 20 for microseconds precision, to 10 for millisecond
// and set to 0 to get only second precision
// (but we kept nanos to be compliant with std::time precision)

// fractional part mask
const SUBSEC_BITMASK: i64 = !((!0) << SUBSEC_BITLEN);

// max of seconds according to fract. part precision
const MAX_SEC: i64 = i64::MAX >> SUBSEC_BITLEN;


/// # A duration value or a timestamp
pub trait TimePoint : Clone+Copy+Eq+Ord+Neg<Output=Self>+TimeConvex<TimePoint=Self> {

    /// The infinite time point (&infin;)
    const INFINITE: Self;

    /// Checks if this value is finite
    fn is_finite(&self) -> bool;

    /// Checks if this value is +&infin;
    fn is_future_infinite(&self) -> bool;

    /// Checks if this value is -&infin;
    fn is_past_infinite(&self) -> bool;

    /// Returns a value *just after* this one
    fn just_after(&self) -> Self;

    /// Returns a value *just before* this one
    fn just_before(&self) -> Self;
}

/// # A set of timepoints
pub trait TimeWindow {
    /// The type of the time bounds.
    ///
    /// This is also the type of the element managed by this time window.
    /// Typically, the timepoint is [`Timestamp`] when dealing with dates and
    /// [`TimeValue`]  when dealing with durations.
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

/// # A trait for convex (interval) time set
///
/// If a time window implements this trait, it is sure
/// that it is a time interval (bounded or not) or an empty set.
/// So some computations could be optimized.
pub trait TimeConvex: TimeWindow {

    /// Compute intersection
    ///
    /// Returns `None` if intersection is empty
    #[inline]
    fn intersection<TW:TimeConvex<TimePoint=Self::TimePoint>>(&self, tw: &TW) -> Option<TimeInterval<Self::TimePoint>> {
        let lower = self.lower_bound().max(tw.lower_bound());
        let upper = self.upper_bound().min(tw.upper_bound());
        if lower > upper { None } else { Some(TimeInterval { lower, upper }) }
    }

    /// Compute convex union
    ///
    /// Never fail
    #[inline]
    fn convex_union<TW:TimeConvex<TimePoint=Self::TimePoint>>(&self, tw: &TW) -> TimeInterval<Self::TimePoint> {
        TimeInterval {
            lower: self.lower_bound().min(tw.lower_bound()),
            upper: self.upper_bound().max(tw.upper_bound())
        }
    }
}


#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use crate::*;

    fn checktw<T:Debug>(check:&str, x:&T) {
        assert_eq!( check, &format!("{:?}", x));
    }


    #[test]
    fn complement()
    {
        let t1 = TimeValue::from_ticks(1);
        let t5 = TimeValue::from_ticks(5);
        let tw = TimeSpan::new(t1, t5).unwrap();
        dbg!(&tw);
        let tw: TimeSpans = !tw;
        dbg!(!tw);
    }

    #[test]
    fn intersection()
    {
        let t1 = TimeValue::from_ticks(1);
        let t5 = TimeValue::from_ticks(5);
        let t10 = TimeValue::from_ticks(10);

        let t10bis = TimeSpan::centered(t10, t5).unwrap();

        let intersection = !t1 & !t5 & !t10bis;
        dbg!(!intersection.clone());
        checktw( "]-oo,0]U[2,4]U[16,+oo[", &intersection);
    }

    #[test]
    fn union()
    {
        let a : TimeInterval<_> = (TimeValue::from_ticks(1)..=TimeValue::from_ticks(10)).try_into().unwrap();
        let b: TimeInterval<_>  = (TimeValue::from_ticks(15)..=TimeValue::from_ticks(18)).try_into().unwrap();
        let c: TimeInterval<_> = (TimeValue::from_ticks(8)..=TimeValue::from_ticks(14)).try_into().unwrap();

        checktw( "[1,18]", &(a|b|c));

        // dbg!((a|b) + c);
    }

    #[test]
    fn translation()
    {
        let t1 = TimeValue::from_ticks(1);
        let t5 = TimeValue::from_ticks(5);
        let tw : TimeSpan = (t1..t5).try_into().unwrap();

        let now = Timestamp::now();
        assert_eq!( tw + now, now + tw);
    }
}
