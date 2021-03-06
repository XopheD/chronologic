// union, intersection and complementary

use crate::*;
use std::ops::*;

macro_rules! timecomplsingle {
    ($time:ty) => {
        impl Not for $time {
            type Output = TimeSet<Self>;
            fn not(self) -> Self::Output {
                let before = self.just_before();
                let after = self.just_after();
                if before.is_past_infinite() {
                    TimeSet(vec![ TimeInterval { lower: after, upper: Self::INFINITE } ])
                } else if after.is_future_infinite() {
                    TimeSet(vec![ TimeInterval { lower: -Self::INFINITE, upper: before } ])
                } else {
                    TimeSet(vec![
                        TimeInterval { lower: -Self::INFINITE, upper: before },
                        TimeInterval { lower: after, upper: Self::INFINITE },
                    ])
                }
            }
        }
    };
}
timecomplsingle!(TimeValue);
timecomplsingle!(Timestamp);

impl<T:TimePoint> Not for TimeInterval<T> {

    type Output = TimeSet<T>;

    fn not(self) -> Self::Output
    {
        let cut1 = self.lower_bound().just_before();
        let cut2 = self.upper_bound().just_after();
        if cut1.is_past_infinite() {
            if cut2.is_future_infinite() {
                TimeSet::empty()
            } else {
                TimeSet(vec![TimeInterval { lower: cut2, upper: T::INFINITE }])
            }
        } else {
            if cut2.is_future_infinite() {
                TimeSet(vec![TimeInterval { lower: -T::INFINITE, upper: cut1 }])
            } else {
                TimeSet(vec![
                    TimeInterval { lower: -T::INFINITE, upper: cut1 },
                    TimeInterval { lower: cut2, upper: T::INFINITE },
                ])
            }
        }
    }
}

impl<T:TimePoint> Not for TimeSet<T>
{
    type Output = Self;

    fn not(self) -> Self::Output
    {
        if let Some(first) = self.0.first() {
            let mut compl = Vec::with_capacity(self.0.len() + 1);
            if let Ok(start) = TimeInterval::before(first.lower.just_before()) {
                compl.push(start);
            }
            let mut previous = first.upper.just_after();
            self.0.iter().skip(1)
                .for_each(|tw| {
                    compl.push(TimeInterval { lower: previous, upper: tw.lower.just_before() });
                    previous = tw.upper.just_after();
                });
            if let Ok(end) = TimeInterval::after(previous) {
                compl.push(end);
            }
            TimeSet(compl)
        } else {
            TimeSet::all()
        }
    }
}


impl<T:TimePoint,TW> BitAnd<TW> for TimeSet<T>
    where
        TW: TimeWindow<TimePoint=T>+TimeConvex
{
    type Output = Self;

    fn bitand(self, other: TW) -> Self::Output
    {
        if other.is_empty() {
            TimeSet::empty()
        } else {
            let mut inners = self.0.into_iter()
                .skip_while(|tw| tw.upper < other.lower_bound())
                .take_while(|tw| tw.lower <= other.upper_bound())
                .collect::<Vec<_>>();
            if let Some(last) = inners.last_mut() {
                last.upper = last.upper.min(other.upper_bound());
                let first = unsafe { inners.get_unchecked_mut(0) };
                first.lower = first.lower.max(other.lower_bound());
            }
            TimeSet(inners)
        }
    }
}

impl<T:TimePoint> BitAnd for TimeSet<T>
{
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output
    { // todo: optim. needed (to avoid multiple cloning)
        Self(other.0.into_iter()
            .map(|c| (self.clone() & c).0.into_iter())
            .flatten()
            .collect())

    }
}

impl<T:TimePoint,TW: TimeWindow<TimePoint=T>+TimeConvex> BitAndAssign<TW> for TimeSet<T>
{
    fn bitand_assign(&mut self, other: TW)
    {
        if other.is_empty() {
            self.0.clear();
        } else {
            self.0 = self.0.iter().copied()
                .skip_while(|tw| tw.upper < other.lower_bound())
                .take_while(|tw| tw.lower <= other.upper_bound())
                .collect::<Vec<_>>();
            if let Some(last) = self.0.last_mut() {
                last.upper = last.upper.min(other.upper_bound());
                let first = unsafe { self.0.get_unchecked_mut(0) };
                first.lower = first.lower.max(other.lower_bound());
            }
        }
    }
}

impl<T:TimePoint> BitAndAssign for TimeSet<T>
{
    fn bitand_assign(&mut self, other: Self) {
        *self = self.clone() & other;
    }
}

impl<T:TimePoint,TW> BitOr<TW> for TimeSet<T>
    where
        TW: TimeWindow<TimePoint=T>+Not<Output=TimeSet<T>>
{
    type Output = Self;

    fn bitor(self, other: TW) -> Self::Output {
        // quick and dirty... (todo)
        !(!self & !other)
    }
}

impl<T:TimePoint,TW> BitOrAssign<TW> for TimeSet<T>
    where
        TW: TimeWindow<TimePoint=T>+Not<Output=TimeSet<T>>
{
    fn bitor_assign(&mut self, other: TW) {
        *self = self.clone() | other
    }
}


impl<T:TimePoint> BitAnd for TimeInterval<T>
{
    type Output = TimeSet<T>;

    fn bitand(self, other: Self) -> Self::Output
    {
        if let Some(tw) = self.intersection(&other) {
            tw.into()
        } else {
            TimeSet::empty()
        }
    }
}


impl<T:TimePoint> BitOr for TimeInterval<T>
{
    type Output = TimeSet<T>;

    fn bitor(self, other: Self) -> Self::Output
    {
        if self.upper < other.lower.just_before() {
            TimeSet(vec![self, other])
        } else if other.upper < self.lower.just_before() {
            TimeSet(vec![other, self])
        } else {
            TimeSet(vec![ TimeInterval {
                lower: self.lower.min(other.lower),
                upper: self.upper.max(other.upper)
            }])
        }
    }
}

pub trait TimeRemoval<TW> {
    fn remove(&mut self, tw: &TW);
}

impl<T:TimePoint> TimeRemoval<Self> for TimeSet<T>
{
    fn remove(&mut self, tw: &Self) {
        *self &= !tw.clone()
    }
}

impl<T:TimePoint,TW> TimeRemoval<TW> for TimeSet<T>
    where
        TW:TimeConvex+TimeWindow<TimePoint=T>
{
    fn remove(&mut self, tw: &TW) {
        *self &= !TimeInterval { lower: tw.lower_bound(), upper: tw.upper_bound() }
    }
}
