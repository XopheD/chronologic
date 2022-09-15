use std::ops::Add;
use crate::*;
use crate::iter::*;


/// # Time window translation iterator
pub trait TimeTranslation<TW>: TimeConvexIterator
{
    type Output:TimeConvexIterator<TimePoint=Self::TimePoint>;
    fn translation(self, tw: TW) -> Self::Output;
}


impl<I:TimeConvexIterator> TimeTranslation<TimeValue> for I
    where
        I::Item: Add<TimeValue,Output=I::Item>
{
    type Output = TimeValueTranslIter<I>;

    fn translation(self, t: TimeValue) -> Self::Output {
        TimeValueTranslIter{ t, iter: self }
    }
}

pub struct TimeValueTranslIter<I:TimeConvexIterator> {
    t: TimeValue,
    iter: I
}

impl<I:TimeConvexIterator> Iterator for TimeValueTranslIter<I>
    where
        I::Item: Add<TimeValue,Output=I::Item>
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|tw| tw+self.t)
            .and_then(|tw| if tw.is_empty() { None } else { Some(tw) })
    }
}

impl<I:TimeConvexIterator> TimeConvexIterator for TimeValueTranslIter<I>
    where
        I::Item: Add<TimeValue,Output=I::Item>
{
    type TimePoint = I::TimePoint;
}



impl<I:TimeConvexIterator> TimeTranslation<&TimeSpan> for I
    where
        I::Item: Add<TimeSpan,Output=I::Item>
{
    type Output = crate::iter::intoiter::IntoConvexIter<I::TimePoint,std::vec::IntoIter<TimeInterval<I::TimePoint>>>;

    fn translation(self, ts: &TimeSpan) -> Self::Output {
        let tw = self.fold(TimeSet::<I::TimePoint>::empty(), |r,tw| r | (tw + *ts));
        tw.into_iter()
    }
}
/*
impl<T:TimePoint> TimeTranslation<TimeValue> for TimeSet<T>
    where
        TimeInterval<T>:TimeTranslation<TimeValue,Output=TimeInterval<T>>
{
    type Output = Self;

    fn translate(mut self, t: &TimeValue) -> TimeResult<Self>
    {
        // adding a constant preserves the structure (order and distance
        // between successive intervals -> nothing to manage)
        // except if infinite values are reached
        self.0.iter_mut()
            .try_for_each(|i| Ok( *i = i.translate(t)? ))?;

        // suppress repeated {+oo,+oo} at the end
        while let Some(end) = self.0.get(self.0.len()-2) {
            if end.is_up_bounded() { break; }
            self.0.pop();
        }

        // suppress repeated {-oo,-oo} at the beginning
        while let Some(begin) = self.0.get(1) {
            if begin.is_low_bounded() { break; }
            self.0.remove(0);
        }

        Ok(self)
    }
}
*/