// #![feature(test)]

//! # Time constraint reasoning
//!
//! This crate is dedicated to reasoning about time.
//! It deals with time constraints, propagate them and
//! maintain an agenda of all the possible dates consistent
//! with the user constraints.
//!
//! ## Time structures
//! Several time structures (interval, sets) are provided
//! to make easier time manipulation.
//!
//! This time data defines several operators for union, intersection,
//! translation in two ways:
//! * by using standard operators (`&` for intersection, `|` for unsion, `+/-` for translation)
//! * by using iterator traits (see module [`iter`]) which allows time manipulation with
//!   saving memory allocation (no intermediate structures needed)
//!
//! ## Time constraint management
//! The module [`graph`] deals with time constraints graph and mainly provides two structures:
//! * [`graph::TimeGraph`]: the time constraints graph, a time constraint is defined as an interval
//!     of duration between two instants, a graph could be considered as a collection of time constraints
//! * [`graph::TimeScheduler`]: the scheduler maintains a set of slots (one for each instant) according to
//!   its time graph
//!
//! Any modification of constraints are automatically propagated (see [`graph::TimeGraph`] for more
//! informations about the propagation algorithm).
//!
mod wins;
pub use wins::*;

pub mod iter;

mod ops;

mod relns;
pub use relns::*;

pub mod graph;
mod seq;


use std::fmt::Debug;
use std::ops::Neg;

// Inner value to represent infinite values (negative or positive)
const INFINITE_TIME_VALUE : i64 = i64::MAX;

const SUBSEC_BITLEN: usize = 30; // more than nanosecond precision
// could be set to 20 for microseconds precision, to 10 for millisecond
// and set to 0 to get only second precision
// (but we kept nanos to be compliant with std::time precision)

// fractional part mask
const SUBSEC_BITMASK: i64 = !((!0) << SUBSEC_BITLEN);

// max of seconds according to fract. part precision
const MAX_SEC: i64 = i64::MAX >> SUBSEC_BITLEN;


/// # A unique point of a time window
///
/// Depending of the implementation, it could be relative to
/// a date (e.g. [`Timestamp`]) or a duration (e.g. [`TimeValue`]).
///
/// All the time data are internally represented by a number of *ticks*.
/// As a consequence, a tick is the most precision that you can get.
///
/// In this crate revision, the precision (i.e. the duration of one tick)
/// is fixed and equals a little bit less that a nanosecond.
/// It is exactly `1/2^30` seconde.
pub trait TimePoint : Debug+Clone+Copy+Eq+Ord+Neg<Output=Self>+Sized {

    /// The infinite time point (&infin;) which
    /// is used to infinite time window bounds
    const INFINITE: Self;

    /// Checks if this value is finite
    fn is_finite(&self) -> bool;

    /// Checks if this value equals +&infin;
    fn is_future_infinite(&self) -> bool;

    /// Checks if this value equals -&infin;
    fn is_past_infinite(&self) -> bool;

    /// Returns a value *just after* this one
    ///
    /// *Just after* means here a point with exactly
    /// one tick more; a tick represents the smallest
    /// duration which could be represented.
    ///
    /// If a time point is infinite (-&infin; or +&infin;),
    /// the *just after* time point does not change and remains infinite.
    fn just_after(&self) -> Self;

    /// Returns a value *just before* this one
    ///
    /// *Just before* means here a point with exactly
    /// one tick less; a tick represents the smallest
    /// duration which could be represented.
    ///
    /// If a time point is infinite (-&infin; or +&infin;),
    /// the *just before* time point does not change and remains infinite.
    fn just_before(&self) -> Self;
}