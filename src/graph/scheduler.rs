use std::fmt;
use std::iter;
use std::ops::BitAndAssign;
use crate::*;
use crate::graph::*;
use crate::graph::propagation::{TimeInconsistencyError, TimePropagation, TimePropagationResult};
use crate::iter::{TimeIntersection, TimeTranslation};


/// # A manager of constrained time variables.
///
/// A time scheduler maintains a set of time windows according
/// to a set of constraints.
///
/// The set of constraints (i.e. the graph) could be shared
/// by many schedulers which could differ by their chosen slots
/// (of course, the constraint set is satisfied for any scheduler).
/// It could be useful to explore many scheduler without duplicating the constraints.
#[derive(Clone)]
pub struct TimeScheduler<'a> {
    constraints: &'a TimeGraph,
    schedule: Box<[TimeSlots]>
}

impl TimeScheduler<'_> {

    pub fn new(graph: &TimeGraph) -> TimeScheduler
    {
        TimeScheduler {
            constraints: graph,
            schedule: iter::repeat(TimeSlots::all()).take(graph.size() as usize).collect()
        }
    }

    #[inline]
    pub fn constraints(&self) -> &'_ TimeGraph { self.constraints }

    #[inline]
    pub fn scheduling(&self, i:Instant) -> Option<&TimeSlots> { self.schedule.get(i as usize) }

    #[inline]
    pub fn schedule(&self) -> &[TimeSlots] { &self.schedule }

    /// The minimum of the upper bounds of each scheduling
    pub fn latest_beginning(&self) -> Timestamp {
        self.schedule.iter().map(|i| i.upper_bound()).min().unwrap()
    }

    /// The maximum of the lower bounds of each scheduling
    pub fn earliest_ending(&self) -> Timestamp {
        self.schedule.iter().map(|i| i.lower_bound()).max().unwrap()
    }

    /// Add constraint in order to guarantee that all the instants
    /// is scheduled before the specified deadline.
    pub fn set_deadline(&mut self, deadline: Timestamp) -> TimePropagationResult
    {
        // first, check if this deadline is compatible
        if self.schedule.iter().any(|tw| tw.lower_bound() > deadline) {
            Err(TimeInconsistencyError::Recovered)
        } else {
            // we know that the propagation will succeed
            Ok((0..self.schedule.len() as u32)
                .fold(TimePropagation::Unchanged,
                      |result, i|
                          match self.retain(i, ..=deadline).unwrap() {
                              TimePropagation::Unchanged => { result }
                              TimePropagation::Propagated => { TimePropagation::Propagated }
                          }))
        }
    }

    /// Add constraint in order to guarantee that all the instants
    /// is scheduled after the specified startline.
    pub fn set_startline(&mut self, startline: Timestamp) -> TimePropagationResult
    {
        // first, check if this startline is compatible
        if self.schedule.iter().any(|tw| tw.upper_bound() < startline) {
            Err(TimeInconsistencyError::Recovered)
        } else {
            // we know that the propagation will succeed
            Ok((0..self.schedule.len() as u32)
                .fold(TimePropagation::Unchanged,
                      |result, i|
                          match self.retain(i, startline..).unwrap() {
                              TimePropagation::Unchanged => { result }
                              TimePropagation::Propagated => { TimePropagation::Propagated }
                          }))
        }
    }


    /// Add a new constraint on one scheduler entry
    ///
    /// Only the timestamps in the specified time window are retained
    /// as possible values for instant `i`
    pub fn retain<TW>(&mut self, i: u32, tw: TW) -> TimePropagationResult
        where
            TW: TimeContaining<TimeSlots> + TimeOverlapping<TimeSlots> + TimeWindow<TimePoint=Timestamp>,
            TimeSlots: BitAndAssign<TW>
    {
        // checks the index now, and use unsafe get_unchecked in the fn body
        assert![(i as usize) < self.schedule.len(), "index out of bounds"];
        unsafe {
            if tw.contains(self.schedule.get_unchecked(i as usize)) {
                Ok(TimePropagation::Unchanged)
            } else if !tw.overlaps(self.schedule.get_unchecked(i as usize)) {
                Err(TimeInconsistencyError::Recovered)
            } else {
                *self.schedule.get_unchecked_mut(i as usize) &= tw;
                self.propagate_scheduling(i);
                Ok(TimePropagation::Propagated)
            }
        }
    }

    pub fn remove<TW>(&mut self, i: u32, tw: TW) -> TimePropagationResult
        where
            TW::Output: TimeContaining<TimeSlots> + TimeOverlapping<TimeSlots> + TimeWindow<TimePoint=Timestamp>,
            TimeSlots: BitAndAssign<TW::Output>,
            TW: std::ops::Not
    {
        self.retain(i, !tw)
    }

    fn propagate_scheduling(&mut self, i: Instant)
    {
        debug_assert!( i as usize <= self.schedule.len() );
        unsafe {
            self.constraints
                .constraints_from(i)
                .for_each(|k| {
                    let j = k.to() as usize;
                    *self.schedule.get_unchecked_mut(j) =
                        self.schedule.get_unchecked(j).iter()
                            .intersection(self.schedule.get_unchecked(i as usize)
                                .iter()
                                .translation(&TimeInterval::from(k)))
                            .collect();
                });
        }
    }
}

impl fmt::Debug for TimeScheduler<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.schedule.iter().enumerate()
            .try_for_each(|(i,tw)| writeln!(f, "t{} in {:?}", i, tw))
    }
}


impl fmt::Display for TimeScheduler<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.schedule.iter().enumerate()
            .try_for_each(|(i,tw)| writeln!(f, "t{} in {}", i, tw))
    }
}

impl TimeSetFormat for TimeScheduler<'_>
{
    fn format_timeset(&self, timefmt: &str) -> String {
        self.schedule.iter()
            .enumerate()
            .map(|(i,tw)| format!("t{} in {}\n", i,tw.format_timeset(timefmt)))
            .reduce(|s1,s2| s1 + &s2)
            .unwrap_or("empty time scheduler (no instant)".to_string())
    }
}

#[cfg(test)]
pub mod tests {
    use crate::graph::*;
    use crate::graph::propagation::TimePropagation::*;
    use crate::graph::TimeScheduler;

    #[test]
    fn propagation()
    {
        let mut g = TimeGraph::with_size(3);
        assert_eq!(Ok(Propagated), g.propagate(((0,1), TimeValue::from_hours(0) ..= TimeValue::from_hours(5))));
        assert_eq!(Ok(Propagated), g.propagate(((1,2), TimeValue::from_hours(7) ..= TimeValue::from_hours(10))));
        assert_eq!(Ok(Propagated), g.propagate(((0,2), TimeValue::from_hours(10) ..= TimeValue::from_hours(25))));

        // this constraint is deduced from previous one, so Unchanged result is expected
        assert_eq!(Ok(Unchanged), g.propagate(((0,2), TimeValue::from_hours(0) ..= TimeValue::from_hours(15))));

        let mut agenda = TimeScheduler::new(&g);
        assert_eq!( Ok(Propagated), agenda.set_startline(Timestamp::default()));
        assert_eq!( Ok(Propagated), agenda.set_deadline(Timestamp::from_origin(TimeValue::from_days(2))));

    }
}