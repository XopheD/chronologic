#![doc(hidden)]

use crate::graph::TimeGraph;
use super::*;


/// # A manager of constrained time variables.
///
/// An agenda maintains a set of time windows according
/// to a set of constraints.
pub struct TimeAgenda<'a> {
    graph : &'a TimeGraph,
    agenda : Vec<TimeSlot>
}

impl TimeAgenda<'_> {

    #[inline]
    pub fn new(graph: &TimeGraph) -> TimeAgenda
    {
        let mut agenda = Vec::with_capacity(graph.size() as usize);
        agenda.resize(graph.size() as usize, TimeSlot::all());
        TimeAgenda { graph, agenda }
    }
}
