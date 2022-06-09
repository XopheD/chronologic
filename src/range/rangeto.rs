use std::ops::{RangeTo, Add, Sub, AddAssign, SubAssign};
use crate::*;
use crate::error::TimeError;


impl<T:TimePoint> TimeWindow for RangeTo<T>
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

impl<T:TimePoint> From<RangeTo<T> > for TimeInterval<T>
{
    #[inline]
    fn from(range: RangeTo<T>) -> Self {
        let upper = range.end.just_before();
        assert!( !upper.is_past_infinite() );
        TimeInterval { lower: -T::INFINITE, upper}
    }
}

//--------------------- TIME RANGE TRANSLATION -----------------------------------

impl<T> AddAssign<RangeTo<TimeValue>> for TimeInterval<T>
    where T:TimePoint+AddAssign<TimeValue>
{
    #[inline]
    fn add_assign(&mut self, other: RangeTo<TimeValue>) {
        self.upper += other.upper_bound();
        assert!( !self.upper.is_past_infinite() );
        self.lower = -T::INFINITE;
    }
}

impl<T> SubAssign<RangeTo<TimeValue>> for TimeInterval<T>
    where T:TimePoint+SubAssign<TimeValue>
{
    #[inline]
    fn sub_assign(&mut self, other: RangeTo<TimeValue>) {
        self.lower -= other.upper_bound();
        assert!( !self.lower.is_future_infinite() );
        self.upper = T::INFINITE;
    }
}

impl<T> Add<RangeTo<TimeValue>> for TimeInterval<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = Self;
    #[inline]
    fn add(self, other: RangeTo<TimeValue>) -> Self::Output {
        TimeInterval::before(self.upper + other.upper_bound()).unwrap()
    }
}

impl<T> Sub<RangeTo<TimeValue>> for TimeInterval<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = Self;
    #[inline]
    fn sub(self, other: RangeTo<TimeValue>) -> Self::Output {
        TimeInterval::after(self.lower - other.upper_bound()).unwrap()
    }
}

impl Add<RangeTo<Timestamp>> for TimeSpan
{
    type Output = TimeSlot;
    #[inline]
    fn add(self, other: RangeTo<Timestamp>) -> Self::Output {
        TimeInterval::before(self.upper + other.upper_bound()).unwrap()
    }
}

impl Sub<RangeTo<Timestamp>> for TimeSpan
{
    type Output = TimeSlot;
    #[inline]
    fn sub(self, other: RangeTo<Timestamp>) -> Self::Output {
        TimeInterval::after(self.lower - other.upper_bound()).unwrap()
    }
}


//--------------------- TIME SET TRANSLATION -----------------------------------


impl<T> AddAssign<RangeTo<TimeValue>> for TimeSet<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    #[inline]
    fn add_assign(&mut self, other: RangeTo<TimeValue>) {
        *self += TimeSpan::from(other);
    }
}

impl<T> SubAssign<RangeTo<TimeValue>> for TimeSet<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    #[inline]
    fn sub_assign(&mut self, other: RangeTo<TimeValue>) {
        *self -= TimeSpan::from(other);
    }
}

impl<T> Add<RangeTo<TimeValue>> for TimeSet<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = TimeInterval<T>;
    #[inline]
    fn add(self, other: RangeTo<TimeValue>) -> Self::Output {
        TimeInterval::before(self.upper_bound() + other.upper_bound()).unwrap()
    }
}

impl<T> Sub<RangeTo<TimeValue>> for TimeSet<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = TimeInterval<T>;
    #[inline] fn sub(self, other: RangeTo<TimeValue>) -> Self::Output {
        TimeInterval::after(self.lower_bound() - other.upper_bound()).unwrap()
    }
}
