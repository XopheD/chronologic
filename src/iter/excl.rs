use crate::iter::*;


/// # Time window exclusion iterator
pub trait TimeExclusion<TW>: TimeConvexIterator
{
    type Output:TimeConvexIterator<TimePoint=Self::TimePoint>;
    fn exclusion(self, tw: TW) -> Self::Output;
}


impl<TW1:TimeConvexIterator,TW2> TimeExclusion<TW2> for TW1
    where
        TW1: TimeIntersection<TW2::Output>,
        TW2: TimeComplementary
{
    type Output = TW1::Output;

    #[inline]
    fn exclusion(self, tw: TW2) -> Self::Output {
        self.intersection(tw.complementary())
    }
}
