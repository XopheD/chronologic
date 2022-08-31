use crate::*;


/// # Time window union
pub trait TimeUnion<TW>
{
    type Output;
    fn union(self, tw: TW) -> Self::Output;
}


impl<T:TimePoint,TW> TimeUnion<TW> for TimeInterval<T>
    where TW: Into<TimeInterval<T>>
{
    type Output = TimeSet<T>;
    #[inline] fn union(self, tw: TW) -> Self::Output { (&self).union(tw) }
}


impl<T:TimePoint> TimeUnion<Self> for TimeSet<T>
{
    type Output = Self;
    #[inline] fn union(self, tw: Self) -> Self::Output { (&self).union(tw) }
}

impl<T:TimePoint> TimeUnion<&Self> for TimeSet<T>
{
    type Output = Self;
    #[inline] fn union(self, tw: &Self) -> Self::Output { (&self).union(tw) }
}

impl<T:TimePoint, TW> TimeUnion<TW> for TimeSet<T>
    where TW: Into<TimeInterval<T>>
{
    type Output = Self;
    #[inline] fn union(self, tw: TW) -> Self::Output { (&self).union(tw) }
}


impl<T:TimePoint> TimeUnion<TimeSet<T>> for &TimeSet<T>
{
    type Output = TimeSet<T>;

    #[inline]
    fn union(self, tw: TimeSet<T>) -> Self::Output {
        self.into_iter().union(tw.into_iter()).collect()
    }
}

impl<T:TimePoint> TimeUnion<Self> for &TimeSet<T>
{
    type Output = TimeSet<T>;

    #[inline]
    fn union(self, tw: &TimeSet<T>) -> Self::Output {
        self.into_iter().union(tw.into_iter()).collect()
    }
}


impl<T:TimePoint, TW> TimeUnion<TW> for &TimeSet<T>
    where TW: Into<TimeInterval<T>>
{
    type Output = TimeSet<T>;

    #[inline]
    fn union(self, tw: TW) -> Self::Output {
        self.into_iter().union(tw.into()).collect()
    }
}


impl<T:TimePoint,TW> TimeUnion<TW> for &TimeInterval<T>
    where TW: Into<TimeInterval<T>>
{
    type Output = TimeSet<T>;

    fn union(self, tw: TW) -> Self::Output
    {
        let i = tw.into();
        if self.is_empty() {
            if i.is_empty() {
                TimeSet::empty()
            } else {
                i.into()
            }
        } else {
            if i.is_empty() {
                self.into()
            } else {
                if i.upper < self.lower.just_before() {
                    TimeSet(vec![i, *self])
                } else if self.upper < i.lower.just_before() {
                    TimeSet(vec![*self, i])
                } else {
                    TimeSet(vec![
                        TimeInterval {
                            lower: self.lower.min(i.lower),
                            upper: self.upper.max(i.upper)
                        }
                    ])
                }
            }
        }
    }
}