use std::ops::{RangeTo, Add, Sub, AddAssign, SubAssign};
use crate::*;
use crate::error::TimeError;


impl<T:TimePoint> TimeSpan for RangeTo<T>
{
    type TimePoint = T;
    #[inline] fn is_empty(&self) -> bool { self.upper_bound().is_past_infinite() }
    #[inline] fn is_singleton(&self) -> bool { false }
    #[inline] fn is_bounded(&self) -> bool { false }
    #[inline] fn is_low_bounded(&self) -> bool { false }
    #[inline] fn is_up_bounded(&self) -> bool { self.upper_bound().is_finite()  }
    #[inline] fn is_convex(&self) -> bool { true }
    #[inline] fn lower_bound(&self) -> Self::TimePoint { - Self::TimePoint::INFINITE }
    #[inline] fn upper_bound(&self) -> Self::TimePoint { self.end.just_before() } // not inclusive
}

impl<T:TimePoint> TimeConvex for RangeTo<T> {
    #[inline]
    fn to_timerange(&self) -> TimeRange<T> {
        let upper = self.end.just_before();
        assert!( !upper.is_past_infinite());
        TimeRange { lower: -T::INFINITE, upper}
    }
}

impl<T:TimePoint> TryFrom<RangeTo<T> > for TimeRange<T>
{
    type Error = TimeError;

    #[inline]
    fn try_from(range: RangeTo<T>) -> Result<Self, Self::Error> {
        let upper = range.end.just_before();
        if upper.is_past_infinite() {
            Err(TimeError::PastOverflow)
        } else {
            Ok(TimeRange { lower: -T::INFINITE, upper})
        }
    }
}

//--------------------- TIME RANGE TRANSLATION -----------------------------------

impl<T> AddAssign<RangeTo<TimeValue>> for TimeRange<T>
    where T:TimePoint+AddAssign<TimeValue>
{
    #[inline]
    fn add_assign(&mut self, other: RangeTo<TimeValue>) {
        self.upper += other.upper_bound();
        assert!( !self.upper.is_past_infinite() );
        self.lower = -T::INFINITE;
    }
}

impl<T> SubAssign<RangeTo<TimeValue>> for TimeRange<T>
    where T:TimePoint+SubAssign<TimeValue>
{
    #[inline]
    fn sub_assign(&mut self, other: RangeTo<TimeValue>) {
        self.lower -= other.upper_bound();
        assert!( !self.lower.is_future_infinite() );
        self.upper = T::INFINITE;
    }
}

impl<T> Add<RangeTo<TimeValue>> for TimeRange<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = Self;
    #[inline]
    fn add(self, other: RangeTo<TimeValue>) -> Self::Output {
        TimeRange::before(self.upper + other.upper_bound()).unwrap()
    }
}

impl<T> Sub<RangeTo<TimeValue>> for TimeRange<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = Self;
    #[inline]
    fn sub(self, other: RangeTo<TimeValue>) -> Self::Output {
        TimeRange::after(self.lower - other.upper_bound()).unwrap()
    }
}

impl Add<RangeTo<Timestamp>> for TimeInterval
{
    type Output = TimeSlot;
    #[inline]
    fn add(self, other: RangeTo<Timestamp>) -> Self::Output {
        TimeRange::before(self.upper + other.upper_bound()).unwrap()
    }
}

impl Sub<RangeTo<Timestamp>> for TimeInterval
{
    type Output = TimeSlot;
    #[inline]
    fn sub(self, other: RangeTo<Timestamp>) -> Self::Output {
        TimeRange::after(self.lower - other.upper_bound()).unwrap()
    }
}


//--------------------- TIME SET TRANSLATION -----------------------------------


impl<T> AddAssign<RangeTo<TimeValue>> for TimeSet<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    #[inline]
    fn add_assign(&mut self, other: RangeTo<TimeValue>) {
        *self += other.to_timerange();
    }
}

impl<T> SubAssign<RangeTo<TimeValue>> for TimeSet<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    #[inline]
    fn sub_assign(&mut self, other: RangeTo<TimeValue>) {
        *self -= other.to_timerange()
    }
}

impl<T> Add<RangeTo<TimeValue>> for TimeSet<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = TimeRange<T>;
    #[inline]
    fn add(self, other: RangeTo<TimeValue>) -> Self::Output {
        TimeRange::before(self.upper_bound() + other.upper_bound()).unwrap()
    }
}

impl<T> Sub<RangeTo<TimeValue>> for TimeSet<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = TimeRange<T>;
    #[inline] fn sub(self, other: RangeTo<TimeValue>) -> Self::Output {
        TimeRange::after(self.lower_bound() - other.upper_bound()).unwrap()
    }
}
