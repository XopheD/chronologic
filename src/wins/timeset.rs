use std::fmt;
use std::ops::Neg;
use crate::*;
use crate::iter::TimeUnion;


/// A union of [`TimeSpan`] (aliased to [`TimeSet<TimeValue>`])
pub type TimeSpans = TimeSet<TimeValue>;

/// A union of [`TimeSlot`] (aliased to [`TimeSet<Timestamp>`])
pub type TimeSlots = TimeSet<Timestamp>;

/// # A union of time intervals
///
/// This is the more generic structure to keep a set of time points.
/// It could be empty, convex or defined by pieces.
///
/// The inner list of time intervals is chronological sorted
/// and all the inner intervals are disjoint. If, when added,
/// two intervals overlaps, then they are merged.
#[derive(Clone, Eq, PartialEq, Hash, Default)]
pub struct TimeSet<T:TimePoint>(pub(crate) Vec<TimeInterval<T>>);

impl<T:TimePoint> TimeSet<T>
{
    /// The full interval `]-oo,+oo[`
    ///
    /// Returns a timeset composed of the full interval `]-oo,+oo[`
    #[inline]
    pub fn all() -> Self { Self(vec![TimeInterval::all()]) }

    /// A convex interval `[a,b]`
    ///
    /// Returns a timeset composed of one convex interval.
    #[inline]
    pub fn convex(lower: T, upper: T) -> Self
    {
        let tw = TimeInterval::new(lower, upper);
        if tw.is_empty() { Self::empty() } else { Self(vec![tw]) }
    }

    /// A singleton `{t}`
    ///
    /// Retuns a timeset composed of the convex interval `[t,t]`
    #[inline]
    pub fn singleton(t: T) -> Self {
        Self(vec![TimeInterval::singleton(t)])
    }

    /// The empty set
    #[inline]
    pub fn empty() -> Self { Self(vec![]) }

}



impl<T:TimePoint> TimeBounds for TimeSet<T>
{
    type TimePoint = T;

    #[inline]
    fn is_empty(&self) -> bool { self.0.is_empty() }

    #[inline]
    fn is_singleton(&self) -> bool {
        (self.0.len() == 1) && unsafe { self.0.get_unchecked(0).is_singleton() }
    }

    #[inline]
    fn is_bounded(&self) -> bool {
        self.is_low_bounded() && self.is_up_bounded()
    }

    #[inline]
    fn is_low_bounded(&self) -> bool {
        self.0.first().map(|s| s.is_low_bounded()).unwrap_or(false)
    }

    #[inline]
    fn is_up_bounded(&self) -> bool {
        self.0.last().map(|s| s.is_up_bounded()).unwrap_or(false)
    }

    #[inline]
    fn lower_bound(&self) -> Self::TimePoint {
        self.0.first()
            .map(|i| i.lower_bound())
            .unwrap_or(Self::TimePoint::INFINITE)
    }

    #[inline]
    fn upper_bound(&self) -> Self::TimePoint {
        self.0.last()
            .map(|i| i.upper_bound())
            .unwrap_or(-Self::TimePoint::INFINITE)
    }
}


impl<T:TimePoint> TimeWindow for TimeSet<T>
{
    #[inline]
    fn is_convex(&self) -> bool { self.0.len() <= 1 }


    #[inline]
    fn convex_count(&self) -> usize { self.0.len() }
}


impl<T:TimePoint> Neg for TimeSet<T>
{
    type Output = Self;
    #[inline] fn neg(self) -> Self {
        // negate each intervals AND reverse the list
        Self(self.0.iter().rev().map(|&t| -t).collect())
    }
}

impl<T:TimePoint> FromIterator<TimeInterval<T>> for TimeSet<T>
{
    fn from_iter<I: IntoIterator<Item=TimeInterval<T>>>(iter: I) -> Self
    {
        let mut iter = iter.into_iter()
            .filter(|i| !i.is_empty());

        match iter.next() {
            None => Self::empty(),
            Some(i) => {
                iter.fold(i.into(), |mut r,i| {
                    // very most of the time, time iterators are chronologically sorted
                    // if the gap is more than one tick, just add the new convex at the end
                    if i.lower_bound() > r.upper_bound().just_after() {
                        r.0.push(i.into()); r
                    } else {
                        // todo: could be improved
                        r.into_iter().union(i).collect()
                    }
                })
            }
        }
    }
}


impl<T:TimePoint> FromIterator<TimeSet<T>> for TimeSet<T>
{
    fn from_iter<I: IntoIterator<Item=TimeSet<T>>>(iter: I) -> Self
    {
        iter.into_iter()
            .reduce(|r,s| r|s)
            .unwrap_or(TimeSet::empty())
    }
}


impl<T,TW> From<TW> for TimeSet<T>
    where
        T:TimePoint,
        TW:TimeConvex<TimePoint=T>
{
    #[inline] fn from(tw: TW) -> Self
    {
        if tw.is_empty() {
            TimeSet::empty()
        } else {
            Self(vec![TimeInterval { lower: tw.lower_bound(), upper: tw.upper_bound()}])
        }
    }
}


impl<T:TimePoint+fmt::Debug> fmt::Debug for TimeSet<T>
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    {
        let mut iter = self.0.iter();
        if let Some(first) = iter.next() {
            write!(formatter, "{:?}", first)?;
            iter.try_for_each(|tw| write!(formatter, "U{:?}", tw))
        } else {
            write!(formatter, "{{}}") /* empty set */
        }
    }
}

impl<T:TimePoint+fmt::Display> fmt::Display for TimeSet<T>
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    {
        let mut iter = self.0.iter();
        if let Some(first) = iter.next() {
            write!(formatter, "{}", first)?;
            iter.try_for_each(|tw| write!(formatter, "U{}", tw))
        } else {
            write!(formatter, "{{}}") /* empty set */
        }
    }
}
