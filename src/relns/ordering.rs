use std::cmp::Ordering;
use crate::*;

/// Chronological ordering
///
/// This trait allows comparisons between timestamps
/// of timestamped data.
pub trait TimeOrder<T=Self>
{
    fn time_cmp(&self, other: &T) -> Ordering ;

    #[inline]
    fn is_before(&self, other: &T) -> bool {
        matches!( self.time_cmp(other), Ordering::Less)
    }

    #[inline]
    fn is_simultaneous(&self, other: &T) -> bool {
        matches!( self.time_cmp(other), Ordering::Equal)
    }

    #[inline]
    fn is_after(&self, other: &T) -> bool {
        matches!( self.time_cmp(other), Ordering::Greater)
    }
}

impl<X:Timestamped, T:Timestamped> TimeOrder<T> for X {

    #[inline]
    fn time_cmp(&self, other: &T) -> Ordering {
        self.timestamp().cmp(&other.timestamp())
    }
}
