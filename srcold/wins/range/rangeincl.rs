use std::ops::{RangeInclusive, Add, Sub, AddAssign, SubAssign};
use crate::*;


/*
impl<T:TimePoint> IntoTimeConvexIterator for RangeInclusive<T>
{
    type TimePoint = T;
    type IntoIter = std::option::IntoIter<TimeInterval<T>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter
    {
        if self.is_empty() {
            None.into_iter()
        } else {
            Some(TimeInterval {
                lower:self.lower_bound(),
                upper:self.upper_bound()
            }).into_iter()
        }
    }
}*/
//--------------------- TIME RANGE TRANSLATION -----------------------------------

impl<T> AddAssign<RangeInclusive<TimeValue>> for TimeInterval<T>
    where T:TimePoint+AddAssign<TimeValue>
{
    #[inline]
    fn add_assign(&mut self, other: RangeInclusive<TimeValue>)
    {
        assert! [ !other.is_empty() ];
        self.lower += other.lower_bound();
        assert!( !self.lower.is_future_infinite() );
        self.upper += other.upper_bound();
        assert!( !self.upper.is_past_infinite() );
    }
}


impl<T> SubAssign<RangeInclusive<TimeValue>> for TimeInterval<T>
    where T:TimePoint+SubAssign<TimeValue>
{
    #[inline]
    fn sub_assign(&mut self, other: RangeInclusive<TimeValue>)
    {
        assert! [ !other.is_empty() ];
        self.lower -= other.upper_bound();
        assert!( !self.lower.is_future_infinite() );
        self.upper -= other.lower_bound();
        assert!( !self.upper.is_past_infinite() );
    }
}

impl<T> Add<RangeInclusive<TimeValue>> for TimeInterval<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = Self;
    #[inline]
    fn add(self, other: RangeInclusive<TimeValue>) -> Self::Output {
        let tw = TimeInterval::new(
            self.lower+other.lower_bound(),
            self.upper+other.upper_bound()
        );
        debug_assert!(!tw.is_empty(), "time interval translation overflows");
        tw
    }
}

impl<T> Sub<RangeInclusive<TimeValue>> for TimeInterval<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = Self;
    #[inline]
    fn sub(self, other: RangeInclusive<TimeValue>) -> Self::Output {
        let tw = TimeInterval::new(
            self.lower-other.upper_bound(),
            self.upper-other.lower_bound()
        );
        debug_assert!(!tw.is_empty(), "time interval translation overflows");
        tw
    }
}

impl Add<RangeInclusive<Timestamp>> for TimeSpan
{
    type Output = TimeSlot;
    #[inline]
    fn add(self, other: RangeInclusive<Timestamp>) -> Self::Output {
        let tw = TimeInterval::new(
            self.lower+other.lower_bound(),
            self.upper+other.upper_bound()
        );
        debug_assert!(!tw.is_empty(), "time interval translation overflows");
        tw
    }
}

impl Sub<RangeInclusive<Timestamp>> for TimeSpan
{
    type Output = TimeSlot;
    #[inline]
    fn sub(self, other: RangeInclusive<Timestamp>) -> Self::Output {
        let tw = TimeInterval::new(
            self.lower-other.upper_bound(),
            self.upper-other.lower_bound()
        );
        debug_assert!(!tw.is_empty(), "time interval translation overflows");
        tw
    }
}


//--------------------- TIME SET TRANSLATION -----------------------------------


impl<T> AddAssign<RangeInclusive<TimeValue>> for TimeSet<T>
    where
        T:TimePoint+Add<TimeValue,Output=T>,
        Self: AddAssign<TimeSpan>
{
    #[inline]
    fn add_assign(&mut self, other: RangeInclusive<TimeValue>) {
        *self += TimeSpan::from(other);
    }
}

impl<T> SubAssign<RangeInclusive<TimeValue>> for TimeSet<T>
    where
        T:TimePoint+Sub<TimeValue,Output=T>,
        Self: SubAssign<TimeSpan>
{
    #[inline]
    fn sub_assign(&mut self, other: RangeInclusive<TimeValue>) {
        *self -= TimeSpan::from(other);
    }
}

impl<T> Add<RangeInclusive<TimeValue>> for TimeSet<T>
    where
        T:TimePoint+Add<TimeValue,Output=T>,
        Self: Add<TimeSpan,Output=Self>
{
    type Output = TimeSet<T>;
    #[inline]
    fn add(self, other: RangeInclusive<TimeValue>) -> Self::Output {
        self + TimeSpan::from(other)
    }
}

impl<T> Sub<RangeInclusive<TimeValue>> for TimeSet<T>
    where
        T:TimePoint+Sub<TimeValue,Output=T>,
        Self: Sub<TimeSpan,Output=Self>
{
    type Output = TimeSet<T>;
    #[inline] fn sub(self, other: RangeInclusive<TimeValue>) -> Self::Output {
        self - TimeSpan::from(other)
    }
}
