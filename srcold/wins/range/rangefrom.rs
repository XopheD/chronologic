use std::ops::{RangeFrom, Add, Sub, AddAssign, SubAssign};
use crate::*;


//--------------------- TIME RANGE TRANSLATION -----------------------------------

impl<T> AddAssign<RangeFrom<TimeValue>> for TimeInterval<T>
    where T:TimePoint+AddAssign<TimeValue>
{
    #[inline]
    fn add_assign(&mut self, other: RangeFrom<TimeValue>) {
        self.lower += other.lower_bound();
        assert!( !self.lower.is_future_infinite() );
        self.upper = T::INFINITE;
    }
}

impl<T> SubAssign<RangeFrom<TimeValue>> for TimeInterval<T>
    where T:TimePoint+SubAssign<TimeValue>
{
    #[inline]
    fn sub_assign(&mut self, other: RangeFrom<TimeValue>) {
        self.upper -= other.lower_bound();
        assert!( !self.upper.is_past_infinite() );
        self.lower = -T::INFINITE;
    }
}

impl<T> Add<RangeFrom<TimeValue>> for TimeInterval<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = Self;
    #[inline]
    fn add(self, other: RangeFrom<TimeValue>) -> Self::Output {
        let t = self.lower + other.lower_bound();
        debug_assert!(!t.is_future_infinite(), "time interval translation overflows");
        TimeInterval{ lower: t, upper: T::INFINITE }
    }
}

impl<T> Sub<RangeFrom<TimeValue>> for TimeInterval<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = Self;
    #[inline]
    fn sub(self, other: RangeFrom<TimeValue>) -> Self::Output {
        let t = self.upper - other.lower_bound();
        debug_assert!(!t.is_future_infinite(), "time interval translation overflows");
        TimeInterval { lower: -T::INFINITE, upper: t }
    }
}

impl Add<RangeFrom<Timestamp>> for TimeSpan
{
    type Output = TimeSlot;
    #[inline]
    fn add(self, other: RangeFrom<Timestamp>) -> Self::Output {
        let t = self.lower + other.lower_bound();
        debug_assert!(!t.is_future_infinite(), "time interval translation overflows");
        TimeInterval{ lower: t, upper: Timestamp::INFINITE }
    }
}

impl Sub<RangeFrom<Timestamp>> for TimeSpan
{
    type Output = TimeSlot;
    #[inline]
    fn sub(self, other: RangeFrom<Timestamp>) -> Self::Output {
        let t = self.upper - other.lower_bound();
        debug_assert!(!t.is_future_infinite(), "time interval translation overflows");
        TimeInterval { lower: -Timestamp::INFINITE, upper: t }
    }
}


//--------------------- TIME SET TRANSLATION -----------------------------------


impl<T> AddAssign<RangeFrom<TimeValue>> for TimeSet<T>
    where
        T:TimePoint+Add<TimeValue,Output=T>,
        Self: AddAssign<TimeSpan>
{
    #[inline]
    fn add_assign(&mut self, other: RangeFrom<TimeValue>) {
        *self += TimeSpan::from(other)
    }
}

impl<T> SubAssign<RangeFrom<TimeValue>> for TimeSet<T>
    where
        T:TimePoint+Sub<TimeValue,Output=T>,
        Self: SubAssign<TimeSpan>
{
    #[inline]
    fn sub_assign(&mut self, other: RangeFrom<TimeValue>) {
        *self -= TimeSpan::from(other)
    }
}

impl<T> Add<RangeFrom<TimeValue>> for TimeSet<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = TimeInterval<T>;
    #[inline]
    fn add(self, other: RangeFrom<TimeValue>) -> Self::Output {
        let t = self.lower_bound() + other.lower_bound();
        debug_assert!(!t.is_future_infinite(), "time interval translation overflows");
        TimeInterval { lower: t, upper: T::INFINITE }
    }
}

impl<T> Sub<RangeFrom<TimeValue>> for TimeSet<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = TimeInterval<T>;
    #[inline] fn sub(self, other: RangeFrom<TimeValue>) -> Self::Output {
        let t = self.upper_bound() - other.upper_bound();
        debug_assert!(!t.is_past_infinite(), "time interval translation overflows");
        TimeInterval { lower: -T::INFINITE, upper: t }
    }
}
