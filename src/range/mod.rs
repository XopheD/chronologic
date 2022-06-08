use std::ops::RangeFull;
use crate::{TimePoint, TimeRange};

mod range;
mod rangefrom;
mod rangetoincl;
mod rangeto;
mod rangeincl;



// not really useful, just for fun...
impl<T:TimePoint> From<RangeFull> for TimeRange<T> {
    #[inline] fn from(_: RangeFull) -> Self { TimeRange::all() }
}



