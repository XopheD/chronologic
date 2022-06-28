#![doc(hidden)]

use std::fmt::{Debug, Formatter};
use std::iter;
use crate::graph::{TimePropagation,TimeGraph};
use super::*;


/// # A manager of constrained time variables.
///
/// An agenda maintains a set of time windows according
/// to a set of constraints.
///
/// The set of constraints (i.e. the graph) could be shared
/// by many agenda which could differ by their chosen slots
/// (but the constraint set is always satisfied).
#[derive(Clone)]
pub struct Agenda<'a> {
    constraints: &'a TimeGraph,
    agenda : Box<[TimeSlots]>
}

impl Agenda<'_> {

    pub fn new(graph: &TimeGraph) -> Agenda
    {
        Agenda {
            constraints: graph,
            agenda: iter::repeat(TimeSlots::all()).take(graph.size() as usize).collect()
        }
    }

    #[inline]
    pub fn constraints(&self) -> &'_ TimeGraph { self.constraints }

    pub fn startline(&self) -> Timestamp {
        self.agenda.iter().map(|i| i.lower_bound()).min().unwrap()
    }

    pub fn deadline(&self) -> Timestamp {
        self.agenda.iter().map(|i| i.upper_bound()).max().unwrap()
    }

    /// Ensure that all the agenda will end at or before the deadline
    pub fn with_deadline(self, deadline: Timestamp) -> TimePropagation<Self>
    {
        // first, check if this deadline is compatible
        if self.agenda.iter().any(|tw| tw.lower_bound() > deadline) {
            TimePropagation::Recovered(self)
        } else {
            // we know that the propagation will succeed
            (0..self.agenda.len() as u32)
                .fold(TimePropagation::Unchanged(self),
                      |a, i| a.and_then(|a| a.restrict(i, ..=deadline))
                ).check_consistency()
        }
    }

    /// Ensure that all the agenda will start at or after the startline
    pub fn with_startline(self, startline: Timestamp) -> TimePropagation<Self>
    {
        // first, check if this startline is compatible
        if self.agenda.iter().any(|tw| tw.upper_bound() < startline) {
            TimePropagation::Recovered(self)
        } else {
            // we know that the propagation will succeed
            (0..self.agenda.len() as u32)
                .fold(TimePropagation::Unchanged(self),
                      |a, i| a.and_then(|a| a.restrict(i, startline..))
                ).check_consistency()
        }
    }


    /// Add a new constraint on one agenda entry
    pub fn restrict<TW>(mut self, i: u32, tw: TW) -> TimePropagation<Self>
        where TW:TimeWindow<TimePoint=Timestamp>+TimeConvex+Clone
    {
        // checks the index now, and use unsafe get_unchecked in the fn body
        assert![ (i as usize) < self.agenda.len(), "index out of bounds"];

        let reduced = self.agenda[i as usize].clone() & tw.clone();
        if reduced.is_empty() {
            TimePropagation::Recovered(self)
        } else if reduced.eq(unsafe { self.agenda.get_unchecked(i as usize) }) {
            TimePropagation::Unchanged(self)
        } else {
            let t = self.agenda.get(i as usize).unwrap();
            if tw.contains(t) {
                TimePropagation::Unchanged(self)
            } else {
                self.agenda.iter_mut()
                    .zip(self.constraints.constraints_iter(i))
                    .for_each(|(t, k)| { *t &= reduced.clone() + k; });
                TimePropagation::Propagated(self)
            }
        }
    }
}


pub struct AgendaEntryMut<'a> {
    index: u32,
    agenda: &'a mut Agenda<'a>
}

impl Debug for Agenda<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.agenda.iter().enumerate()
            .try_for_each(|(i,tw)| writeln!(f, "t{} in {:?}", i, tw))
    }
}



#[cfg(test)]
pub mod tests {
    use crate::*;
    use crate::graph::TimeGraph;
    use crate::agenda::Agenda;

    #[test]
    fn propagation() -> Result<(),Option<TimeGraph>>
    {
        let graph = TimeGraph::with_size(3)
            .add_time_constraint((0,1), TimeValue::from_ticks(0) ..= TimeValue::from_ticks(5))
            .and_then(|g| g.add_time_constraint((1,2), TimeValue::from_ticks(7) ..= TimeValue::from_ticks(10)))
            .and_then(|g| g.add_time_constraint((0,2), TimeValue::from_ticks(10) ..= TimeValue::from_ticks(25)))
            .unwrap();

        let agenda = Agenda::new(&graph)
            .with_startline(Timestamp::default()).unwrap()
            .with_deadline(Timestamp::from_origin(TimeValue::from_ticks(100))).unwrap();

        dbg!(agenda);
        Ok(())
    }
}