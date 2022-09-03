use crate::*;

/// # A trait for checking time overlapping
pub trait TimeOverlapping<TW> {
    fn overlaps(&self, rhs: &TW) -> bool;
    fn contains(&self, rhs: &TW) -> bool;
}


impl<TW1:TimeConvex,TW2:TimeConvex> TimeOverlapping<TW2> for TW1
    where TW2: TimeBounds<TimePoint=TW1::TimePoint>
{
    #[inline]
    fn overlaps(&self, rhs: &TW2) -> bool {
        self.lower_bound() <= rhs.upper_bound() && rhs.lower_bound() <= self.upper_bound()
    }

    #[inline]
    fn contains(&self, rhs: &TW2) -> bool {
        self.lower_bound() <= rhs.lower_bound() && rhs.upper_bound() <= self.upper_bound()
    }
}


impl<TW:TimeConvex> TimeOverlapping<TimeSet<TW::TimePoint>> for TW
{
    #[inline]
    fn overlaps(&self, rhs: &TimeSet<TW::TimePoint>) -> bool {
        self.lower_bound() <= rhs.upper_bound() && rhs.lower_bound() <= self.upper_bound()
    }

    #[inline]
    fn contains(&self, rhs: &TimeSet<TW::TimePoint>) -> bool {
        self.lower_bound() <= rhs.lower_bound() && rhs.upper_bound() <= self.upper_bound()
    }
}

impl<T:TimePoint, TW> TimeOverlapping<TW> for TimeSet<T>
    where TW: TimeConvex<TimePoint=T>
{
    #[inline]
    fn overlaps(&self, rhs: &TW) -> bool
    {
        self.0.iter()
            .skip_while(|ts| ts.upper_bound() < rhs.lower_bound())
            .next()
            .map(|ts| ts.lower_bound() <= rhs.upper_bound())
            .unwrap_or(false)
    }

    #[inline]
    fn contains(&self, rhs: &TW) -> bool
    {
        self.0.iter()
            .skip_while(|ts| ts.upper_bound() < rhs.lower_bound())
            .next()
            .map(|ts| ts.contains(rhs))
            .unwrap_or(false)
    }
}

impl<T:TimePoint> TimeOverlapping<Self> for TimeSet<T>
{
    fn overlaps(&self, rhs: &Self) -> bool {
        // todo: optimise it by using order of inner intervals
        rhs.into_iter().any(|tw| self.overlaps(&tw))
    }

    fn contains(&self, rhs: &Self) -> bool {
        // todo: optimise it by using order of inner intervals
        rhs.into_iter().all(|tw| self.contains(&tw))
    }
}