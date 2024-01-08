use std::fmt;
use std::iter::FusedIterator;
use std::mem::swap;
use crate::*;
use crate::graph::{Instant, TimeConstraint, TimeGraph};



impl TimeGraph {

    /// Gets the constraint between two instants, if it exists
    ///
    /// If one instant is out of the graph or if the instants are
    /// not constrained each other (i.e. `]-oo,+oo[`), then there is
    /// no constraint and `None` is returned.
    #[inline]
    pub fn constraint(&self, from:Instant, to:Instant) -> Option<TimeGraphConstraint<'_>>
    {
        if from >= self.size() || to >= self.size() {
            None
        } else {
            let k = TimeGraphConstraint { from, to, graph: self };
            if k.is_all() { None } else { Some(k) }
        }
    }

    /// Gets an iterator over all the propagated constraints _starting from_ `i`
    #[inline]
    pub fn constraints_from(&self, from: Instant) -> impl Iterator<Item=TimeGraphConstraint<'_>> {
        TimeConstraintFromIter { graph: self, i: from, j:0 }
    }

    /// Gets an iterator over all the propagated constraints _ending to_ `i`
    #[inline]
    pub fn constraints_to(&self, to:Instant) -> impl Iterator<Item=TimeGraphConstraint<'_>> {
        self.constraints_from(to).map(|mut k| {
            swap(&mut k.from, &mut k.to); k
        })
    }

    /// Gets an iterator over all the propagated constraints of the graph.
    ///
    /// Only relevant constraints are iterated.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item=TimeGraphConstraint<'_>> {
        TimeConstraintIter { graph: self, i:0, j:0 }
    }
}


/// A time constraint between two instants with start <= end
pub struct TimeGraphConstraint<'a> {
    from: Instant,
    to: Instant,
    graph: &'a TimeGraph
}

impl TimeBounds for TimeGraphConstraint<'_>
{
    type TimePoint = TimeValue;

    // we know that the graph has no empty constraint
    #[inline] fn is_empty(&self) -> bool { false }

    #[inline]
    fn is_low_bounded(&self) -> bool {
        !self.lower_bound().is_past_infinite()
    }

    #[inline]
    fn is_up_bounded(&self) -> bool {
        !self.upper_bound().is_future_infinite()
    }

    #[inline]
    fn lower_bound(&self) -> Self::TimePoint {
        unsafe { self.graph.lower(self.from, self.to) }
    }

    #[inline]
    fn upper_bound(&self) -> Self::TimePoint {
        unsafe { - self.graph.lower(self.to, self.from) }
    }
}
impl<'a> TimeConvex for TimeGraphConstraint<'a> {

}

impl<'a> TimeConstraint for TimeGraphConstraint<'a>
{
    #[inline]
    fn from(&self) -> Instant { self.from }
    #[inline]
    fn to(&self) -> Instant { self.to }
}


impl<'a> fmt::Debug for TimeGraphConstraint<'a>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        // SAFETY: this TimeConstraint structure guarantees that start >= end
        // (they only come from the time graph iterator)
        let k = TimeInterval {
            lower: self.lower_bound(),
            upper: self.upper_bound()
        };
        write!(f, "({}->{})={:?}", self.from, self.to, k)
    }
}


impl<'a> fmt::Display for TimeGraphConstraint<'a>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        // SAFETY: this TimeConstraint structure guarantees that start >= end
        // (they only come from the time graph iterator)
        let k = TimeInterval {
            lower: self.lower_bound(),
            upper: self.upper_bound()
        };
        write!(f, "t{} - t{} in {}", self.to, self.from, k)
    }
}


impl<'a> From<TimeGraphConstraint<'a>> for ((Instant, Instant), TimeSpan)
{
    #[inline]
    fn from(k: TimeGraphConstraint<'a>) -> Self {
        ((k.from, k.to), TimeInterval { lower: k.lower_bound(), upper: k.upper_bound() })
    }
}

#[derive(Debug,Clone)]
pub struct TimeConstraintIter<'a> {
    graph: &'a TimeGraph,
    i: Instant, j: Instant
}


impl<'a> Iterator for TimeConstraintIter<'a> {

    type Item = TimeGraphConstraint<'a>;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.i == self.graph.size() {
            // this bound checking guarantees also that
            // this iterator is a FusedIterator
            None
        } else {
            loop {
                self.j += 1;
                if self.j == self.graph.size() {
                    self.i += 1;
                    if self.i == self.graph.size() {
                        return None;
                    }
                    self.j = self.i;

                } else if unsafe { !self.graph.lower(self.i,self.j).is_past_infinite() }
                    || unsafe { !self.graph.lower(self.j,self.i).is_past_infinite() }
                {
                    return Some(TimeGraphConstraint {
                        from: self.i,
                        to: self.j,
                        graph: self.graph
                    })
                }
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>)
    {
        let all_constraints = (self.graph.size() * self.graph.size() - self.graph.size())/2;
        let previous_constraints =  (self.i * self.i - self.i)/2;
        let max = all_constraints - previous_constraints + 1 - self.j;
        (0, Some(max as usize))
    }
}

impl FusedIterator for TimeConstraintIter<'_> { }





#[derive(Debug,Clone)]
pub struct TimeConstraintFromIter<'a> {
    graph: &'a TimeGraph,
    i: Instant,
    j: Instant
}

impl<'a> Iterator for TimeConstraintFromIter<'a> {

    type Item = TimeGraphConstraint<'a>;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.i >= self.graph.size() {
            // this bound checking guarantees also that
            // this iterator is a FusedIterator
            None
        } else {
            let current = TimeGraphConstraint {
                from: self.i,
                to: self.j,
                graph: self.graph
            };
            loop { // search the next current
                self.j += 1;

                if self.j == self.graph.size() {
                    self.i = self.graph.size();
                    break;

                } else if self.i != self.j &&
                    (unsafe { !self.graph.lower(self.i,self.j).is_past_infinite() }
                        || unsafe { !self.graph.lower(self.j,self.i).is_past_infinite() }) {
                    break;
                }
            }
            Some(current)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>)
    {
        let all_constraints = (self.graph.size() * self.graph.size() - self.graph.size())/2;
        let previous_constraints =  (self.i * self.i - self.i)/2;
        let max = all_constraints - previous_constraints + 1 - self.j;
        (0, Some(max as usize))
    }
}

impl FusedIterator for TimeConstraintFromIter<'_> { }




impl<K:TimeConstraint> FromIterator<K> for TimeGraph
{
    fn from_iter<I:IntoIterator<Item=K>>(iter: I) -> Self
    {
        let mut graph = TimeGraph::default();
        graph.extend(iter).expect("inconsistent set of time constraints");
        graph
    }
}


impl<K:TimeConstraint> Extend<K> for TimeGraph
{
    fn extend<T: IntoIterator<Item=K>>(&mut self, iter: T) {
        self.extend(iter).expect("inconsistent set of time constraints");
    }
}