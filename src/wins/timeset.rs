use std::ops::Neg;
use crate::*;

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
#[derive(Clone, Eq, Hash)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
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

    #[inline]
    pub fn shrink_to_fit(&mut self) { self.0.shrink_to_fit() }
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
            .map(TimeInterval::lower_bound)
            .unwrap_or(Self::TimePoint::INFINITE)
    }

    #[inline]
    fn upper_bound(&self) -> Self::TimePoint {
        self.0.last()
            .map(TimeInterval::upper_bound)
            .unwrap_or(-Self::TimePoint::INFINITE)
    }
}


impl<T:TimePoint> TimeWindow for TimeSet<T>
{
    #[inline]
    fn convex_count(&self) -> usize { self.0.len() }

    type ConvexIter = crate::iter::intoiter::IntoConvexIter<T,std::vec::IntoIter<TimeInterval<T>>>;

    fn iter(&self) -> Self::ConvexIter {
        crate::iter::intoiter::IntoConvexIter(self.0.clone().into_iter())
    }
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
        let mut iter = iter.into_iter();
        match iter.next() {
            None => Self::empty(),
            Some(first) => iter.fold(first.into(), |mut r, i| {
                // very most of the time, time iterators are chronologically sorted
                // so if the gap is more than one tick, just add the new convex at the end
                if i.lower_bound() > r.upper_bound().just_after() {
                    r.0.push(i)
                } else {
                    r |= i
                }
                r
            })
        }
    }
}


impl<T: TimePoint> FromIterator<TimeSet<T>> for TimeSet<T>
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


impl<T:TimePoint> TimeTruncation for TimeSet<T>
{
    /// Returns `true` if something changed
    fn truncate_before(&mut self, lower: T) -> bool
    {
        match self.0.iter().position(|i| i.upper_bound() >= lower)
        {
            Some(0) => {
                // only the first element could change
                unsafe { self.0.get_unchecked_mut(0) }.truncate_before(lower)
            }
            Some(pos) => {
                let _ = self.0.drain(0..pos);
                // SAFETY: the position is returned by position, at least one element remains
                let _ = unsafe { self.0.get_unchecked_mut(0) }.truncate_before(lower);
                true
            }
            None if self.is_empty() => { false }
            None => { self.0.clear(); true }
        }
    }

    /// Returns `true` if something changed
    fn truncate_after(&mut self, upper: T) -> bool
    {
        match self.0.iter().rposition(|i| i.lower_bound() <= upper)
        {
            Some(pos) if pos+1 == self.0.len() => {
                // only the last element could change
                unsafe { self.0.get_unchecked_mut(pos) }.truncate_after(upper)
            }
            Some(pos) => {
                self.0.truncate(pos+1);
                // SAFETY: the position is returned by rposition, so it is surely valid
                let _ = unsafe { self.0.get_unchecked_mut(pos) }.truncate_after(upper);
                true
            }
            None if self.is_empty() => { false }
            None => { self.0.clear(); true }
        }
    }
}

