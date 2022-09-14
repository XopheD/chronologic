use std::error::Error;
use std::fmt;
use std::mem::swap;
use crate::graph::{Instant, TimeConstraint, TimeGraph};
use crate::TimeValue;

pub type TimePropagationResult = Result<TimePropagation,TimeInconsistencyError>;

/// Result of a propagation operation inside
/// a constraint set (graph or agenda).
#[derive(Clone,Copy,Debug)]
pub enum TimePropagation {
    /// The propagation is done without modifying the initial data
    ///
    /// Typically, it is the case when we attempt to add a new time
    /// constraint which is always ensured by the previous ones.
    Unchanged,

    /// The propagation is done and modifies the previous constraint
    Propagated,
}

#[derive(Clone,Copy,Debug)]
pub enum TimeInconsistencyError {
    /// The propagation failed but the original data are restored
    /// so the graph remains unchanged.
    Recovered,

    /// The propagation failed and the original data are corrupted
    ///
    /// Using corrupted data could lead to unexpected behavior (basically,
    /// wrong further time propagation).
    /// The graph is emptied.
    Fatal,
}

impl Error for TimeInconsistencyError { }

impl fmt::Display for TimeInconsistencyError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("inconsistency detected when propagating time constraints")
    }
}

impl TimeGraph
{
    pub fn propagate<K:TimeConstraint>(&mut self, k: K) -> TimePropagationResult
    {
        if k.is_empty() {
            Err(TimeInconsistencyError::Recovered)
        } else {
            let max = k.from().max(k.to());
            if self.size() <= max {
                // si i ou j n'était pas dans le graphe, on n'aura rien à propager
                self.resize(max + 1);
                unsafe {
                    // SAFETY: we have just resize the graph for that
                    *self.lower_mut(k.from(), k.to()) = k.lower_bound();
                    *self.lower_mut(k.to(), k.from()) = -k.upper_bound();
                }
                Ok(TimePropagation::Propagated)
            } else {
                unsafe {
                    // SAFETY: we check the size of the graph just before entering here
                    if k.lower_bound() <= self.lower(k.from(), k.to()) {
                        //- la contrainte ij ne change pas
                        if -k.upper_bound() <= self.lower(k.to(), k.from()) {
                            //- la contrainte ji ne change pas non plus, c’est fini !
                            Ok(TimePropagation::Unchanged)
                        } else if self.lower(k.from(), k.to()) > k.upper_bound() {
                            //- la nouvelle contrainte ji est inconsistante
                            Err(TimeInconsistencyError::Recovered)
                        } else {
                            //- OK, on propage la contrainte ji (et c'est tout)
                            *self.lower_mut(k.to(), k.from()) = -k.upper_bound();
                            self.propagate_lower_bound(k.to(), k.from());
                            Ok(TimePropagation::Propagated)
                        }
                    } else {
                        //- la contrainte ij change
                        if self.lower(k.to(), k.from()) > -k.lower_bound() {
                            //- la nouvelle contrainte ij est inconsistante
                            Err(TimeInconsistencyError::Recovered)
                        } else {
                            //- OK, on peut propager la contrainte ij
                            *self.lower_mut(k.from(), k.to()) = k.lower_bound();
                            self.propagate_lower_bound(k.from(), k.to());
                            if self.lower(k.to(), k.from()) < -k.upper_bound() {
                                //- la contrainte ji. change aussi
                                *self.lower_mut(k.to(), k.from()) = -k.upper_bound();
                                self.propagate_lower_bound(k.to(), k.from());
                            }
                            Ok(TimePropagation::Propagated)
                        }
                    }
                }
            }
        }
    }

    /// Merge two timegraphs
    pub fn merge(&mut self, mut graph: TimeGraph) -> TimePropagationResult
    {
        let mut change = false;
        let mut swapped = false;
        if self.size() < graph.size() { swap(self, &mut graph); swapped = true; }
        self.data.iter_mut()
            .zip(graph.data.into_iter())
            .for_each(|(a,b)| if *a < b { *a = b; change = true; });
        if change {
            self.global_propagation()
        } else if swapped {
            Ok(TimePropagation::Propagated)
        } else {
            Ok(TimePropagation::Unchanged)
        }
    }

    unsafe fn propagate_lower_bound(&mut self, io:Instant, jo:Instant)
    {
        //- propagation incrementale
        //- on suppose que la table des contraintes est a jour
        //- (en nombre d'instants et en propagation des contraintes) SAUF (io,jo).
        //- On applique l'algorithme de propagation globale sur les noeuds
        //- qui nous interesse (donc io et jo).
        //- La complexite de cet algorithme est exactement en n2+n.

        //- ATTENTION: si la table n'etait pas propagee avant l'ajout de la
        //- contrainte (io,jo), l'algo. fera n'importe quoi
        //- (en tout cas, certainement pas la propagation complete)

        //- boucle autour du noeud io
        // C(i,jo) <- max (C(i,jo), (C(i,io) + C(io,jo)))
        {
            let io_jo =  self.lower(io, jo);
            for i in 0..self.size() {
                let val: TimeValue = self.lower(i,io)  + io_jo;
                let k = self.lower_mut(i, jo);
                if val > *k { *k = val; }
            }
        }

        //- boucle autour du noeud jo
        //- C(j,i) <- C(j,i) & (C(j,jo) + C(jo,i))
        for j in 0..self.size() {
            let j_jo = self.lower(j,jo);
            for i in 0..self.size() {
                let val: TimeValue = j_jo + self.lower(jo,i);
                let k = self.lower_mut(j, i);
                if val > *k { *k = val; }
            }
        }
    }

    /// Global propagation in O(n<sup>3</sup>).
    ///
    /// All the graph constraints are propagated.
    fn global_propagation(&mut self) -> TimePropagationResult
    {
        for k in 0..self.size() {
            for i in 0..self.size() {
                for j in 0..self.size() {
                    let val: TimeValue = unsafe { self.lower(i,k)+self.lower(k,j) };
                    let x = unsafe { self.lower_mut(i,j) };
                    if val > *x { *x = val; }
                }
                if unsafe { self.lower(i,i) }.is_strictly_positive() {
                    self.size = 0;
                    return Err(TimeInconsistencyError::Fatal)
                }
            }
        }
        Ok(TimePropagation::Propagated)
    }

    /// Add several constraints in one shot
    ///
    /// If this set of constraints are inconsistent with the graph,
    /// there is no possible recovery and the graph is definitively corrupted
    pub fn extend<I,K>(&mut self, iter:I) -> TimePropagationResult
        where
            K: TimeConstraint,
            I: IntoIterator<Item=K>
    {
        let mut iter = iter.into_iter();
        match iter.size_hint() {
            (_, Some(0)) => {
                Ok(TimePropagation::Unchanged)
            }
            (_, Some(1)) => {
                match iter.next() {
                    None => Ok(TimePropagation::Unchanged),
                    Some(k) => self.propagate(k)
                }
            }
            _ => {
                iter.into_iter()
                    .for_each(|k| {
                        let max = k.from().max(k.to());
                        if max >= self.size() { self.resize(max+1) }

                        let lower = unsafe { self.lower_mut(k.from(), k.to()) };
                        if *lower < k.lower_bound() {
                            *lower = k.lower_bound();
                        }
                        // SAFETY: if lower exists (checked just above), the upper does...
                        let upper = unsafe { self.lower_mut(k.to(), k.from()) };
                        if *upper < -k.upper_bound() {
                            *upper = -k.upper_bound();
                        }
                    });
                self.global_propagation()
            }
        }
    }

}
