use crate::*;
use crate::wins::*;

/// Make intersection in place
pub trait TimeIntersection<Rhs>
    where
        Self: TimeWindow
{
    type Output;
    fn intersection(self, rhs: Rhs) -> Self::Output;
}


impl<TW1,TW2> TimeIntersection<TW1> for TW2
    where
        TW1: TimeConvex,
        TW2: TimeConvex<TimePoint=TW1::TimePoint>,
        TW1: Into<TimeInterval<TW1::TimePoint>>,
        TW2: Into<TimeInterval<TW2::TimePoint>>
{
    type Output = Option<TimeInterval<TW1::TimePoint>>;

    fn intersection(self, rhs: TW1) -> Self::Output
    {
        if self.is_empty() || rhs.is_empty()
            || self.upper_bound() < rhs.lower_bound()
            || rhs.upper_bound() < self.lower_bound()
        {
            None // empty intersection
        } else {
            Some(TimeInterval {
                lower: self.lower_bound().max(rhs.lower_bound()),
                upper: self.upper_bound().min(rhs.upper_bound())
            })
        }
    }
}

impl<T:TimePoint,TW> TimeIntersection<TW> for TimeSet<T>
    where
        TW: TimeConvex<TimePoint=T>
{
    type Output = Self;

    fn intersection(mut self, other: TW) -> Self::Output
    {
        if other.is_empty() {
            TimeSet::empty()
        } else {
            // keep only the relevant convex parts
            self.0 = self.0.into_iter()
                .skip_while(|tw| tw.upper < other.lower_bound())
                .take_while(|tw| tw.lower <= other.upper_bound())
                .collect::<Vec<_>>();
            // and adjust the bounds... (if non empty)
            if let Some(last) = self.0.last_mut() {
                last.upper = last.upper.min(other.upper_bound());
                let first = unsafe { self.0.get_unchecked_mut(0) };
                first.lower = first.lower.max(other.lower_bound());
            }
            self
        }
    }
}

impl<TW:TimeConvex> TimeIntersection<TimeSet<TW::TimePoint>> for TW
{
    type Output = TimeSet<TW::TimePoint>;
    #[inline]
    fn intersection(self, rhs: Self::Output) -> Self::Output {
        rhs.intersection(self)
    }
}


impl<T:TimePoint> TimeIntersection<Self> for TimeSet<T>
    where
        Self: TimeIntersection<TimeInterval<T>,Output=Self>
{
    type Output = Self;

    fn intersection(self, rhs: Self) -> Self::Output
    {
        // todo: optim. needed (to avoid multiple cloning)
        Self(rhs.0.into_iter()
            .map(|c| self.clone().intersection(c))
            .map(|c| c.0.into_iter())
            .flatten()
            .collect())
    }
}
