//! # Time constraints management

/// # A graph of non disjunctive time constraints.

/// Each node  of the graph corresponds to an instant and
/// the time constraints between two nodes are defined as a TimeInterval.
/// Each added constraint is
/// automatically propagated (through a path-consistency algorithm) and so,
/// the global consistency is always ensured.
///
/// ## Minimize the complete graph
/// A time graph is always a complete graph since one can alwys define a
/// time constraint between each couple of nodes, the default used constraint
/// is ]-&infin;, +&infin;[.
/// The <em>minimal complete graph</em> is the least constrained complete graph
/// which is in respect with all the user defined constraints. We can prove that
/// this graph is unique. For instance, the following figure shows a user defined
/// constraints graph on the left side and the corresponding minimal graph on the
/// right hand.
///
/// ![TimeGraph][timegraph-1]
///
/// ## Global Propagation Algorithm
/// We use a Floyd-Warshall path consistency algorithm
/// \[3\]: we compute the smallest distance
/// between two nodes by exploring every path between them. In other words,
/// we extract the more constrained path.<p>
/// Actually, the task is not so hard because of the completeness of the graph: in this case, we
/// know that a local consistency ensures the global consistency. So we only study all the paths
/// of three nodes (the ends of the constraints and any intermediate one) \[2\].<p>
/// Any edge is updated by intersecting the current constraint and the computed one
/// through a third node (see the following figure).
///
/// ![TimeGraph][timegraph-2]
///
/// A first algorithm can be to iterate this calculus untill the constraints remain
/// stable. Another one is proposed to do the propagation is O(n<sup>3</sup>) where
/// n is the size of the graph \[1\].
///
/// ## Incremental Propagation Algorithm
/// In order to provide a useful feedback to the user, we use a derivated algorithm
/// which propagate the constraints one by one, with a complexity of O(n<sup>2</sup>) at each step
/// (each added constraint by calling [`Self::add_time_constraint`]).
/// So, in the worst case, we reach a complexity of O(n<sup>4</sup>) (since
/// the worst case is when we have a constraint for each couple of nodes, so n<sup>2</sup> constraints).
///
/// ## Implementation: (Max,+) square matrix.
///
/// This matrix is used to implement a time constraint graph as follows:
/// the cell (i,j) represents the lower bound of the time constraint from
/// this instant i to the instant j. Notice that, in this particular case,
/// the diagonal is filled with 0 element.
///
/// As an illustration, the following figure show a time graph with the associated
/// time matrix:
///
/// ![TimeGraph][timegraph-3]
///
/// ## References
/// 1. C. Dousson. _"Evolution Monitoring and Chronicle Recognition."_
///    PhD thesis (in french), computer sciences, A.I., Université Paul Sabatier, Toulouse (1994)
/// 1. U. Montanari. _"Networks of constraints: fundamental properties and applications to picture
///      processing"_, Information sciences 7, 1974, pp 95-132.
/// 1. C.H. Papadimitriou and K. Steiglitz. _"Combinatorial optimization: algorithms and complexity."_
/// 	Prentice-Hall, Englewood Cliffs, N.J. 1982.</li>
///
#[embed_doc_image("timegraph-1", "images/timegraph-1.png")]
#[embed_doc_image("timegraph-2", "images/timegraph-2.png")]
#[embed_doc_image("timegraph-3", "images/timegraph-3.png")]

#[derive(Clone)]
pub struct TimeGraph {
    size : Instant,
    data : Vec<TimeValue>,
    // to make growing easier (i.e. without remaining the matrix order)
    // the matrix in encoded in a vector as follows:
    //
    //   0 |  3 |  8 | 15
    //   1 |  2 |  7 | 14
    //   4 |  5 |  6 | 13
    //   9 | 10 | 11 | 12
    //  16 | 17 ...
    //
    //  [i,j] = i*i + j  (if i >= j)
    //  [i,j] = j*j + 2j - i (if i <= j)
}


use std::fmt;
use super::*;
use embed_doc_image::embed_doc_image;



mod constraints;
mod propagation;
mod storage;
mod scheduler;
pub use scheduler::TimeScheduler;


/// Index of an instant in the graph
pub type Instant = u32;

pub trait TimeConstraint: TimeConvex<TimePoint=TimeValue> {
    /// The first instant of the constraint
    fn from(&self) -> Instant;
    /// The second instant of the constraint
    fn to(&self) -> Instant;

    fn equiv<K: TimeConstraint>(&self, k: &K) -> bool
    {
        if self.from() == k.from() {
            self.to() == k.to() && self.lower_bound() == k.lower_bound() && self.upper_bound() == k.upper_bound()
        } else if self.from() == k.to() {
            self.to() == k.from() && self.lower_bound() == -k.upper_bound() && self.upper_bound() == -k.lower_bound()
        } else {
            false
        }
    }
}

