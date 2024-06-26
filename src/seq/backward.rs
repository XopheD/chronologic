use std::iter::FusedIterator;
use crate::{TimePoint, Timestamp, Timestamped, TimeValue};
use crate::seq::{TimeSeqBackward, TimeSeqForward, TimeSequence};


impl Timestamped for TimeSeqBackward {

    #[inline] fn timestamp(&self) -> Timestamp { self.t }
}

impl TimeSequence for TimeSeqBackward
{
    fn new<T:Timestamped>(t: T, delta: TimeValue) -> Self
    {
        assert!( delta.is_strictly_positive(), "sequence interval should be strictly positive" );
        let t = t.timestamp() - delta; // so we will start at t
        assert!( t.is_finite(), "infinite start value");
        Self { t, step: delta }
    }

    #[inline]
    fn reverse(self) -> impl TimeSequence {
        TimeSeqForward { t: self.t, step: self.step }
    }

    #[inline]
    fn increment(&self) -> TimeValue { - self.step }

    #[inline]
    fn period(&self) -> TimeValue { self.step }

    fn len(&self) -> usize
    {
        if self.t.0.is_strictly_negative() {
            (self.t.0.as_ticks() - i64::MIN) as usize / self.step.as_ticks() as usize
        } else {
            self.t.0.as_ticks() as usize + i64::MAX as usize + 1 / self.step.as_ticks() as usize
        }
    }
}

impl FusedIterator for TimeSeqBackward { }


impl Iterator for TimeSeqBackward {

    type Item = Timestamp;

    #[inline]
    fn next(&mut self) -> Option<Timestamp>
    {
        self.t -= self.step;
        (!self.t.is_past_infinite()).then_some(self.t)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>)
    {
        let n = self.len();
        (n,Some(n))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    fn last(self) -> Option<Self::Item>
    {
        let n = self.len();
        (n > 0).then_some(self.t - n*self.step)
    }


    fn nth(&mut self, n: usize) -> Option<Self::Item>
    {
        if self.len() < n {
            self.t = -Timestamp::INFINITE;
            None
        } else {
            self.t -= self.step * n;
            Some(self.t)
        }
    }
}