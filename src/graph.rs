use super::*;
use std::cmp::{min,max};
use embed_doc_image::embed_doc_image;

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
/// ## Incremental Propagation Algorithm</h3>
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
/// 1. C. Dousson. _"Suivi d'évolutions et reconnaissance de chroniques."_
///    Thèse d'informatique, option I.A., Université Paul Sabatier, Toulouse (1994)
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
    size : u32,
    data : Vec<TimeValue>
}

#[derive(Clone,Default,Eq,PartialEq)]
pub struct TimeConstraint {
    pub start: u32,
    pub end: u32,
    pub constraint: TimeSpan
}

impl TimeGraph {
    
    /// Create a new unconstrained graph
    ///
    /// The graph contains the specified number of instants (nodes)
    /// and all the constraints are set to ]-oo,+oo[.
    pub fn with_size(size:u32) -> TimeGraph
    {
        let mut data = Vec::with_capacity(size as usize * size as usize);
        data.resize(size as usize * size as usize, -TimeValue::INFINITE);
        (0..size as usize).for_each(|x| data[(x*size as usize)+x] = Default::default());
        TimeGraph { size, data }
    }
    
    /// Number of nodes of the graph
    #[inline]
    pub fn size(&self) -> u32 { self.size }
    
    #[inline]
    pub fn lower_constraint(&self, i:u32, j:u32) -> TimeValue 
    {
        self.data[(i*self.size+j) as usize] 
    }
    
    #[inline]
    pub fn upper_constraint(&self, i:u32, j:u32) -> TimeValue 
    {
        - self.lower_constraint(j, i)
    }

    #[inline]
    pub fn constraint(&self, i:u32, j:u32) -> TimeSpan
    {
        TimeSpan {
            lower: self.lower_constraint(i, j),
            upper: self.upper_constraint(i, j)
        }
    }
    
    fn get_mut(&mut self, i:u32, j:u32) -> &mut TimeValue 
    {
        &mut (self.data[(i*self.size+j) as usize])
    }
    
    /// Checks if two instants are simultaneous.
    #[inline]
    pub fn are_simultaneous(&self, i:u32, j:u32) -> bool
    {
        self.lower_constraint(i,j).is_zero() && self.lower_constraint(j,i).is_zero()
    }

    #[inline]
    pub fn are_ordered(&self, i:u32, j:u32) -> bool
    {
        self.lower_constraint(i,j).is_positive()
    }
    
    /// Resize the graph
    ///
    /// If the new size is smaller than the current one,
    /// they the related constraint are also removed
    /// buth the impact of their propagation remains.
    pub fn resize(&mut self, size:u32)
    {
        let mut g = TimeGraph::with_size(size);
        if self.size != g.size {
            let size:u32= min(self.size,g.size);
            for i in 0..size {
                for j in 0..size {
                    g.data[(i*g.size+j) as usize] = self.lower_constraint(i,j);
                }
            }
            *self = g;
        }
    }
    
    // Checks if two instants are necessarily distinct.
    #[inline]
    pub fn are_distinct(&self, i:u32, j:u32) -> bool
    {
        if self.lower_constraint(i,j).is_strictly_positive() {
            self.lower_constraint(j,i).is_strictly_negative()
        } else if self.lower_constraint(i,j).is_strictly_negative() {
            self.lower_constraint(j,i).is_strictly_positive()
        } else {
            false
        }
    }
    