impl<K:TimeConstraint> From<K> for TimeSpan
{
    #[inline]
    fn from(k: K) -> Self {
        TimeInterval { lower: k.lower_bound(), upper: k.upper_bound() }
    }
}


impl<TW> TimeConstraint for ((Instant, Instant), TW)
    where
        TW:TimeConvex<TimePoint=TimeValue>
{
    #[inline]
    fn from(&self) -> Instant { self.0.0 }
    #[inline]
    fn to(&self) -> Instant { self.0.1 }
}


impl<TW> TimeConvex for ((Instant, Instant), TW)
    where
        TW:TimeConvex<TimePoint=TimeValue>
{
}

impl<TW> TimeBounds for ((Instant, Instant), TW)
    where
        TW:TimeBounds<TimePoint=TimeValue>
{
    type TimePoint = TimeValue;
    #[inline] fn is_empty(&self) -> bool { self.1.is_empty() }
    #[inline] fn is_low_bounded(&self) -> bool { self.1.is_low_bounded() }
    #[inline] fn is_up_bounded(&self) -> bool { self.1.is_up_bounded() }
    #[inline] fn lower_bound(&self) -> TimeValue { self.1.lower_bound() }
    #[inline] fn upper_bound(&self) -> TimeValue { self.1.upper_bound() }
}

