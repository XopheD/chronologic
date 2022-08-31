use crate::*;
use crate::wins::*;

/// An optional timepoint could considered as a time window
/// whith is empty when the option variant is `None`
impl<T:TimePoint> TimeBounds for Option<T>
{
    type TimePoint = T;
    #[inline] fn is_empty(&self) -> bool { self.is_none() }
    #[inline] fn is_singleton(&self) -> bool { self.is_some() }
    #[inline] fn is_bounded(&self) -> bool { self.is_singleton() }
    #[inline] fn is_low_bounded(&self) -> bool { self.is_singleton() }
    #[inline] fn is_up_bounded(&self) -> bool { self.is_singleton() }
    #[inline] fn lower_bound(&self) -> Self::TimePoint { self.expect("empty set") }
    #[inline] fn upper_bound(&self) -> Self::TimePoint { self.expect("empty set") }
}

impl<T:TimePoint> TimeConvex for Option<T>
{
}

/// An optional time interval could considered as a time window
/// whith is empty when the option variant is `None`
impl<T:TimePoint> TimeBounds for Option<TimeInterval<T>>
{
    type TimePoint = T;

    #[inline] fn is_empty(&self) -> bool { self.is_none() }

    #[inline]
    fn is_singleton(&self) -> bool {
        self.map(|i| i.is_singleton()).unwrap_or(false)
    }

    #[inline]
    fn is_bounded(&self) -> bool {
        self.map(|i| i.is_bounded()).unwrap_or(false)
    }

    #[inline]
    fn is_low_bounded(&self) -> bool {
        self.map(|i| i.is_low_bounded()).unwrap_or(false)
    }

    #[inline]
    fn is_up_bounded(&self) -> bool {
        self.map(|i| i.is_up_bounded()).unwrap_or(false)
    }

    #[inline]
    fn lower_bound(&self) -> Self::TimePoint {
        self.expect("can’t access lower bound of empty interval").lower_bound()
    }

    #[inline]
    fn upper_bound(&self) -> Self::TimePoint {
        self.expect("can’t access upper bound of empty interval").upper_bound()
    }
}


impl<T:TimePoint> TimeConvex for Option<TimeInterval<T>>
{
}