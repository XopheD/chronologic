use crate::*;

/// # A trait for time containing
pub trait TimeContaining<TW> {
    fn contains(&self, rhs: &TW) -> bool;
}


impl<TW1:TimeConvex,TW2> TimeContaining<TW2> for TW1
    where TW2: TimeBounds<TimePoint=TW1::TimePoint>
{
    #[inline]
    fn contains(&self, rhs: &TW2) -> bool {
        self.lower_bound() <= rhs.lower_bound() && rhs.upper_bound() <= self.upper_bound()
    }
}


impl<T:TimePoint,TW> TimeContaining<TW> for TimeSet<T>
    where TW: TimeConvex<TimePoint=T>
{
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

impl<T:TimePoint> TimeContaining<Self> for TimeSet<T>
{
    fn contains(&self, rhs: &Self) -> bool {
        // todo: optimise it by using order of inner intervals
        rhs.into_iter().all(|tw| self.contains(&tw))
    }
}