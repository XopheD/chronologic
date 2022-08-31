use std::ops::{Range, Add, Sub, AddAssign, SubAssign};
use crate::*;

//--------------------- TIME RANGE TRANSLATION -----------------------------------


impl<T> AddAssign<Range<TimeValue>> for TimeInterval<T>
    where T:TimePoint+AddAssign<TimeValue>
{
    #[inline]
    fn add_assign(&mut self, other: Range<TimeValue>)
    {
        assert! [ !other.is_empty() ];
        self.lower += other.lower_bound();
        assert!( !self.lower.is_future_infinite() );
        self.upper += other.upper_bound();
        assert!( !self.upper.is_past_infinite() );
    }
}

impl<T> SubAssign<Range<TimeValue>> for TimeInterval<T>
    where T:TimePoint+SubAssign<TimeValue>
{
    #[inline]
    fn sub_assign(&mut self, other: Range<TimeValue>)
    {
        assert! [ !other.is_empty() ];
        self.lower -= other.upper_bound();
        assert!( !self.lower.is_future_infinite() );
        self.upper -= other.lower_bound();
        assert!( !self.upper.is_past_infinite() );
    }
}

impl<T> Add<Range<TimeValue>> for TimeInterval<T>
    where T:TimePoint+Add<TimeValue,Output=T>
{
    type Output = Self;
    #[inline]
    fn add(self, other: Range<TimeValue>) -> Self::Output {
        let tw = TimeInterval::new(self.lower+other.lower_bound(),self.upper+other.upper_bound());
        debug_assert!(!tw.is_empty(), "time interval translation overflows");
        tw
    }
}

impl<T> Sub<Range<TimeValue>> for TimeInterval<T>
    where T:TimePoint+Sub<TimeValue,Output=T>
{
    type Output = Self;
    #[inline]
    fn sub(self, other: Range<TimeValue>) -> Self::Output {
        let tw = TimeInterval::new(self.lower-other.upper_bound(), self.upper-other.lower_bound());
        debug_assert!(!tw.is_empty(), "time interval translation overflows");
        tw
    }
}

impl Add<Range<Timestamp>> for TimeSpan
{
    type Output = TimeSlot;
    #[inline]
    fn add(self, other: Range<Timestamp>) -> Self::Output {
        let tw = TimeInterval::new(
            self.lower+other.lower_bound(),
            self.upper+other.upper_bound()
        );
        debug_assert!(!tw.is_empty(), "time interval translation overflows");
        tw
    }
}

impl Sub<Range<Timestamp>> for TimeSpan
{
    type Output = TimeSlot;
    #[inline]
    fn sub(self, other: Range<Timestamp>) -> Self::Output {
        let tw = TimeInterval::new(
            self.lower-other.upper_bound(),
            self.upper-other.lower_bound()
        );
        debug_assert!(!tw.is_empty(), "time interval translation overflows");
        tw
    }
}


//--------------------- TIME SET TRANSLATION -----------------------------------


impl<T> AddAssign<Range<TimeValue>> for TimeSet<T>
    where
        T:TimePoint+Add<TimeValue,Output=T>,
        Self: AddAssign<TimeSpan>
{
    #[inline]
    fn add_assign(&mut self, other: Range<TimeValue>) {
        *self += TimeSpan::from(other);
    }
}

impl<T> SubAssign<Range<TimeValue>> for TimeSet<T>
    where
        T:TimePoint+Sub<TimeValue,Output=T>,
        Self: SubAssign<TimeSpan>
{
    #[inline]
    fn sub_assign(&mut self, other: Range<TimeValue>) {
        *self -= TimeSpan::from(other)
    }
}

impl<T> Add<Range<TimeValue>> for TimeSet<T>
    where
        T:TimePoint+Add<TimeValue,Output=T>,
        Self: Add<TimeSpan, Output=Self>
{
    type Output = TimeSet<T>;
    #[inline]
    fn add(self, other: Range<TimeValue>) -> Self::Output {
        self + TimeSpan::from(other)
    }
}

impl<T> Sub<Range<TimeValue>> for TimeSet<T>
    where
        T:TimePoint+Sub<TimeValue,Output=T>,
        Self: Sub<TimeSpan, Output=Self>
{
    type Output = TimeSet<T>;
    #[inline] fn sub(self, other: Range<TimeValue>) -> Self::Output {
        self - TimeSpan::from(other)
    }
}