    pub fn add_time_constraint(&mut self, i:u32, j:u32, k: &TimeSpan) -> Result<bool,()>
    {
        if self.size <= max(i,j) {
            // si i ou j n'était pas dans le graphe
            // on n'aura rien à propager
            self.resize(max(i,j)+1);
            *self.get_mut(i,j) = k.lower_bound();
            *self.get_mut(j,i) = -k.upper_bound();
            Ok(true)
        } else {
            let lower = self.lower_constraint(i,j);
            if k.lower_bound() <= lower {
                //- la contrainte basse ne change pas
                let upper = -self.lower_constraint(j,i);
                if k.upper_bound() >= upper {
                    //- la contrainte sup. ne change pas non plus
                    Ok(false)
                } else if k.upper_bound() < lower {
                    //- la contrainte sup est inconsistante
                    Err(())
                } else {
                    //- OK, on propage la contrainte sup (et c'est tout)
                    *self.get_mut(j,i) = -k.upper_bound();
                    self.propagate_lower_bound(j,i);
                    Ok(true)
                }
            } else {
                //- la contrainte basse change
                let upper = -self.lower_constraint(j,i);
                if (k.lower_bound() > upper) || (k.lower_bound() < lower) {
                    //- la contrainte est inconsistante
                    Err(())
                } else {
                    //- OK, on peut propager la borne inf
                    *self.get_mut(i,j) = k.lower_bound();
                    self.propagate_lower_bound(i,j);
                    if k.upper_bound() < upper {
                        //- la contrainte sup. change aussi
                        *self.get_mut(j,i) = -k.upper_bound();
                        self.propagate_lower_bound(j,i);
                    }
                    Ok(true)
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
    pub fn add_lower_time_constraint(&mut self, i:u32, j:u32, lower:TimeValue) -> Result<bool,()>
    {
        if self.size <= max(i,j) {
            self.resize(max(i,j)+1);
            *self.get_mut(i,j) = lower;
            Ok(true)
        } else {
            if lower <= self.lower_constraint(i,j) {
                //- la contrainte basse ne change pas
                Ok(false)
            } else if lower > -self.lower_constraint(j,i) {
                //- la contrainte sup est inconsistante
                Err(())
            } else {
                //- OK, on peut propager la borne inf
                *self.get_mut(i,j) = lower;
                self.propagate_lower_bound(j,i);
                Ok(true)
            }
        }
    }
    
    #[inline]
    pub fn add_upper_time_constraint(&mut self, i:u32, j:u32, k:TimeValue) -> Result<bool,()>
    {
        self.add_lower_time_constraint(j, i, -k)
    }
    
    fn propagate_lower_bound(&mut self, io:u32, jo:u32) 
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
            let val: TimeValue = self.lower_constraint(i, io)+self.lower_constraint(io, jo);
            if val > self.lower_constraint(i, jo) {
                *self.get_mut(i, jo) = val;
            }
        }
        
        //- boucle autour du noeud jo
        //- C(j,i) <- C(j,i) & (C(j,jo) + C(jo,i))
        for j in 0..self.size {
            for i in 0..self.size {
                let val: TimeValue = self.lower_constraint(j, jo)+self.lower_constraint(jo, i);
                if val > self.lower_constraint(j, i) {
                    *self.get_mut(j, i) = val;
                }
            }
        }
    }
    
    /*
    /// Global propagation in O(n<sup>3</sup>).
    ///
    /// All the graph constraints are propagated.
    pub fn propagateLowerBounds(&mut self) -> Result<bool>
    {
        for (int k = 0; k < size(); ++k) {
            for (int i = 0; i < size(); ++i) {
                for (int j = 0; j < size(); ++j) {
                    matrix.setMax(i, j, TimeValue.add(matrix.get(i,k), matrix.get(k,j)));
                }
                if (matrix.get(i,i) > 0) {
                    throw new TimeInconsistencyException();
                }
            }
        }
        
    }*/
}

/*
impl FromIterator<TimeConstraint> for TimeGraph 
{
    fn from_iter<I: IntoIterator<Item=TimeConstraint>>(iter: I) -> TimeGraph 
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
*/



use std::fmt;

impl fmt::Display for TimeGraph
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    {
        for i in 0..self.size {
            for j in 0..i {
                let k = self.constraint(i, j);
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
                write!(formatter,"[{:?}",self.lower_constraint(i, 0))?;
                for j in 1..self.size {
                    write!(formatter,",{:?}",self.lower_constraint(i, j))?;
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
    fn propagation() -> Result<(),()>
    {
        let mut graph = TimeGraph::with_size(3);
        graph.add_time_constraint(0, 1, &TimeInterval::new(TimeValue::from_ticks(0), TimeValue::from_ticks(5)).unwrap())?;
        graph.add_time_constraint(1, 2, &TimeInterval::new(TimeValue::from_ticks(7), TimeValue::from_ticks(10)).unwrap())?;
        graph.add_time_constraint(0, 2, &TimeInterval::new(TimeValue::from_ticks(10), TimeValue::from_ticks(25)).unwrap())?;
        Ok(())
    }
}