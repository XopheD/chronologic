use std::ops::{RangeFrom, Add, Sub, AddAssign, SubAssign};
use crate::*;
use crate::error::TimeError;

impl<T:TimePoint> TimeSpan for RangeFrom<T>
{
    type TimePoint = T;
    #[inline] fn is_empty(&self) -> bool { self.lower_bound().is_future_infinite() }
    #[inline] fn is_singleton(&self) -> bool { false }
    #[inline] fn is_bounded(&self) -> bool { false }
    #[inline] fn is_low_bounded(&self) -> bool { self.lower_bound().is_finite() }
    #[inline] fn is_up_bounded(&self) -> bool { false }
    #[inline] fn is_convex(&self) -> bool { true }
    #[inline] fn lower_bound(&self) -> Self::TimePoint { self.start }
    #[inline] fn upper_bound(&self) -> Self::TimePoint { Self::TimePoint::INFINITE }
}

impl<T:TimePoint> TimeConvex for RangeFrom<T> {
    #[inline]
    fn to_timerange(&self) -> TimeRange<T>  {
        assert!( !self.start.is_future_infinite());
        TimeRange { lower: self.start, upper: T::INFINITE}
    }
}

impl<T:TimePoint> TryFrom<RangeFrom<T> > for TimeRange<T>
{
    type Error = TimeError;

    #[inline]
    fn try_from(range: RangeFrom<T>) -> Result<Self, Self::Error> {
        if range.start.is_future_infinite() {
            Err(TimeError::FutureOverflow)
        } else {
            Ok(TimeRange { lower: range.start, upper: T::INFINITE})
        }
    }
}

//--------------------- TIME RANGE TRANSLATION -----------------------------------

impl<T> AddAssign<RangeFrom<TimeValue>> for TimeRange<T>
    where T:TimePoint+AddAssign<TimeValue>
{
    #[inline]
    fn add_assign(&mut self, other: RangeFrom<TimeValue>) {
        self.lower += other.lower_bound();
        assert!( !self.lower.is_future_infinite() );
        self.upper = T::INFINITE;
    }
}

impl<T> SubAssign<RangeFrom<TimeValue>> for TimeRange<T>
    where T:TimePoint+SubAssign<TimeValue>
{
    #[inline]
    fn sub_assign(&mut self, other: RangeFrom<TimeValue>) {
        self.upper -= other.lower_bound();
        assert!( !self.upper.is_past_infinite() );
        self.lower = -T::INFINITE;
    }
}

impl<T> Add<RangeFrom<TimeValue>> for TimeRange<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = Self;
    #[inline]
    fn add(self, other: RangeFrom<TimeValue>) -> Self::Output {
        TimeRange::after(self.lower + other.lower_bound()).unwrap()
    }
}

impl<T> Sub<RangeFrom<TimeValue>> for TimeRange<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = Self;
    #[inline]
    fn sub(self, other: RangeFrom<TimeValue>) -> Self::Output {
        TimeRange::before(self.upper - other.lower_bound()).unwrap()
    }
}

impl Add<RangeFrom<Timestamp>> for TimeInterval
{
    type Output = TimeSlot;
    #[inline]
    fn add(self, other: RangeFrom<Timestamp>) -> Self::Output {
        TimeRange::after(self.lower + other.lower_bound()).unwrap()
    }
}

impl Sub<RangeFrom<Timestamp>> for TimeInterval
{
    type Output = TimeSlot;
    #[inline]
    fn sub(self, other: RangeFrom<Timestamp>) -> Self::Output {
        TimeRange::before(self.upper - other.lower_bound()).unwrap()
    }
}


//--------------------- TIME SET TRANSLATION -----------------------------------


impl<T> AddAssign<RangeFrom<TimeValue>> for TimeSet<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    #[inline]
    fn add_assign(&mut self, other: RangeFrom<TimeValue>) {
        *self += other.to_timerange();
    }
}

impl<T> SubAssign<RangeFrom<TimeValue>> for TimeSet<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    #[inline]
    fn sub_assign(&mut self, other: RangeFrom<TimeValue>) {
        *self -= other.to_timerange()
    }
}

impl<T> Add<RangeFrom<TimeValue>> for TimeSet<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = TimeRange<T>;
    #[inline]
    fn add(self, other: RangeFrom<TimeValue>) -> Self::Output {
        TimeRange::after(self.lower_bound() + other.lower_bound()).unwrap()
    }
}

impl<T> Sub<RangeFrom<TimeValue>> for TimeSet<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = TimeRange<T>;
    #[inline] fn sub(self, other: RangeFrom<TimeValue>) -> Self::Output {
        TimeRange::before(self.upper_bound() - other.upper_bound()).unwrap()
    }
}
