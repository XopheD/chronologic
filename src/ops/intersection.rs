use crate::*;


/// Time window intersection
pub trait TimeIntersection<TW>
{
    type Output;
    fn intersection(self, tw: TW) -> Self::Output;
}


impl<T:TimePoint,TW> TimeIntersection<TW> for TimeInterval<T>
    where TW: Into<TimeInterval<T>>
{
    type Output = Self;
    #[inline] fn intersection(self, tw: TW) -> Self::Output { (&self).intersection(tw) }
}


impl<T:TimePoint> TimeIntersection<Self> for TimeSet<T>
{
    type Output = Self;
    #[inline] fn intersection(self, tw: Self) -> Self::Output { (&self).intersection(tw) }
}

impl<T:TimePoint> TimeIntersection<&Self> for TimeSet<T>
{
    type Output = Self;
    #[inline] fn intersection(self, tw: &Self) -> Self::Output { (&self).intersection(tw) }
}

impl<T:TimePoint, TW> TimeIntersection<TW> for TimeSet<T>
    where TW: Into<TimeInterval<T>>
{
    type Output = Self;
    #[inline] fn intersection(self, tw: TW) -> Self::Output { (&self).intersection(tw) }
}


impl<T:TimePoint> TimeIntersection<TimeSet<T>> for &TimeSet<T>
{
    type Output = TimeSet<T>;

    #[inline]
    fn intersection(self, tw: TimeSet<T>) -> Self::Output {
        self.into_iter().intersection(tw.into_iter()).collect()
    }
}

impl<T:TimePoint> TimeIntersection<Self> for &TimeSet<T>
{
    type Output = TimeSet<T>;

    #[inline]
    fn intersection(self, tw: &TimeSet<T>) -> Self::Output {
        self.into_iter().intersection(tw.into_iter()).collect()
    }
}


impl<T:TimePoint, TW> TimeIntersection<TW> for &TimeSet<T>
    where TW: Into<TimeInterval<T>>
{
    type Output = TimeSet<T>;

    #[inline]
    fn intersection(self, tw: TW) -> Self::Output {
        self.into_iter().intersection(tw.into()).collect()
    }
}


impl<T:TimePoint,TW> TimeIntersection<TW> for &TimeInterval<T>
    where TW: Into<TimeInterval<T>>
{
    type Output = TimeInterval<T>;

    #[inline]
    fn intersection(self, tw: TW) -> Self::Output {
        let i = tw.into();
        TimeInterval {
            lower: self.lower.max(i.lower),
            upper: self.upper.min(i.upper)
        }
    }
}