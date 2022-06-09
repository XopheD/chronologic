use std::ops::RangeFull;
use crate::{TimePoint, TimeInterval};

mod range;
mod rangefrom;
mod rangetoincl;
mod rangeto;
mod rangeincl;



// not really useful, just for fun...
impl<T:TimePoint> From<RangeFull> for TimeInterval<T> {
    #[inline] fn from(_: RangeFull) -> Self { TimeInterval::all() }
}



