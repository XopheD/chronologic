mod forward;
mod backward;

use crate::{Timestamp, Timestamped, TimeValue};


#[derive(Copy, Clone)]
pub struct TimeSeqForward {
    t: Timestamp, // last emitted
    step: TimeValue
}

#[derive(Copy, Clone)]
pub struct TimeSeqBackward {
    t: Timestamp, // last emitted
    step: TimeValue
}

pub trait TimeSequence: Timestamped
{
    fn new<T:Timestamped>(t: T, delta: TimeValue) -> Self;
    fn reverse(self) -> impl TimeSequence;
    fn increment(&self) -> TimeValue;
    fn period(&self) -> TimeValue;
    fn len(&self) -> usize;
}

impl Timestamp {

    pub fn forward_sequence(self, period: TimeValue) -> TimeSeqForward {
        TimeSeqForward::new(self, period)
    }

    pub fn forward_periods(self, period: TimeValue) -> TimeSeqForward {
        TimeSeqForward::new(self.ceil(period), period)
    }

    pub fn backward_sequence(self, period: TimeValue) -> TimeSeqBackward {
        TimeSeqBackward::new(self, period)
    }

    pub fn backward_periods(self, period: TimeValue) -> TimeSeqBackward {
        TimeSeqBackward::new(self.ceil(period), period)
    }
}