/*


    pub fn constraints<'a>(&'a self, i:Instant) -> impl 'a + Iterator<Item=TimeSpan> + ExactSizeIterator + FusedIterator
    {
        struct Iter<'a>{lower:usize,upper:usize,size:usize,graph:&'a [TimeValue]}
        impl Iterator for Iter<'_> {
            type Item = TimeSpan;
            fn next(&mut self) -> Option<Self::Item> {
                if self.upper >= self.graph.len() {
                    None
                } else {
                    debug_assert!( self.lower < self.graph.len());
                    debug_assert!( self.upper < self.graph.len());
                    let tw = TimeSpan {
                        lower: unsafe { *self.graph.get_unchecked(self.lower)},
                        upper: - unsafe { *self.graph.get_unchecked(self.upper)},
                    };
                    self.lower += 1;
                    self.upper += self.size;
                    Some(tw)
                }
            }
            #[inline] fn size_hint(&self) -> (usize, Option<usize>) {
                let len = self.len(); (len,Some(len))
            }
        }
        impl ExactSizeIterator for Iter<'_> {
            #[inline] fn len(&self) -> usize { self.size - self.lower % self.size }
        }
        impl FusedIterator for Iter<'_> {}

        Iter {
            size: self.size as usize, // the number of instants in the time graph
            graph: self.data.as_slice(), //  the time constraint matrix (flattened)
            lower: (i*self.size) as usize, // the row `i` contains the lower bound
            upper: i as usize, // the column `i` contains the opposite of the upper bound
        }
    }

    #[inline]
    pub fn time_cmp(&self, i:Instant, j:Instant) -> Option<Ordering>
    {
        let k = self.constraint(i,j);
        if k.lower_bound().is_strictly_positive() {
                Some(Ordering::Less)
            } else if k.upper_bound().is_strictly_negative() {
                Some(Ordering::Greater)
            } else if k.is_singleton() {
                debug_assert_eq!(k.lower_bound(), TimeValue::default());
                Some(Ordering::Equal)
            } else {
                None
            }
        }
    }

    // Checks if two instants are necessarily distinct.
    #[inline]
    pub fn are_distinct(&self, i:Instant, j:Instant) -> bool
    {
        self.constraint(i,j)
            .map(|k| {
                k.lower_bound().is_strictly_positive() || k.upper_bound().is_strictly_negative()
            })
            .unwrap_or(false)
    }


    
    pub fn add_time_constraint<TW>(&mut self, (i,j):(Instant,Instant), k: TW) -> TimePropagationResult
        where
            TW:TimeConvex<TimePoint=TimeValue>
    {
        if self.size <= max(i,j) {
            // si i ou j n'était pas dans le graphe
            // on n'aura rien à propager
            self.resize(max(i,j)+1);
            *self.get_mut(i,j) = k.lower_bound();
            *self.get_mut(j,i) = -k.upper_bound();
            Ok(TimePropagation::Propagated)
        } else {
            let lower = unsafe { self.lower_constraint_unchecked(i,j) };
            if k.lower_bound() <= lower {
                //- la contrainte basse ne change pas
                let upper = - unsafe { self.lower_constraint_unchecked(j,i) };
                if k.upper_bound() >= upper {
                    //- la contrainte sup. ne change pas non plus
                    Ok(TimePropagation::Unchanged)
                } else if k.upper_bound() < lower {
                    //- la contrainte sup est inconsistante
                    Err(TimeInconsistencyError::Recovered)
                } else {
                    //- OK, on propage la contrainte sup (et c'est tout)
                    *self.get_mut(j,i) = -k.upper_bound();
                    self.propagate_lower_bound(j,i);
                    Ok(TimePropagation::Propagated)
                }
            } else {
                //- la contrainte basse change
                let upper = - unsafe { self.lower_constraint_unchecked(j,i) };
                if (k.lower_bound() > upper) || (k.lower_bound() < lower) {
                    //- la contrainte est inconsistante
                    Err(TimeInconsistencyError::Recovered)
                } else {
                    //- OK, on peut propager la borne inf
                    *self.get_mut(i,j) = k.lower_bound();
                    self.propagate_lower_bound(i,j);
                    if k.upper_bound() < upper {
                        //- la contrainte sup. change aussi
                        *self.get_mut(j,i) = -k.upper_bound();
                        self.propagate_lower_bound(j,i);
                    }
                    Ok(TimePropagation::Propagated)
                }
            }
        }
    }
    
    /// Propagate a new lower constraint
    ///
    /// If the new constraint is inconsistent with the graph,
    /// it remains unchanged and an error is returned.
    ///
    /// If the new constraint is consistent, then it will be propagated.
    /// true is returned if something change and false is returned if
    /// nothing changed (i.e. if the constraint was already deduced the graph)
    pub fn add_lower_time_constraint(&mut self, i:Instant, j:Instant, lower:TimeValue) -> TimePropagationResult
    {
        if self.size <= max(i,j) {
            self.resize(max(i,j)+1);
            *self.get_mut(i,j) = lower;
            Ok(TimePropagation::Propagated)
        } else {
            if lower <= unsafe { self.lower_constraint_unchecked(i,j) } {
                //- la contrainte basse ne change pas
                Ok(TimePropagation::Unchanged)
            } else if lower > unsafe { -self.lower_constraint_unchecked(j,i) } {
                //- la contrainte sup est inconsistante
                Err(TimeInconsistencyError::Recovered)
            } else {
                //- OK, on peut propager la borne inf
                *self.get_mut(i,j) = lower;
                self.propagate_lower_bound(j,i);
                Ok(TimePropagation::Propagated)
            }
        }
    }
    
    #[inline]
    pub fn add_upper_time_constraint(&mut self, i:Instant, j:Instant, k:TimeValue) -> TimePropagationResult {
        self.add_lower_time_constraint(j, i, -k)
    }

    #[inline]
    fn get_mut(&mut self, i:Instant, j:Instant) -> &mut TimeValue
    {
        &mut (self.data[(i*self.size+j) as usize])
    }

    #[inline]
    unsafe fn get_unchecked(&self, i:Instant, j:Instant) -> &TimeValue
    {
        self.data.get_unchecked((i*self.size+j) as usize)
    }

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, i:Instant, j:Instant) -> &mut TimeValue
    {
        self.data.get_unchecked_mut((i*self.size+j) as usize)
    }


    fn propagate_lower_bound(&mut self, io:Instant, jo:Instant)
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
        for i in 0..self.size {
            let val: TimeValue = unsafe {
                self.lower_constraint_unchecked(i, io) + self.lower_constraint_unchecked(io, jo)
            };
            if val > unsafe { self.lower_constraint_unchecked(i, jo) } {
                unsafe { *self.get_unchecked_mut(i, jo) = val; }
            }
        }
        
        //- boucle autour du noeud jo
        //- C(j,i) <- C(j,i) & (C(j,jo) + C(jo,i))
        for j in 0..self.size {
            for i in 0..self.size {
                let val: TimeValue = unsafe {
                    self.lower_constraint_unchecked(j, jo)+self.lower_constraint_unchecked(jo, i)
                };
                if val > unsafe { self.lower_constraint_unchecked(j, i) } {
                    unsafe { *self.get_unchecked_mut(j, i) = val; }
                }
            }
        }
    }

    pub fn merge(&mut self, mut rhs: TimeGraph) -> TimePropagationResult
    {
        if self.size < rhs.size {
            std::mem::swap(self, &mut rhs)
        }
        let mut stgchanged = false;
        if self.size == rhs.size {
            // the two graphs have the same size so the bounds
            // are in the same place in the flattened matrix
            self.data.iter_mut()
                .zip(rhs.data.iter())
                .for_each(|(a,b)| if *a < *b { *a = *b; stgchanged = true; })
        } else {
            for i in 0..rhs.size {
                for j in 0..rhs.size {
                    let a = unsafe { self.get_unchecked_mut(i,j) };
                    let b = unsafe { rhs.get_unchecked(i,j) };
                    if *a < *b { *a = *b; stgchanged = true; }
                }
            }
        }
        if stgchanged {
            self.propagate()?;
            Ok(TimePropagation::Propagated)
        } else {
            Ok(TimePropagation::Unchanged)
        }
    }

    pub fn add_time_constraints<TW,I>(&mut self, iter:I) -> TimePropagationResult
        where
            TW:TimeConvex+TimeWindow<TimePoint=TimeValue>,
            I: IntoIterator<Item=((Instant, Instant), TW)>
    {
        iter.into_iter()
            .for_each(|((i,j), tw)| {
                let lower = self.get_mut(i,j);
                if *lower < tw.lower_bound() {
                    *lower = tw.lower_bound();
                }

                // SAFETY: if lower exists, the upper does...
                let upper = unsafe{self.get_unchecked_mut(j,i)};
                if *upper < -tw.upper_bound() {
                    *upper = -tw.upper_bound();
                }
            });
        self.propagate()
    }

    /// Global propagation in O(n<sup>3</sup>).
    ///
    /// All the graph constraints are propagated.
    fn propagate(&mut self) -> TimePropagationResult
    {
        let mut stgchanged = false;
        for k in 0..self.size {
            for i in 0..self.size {
                for j in 0..self.size {
                    let val: TimeValue = unsafe {
                        self.lower_constraint_unchecked(i, k)+self.lower_constraint_unchecked(k, j)
                    };
                    if val > unsafe { self.lower_constraint_unchecked(i, j) } {
                        unsafe { *self.get_unchecked_mut(i, j) = val; }
                        stgchanged = true;
                    }
                }
                if unsafe { self.lower_constraint_unchecked(i,i) }.is_strictly_positive() {
                    return Err(TimeInconsistencyError::Fatal)
                }
            }
        }
        if stgchanged {
            Ok(TimePropagation::Propagated)
        } else {
            Ok(TimePropagation::Unchanged)
        }
    }
}


impl<TW> FromIterator<((Instant,Instant),TW> for TimeGraph
    where
        TW: TimeConvex<TimePoint=TimeValue>
{
    fn from_iter<I: IntoIterator<Item=((Instant,Instant))>>(iter: I) -> TimeGraph
    {
        let mut graph = TimeGraph::with_size(32);
        
        for k in iter {
            match graph.add_time_constraint(k.start, k.end, k.constraint) {
                Ok(result) => (),
                Err(()) => {
                    graph.clear(); 
                    return graph }
                }
            }
            
            graph
        }
    }
}





impl fmt::Display for TimeGraph
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    {
        for i in 0..self.size {
            for j in 0..i {
                let k : TimeInterval<_> = self.constraint(i, j).unwrap().into();
                if k != TimeInterval::all() {
                    if k.upper_bound().is_positive() {
                        writeln!(formatter,"t{} - t{} in {};", j, i, k)?;
                    } else {
                        writeln!(formatter,"t{} - t{} in {};", i, j, -k)?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl fmt::Debug for TimeGraph
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    {
        if self.size == 0 {
            writeln!(formatter, "[]")
        } else {
            for i in 0..self.size {
                write!(formatter,"[{:?}", unsafe { self.lower_constraint_unchecked(i, 0) })?;
                for j in 1..self.size {
                    write!(formatter,",{:?}", unsafe { self.lower_constraint_unchecked(i, j) })?;
                }
                writeln!(formatter,"]")?;
            }
            Ok(())
        }
    }
}



#[cfg(test)]
pub mod tests {
    use crate::*;
    use crate::graph::TimeGraph;

    #[test]
    fn propagation() -> Result<(),Option<TimeGraph>>
    {
        let mut g = TimeGraph::with_size(3);
        g.add_time_constraint((0,1), TimeValue::from_ticks(0)..= TimeValue::from_ticks(5));
        g.add_time_constraint((1,2), TimeValue::from_ticks(7)..= TimeValue::from_ticks(10));
        g.add_time_constraint((0,2), TimeValue::from_ticks(10)..=TimeValue::from_ticks(25));

        dbg!(&g);

        let mut g = TimeGraph::with_size(3);
        g.add_time_constraints(vec![
                ((0,1), TimeValue::from_ticks(0)..= TimeValue::from_ticks(5)),
                ((1,2), TimeValue::from_ticks(1)..= TimeValue::from_ticks(6)),
                ((0,2), TimeValue::from_ticks(10)..=TimeValue::from_ticks(25)),
        ]);
        dbg!(g);
        Ok(())
    }
}
*/