use std::cmp::Ordering;
use super::*;

impl TimeGraph {
    /// Clears all the constraints
    ///
    /// The size remains the same
    pub fn reset(&mut self)
    {
        self.data.fill(-TimeValue::INFINITE);
        (0..self.size).for_each(|i| unsafe {
            // setting all constraint (i,i) to 0 (constraint from i to itseltf)
            *self.lower_mut(i,i) = TimeValue::default();
        });
    }

    /// Create a new unconstrained graph
    ///
    /// The graph contains the specified number of instants (nodes)
    /// and all the constraints are set to ]-oo,+oo[.
    pub fn with_size(size: u32) -> TimeGraph
    {
        let mut this = TimeGraph::default();
        this.resize(size);
        this
    }

    /// Number of instants (nodes) of the graph
    #[inline]
    pub fn size(&self) -> u32 { self.size }

    /// Resize the graph
    ///
    /// If the new size is smaller than the current one,
    /// they the related constraint are also removed
    /// buth the impact of their propagation remains.
    pub fn resize(&mut self, n: Instant)
    {
        self.data.resize((n * n) as usize, -TimeValue::INFINITE);
        (self.size..n).for_each(|i| unsafe {
            // setting all constraint (i,i) to 0 (constraint from i to itseltf)
            *self.lower_mut(i,i) = TimeValue::default();
        });
        self.size = n;
    }

    /// Shrinks the capacity of the graph as much as possible.
    ///
    /// The latest unconstrained instants are also removed so the size of the graph could change.
    ///
    /// It will drop down as close as possible to the length but the allocator may still
    /// inform the graph that there is space for a few more elements.
    #[inline]
    pub fn shrink_to_fit(&mut self)
    {
       self.size = (0..self.size).rev()
            .find(|&i|
                (i*i..=i*i+2*i).filter(|&x| x != i*i+i)
                    .any(|x| unsafe { !self.data.get_unchecked(x as usize).is_past_infinite()})
            ).map(|i| i+1)
            .unwrap_or(0);
        self.data.shrink_to_fit()
    }

    /// Shrinks the capacity of the graph with a lower bound.
    ///
    /// The capacity will remain at least as large as both the length and the supplied value.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    #[inline]
    pub fn shrink_to(&mut self, n: Instant) { self.data.shrink_to((n * n) as usize) }

    #[inline]
    pub(super)
    unsafe fn lower(&self, i:Instant, j:Instant) -> TimeValue {
        let ij = if i >= j { i*i + j } else { j*(j+2) - i };
        *self.data.get_unchecked(ij as usize)
    }

    #[inline]
    pub(super)
    unsafe fn lower_mut(&mut self, i:Instant, j:Instant) -> &mut TimeValue {
        let ij = if i >= j { i*i + j } else { j*(j+2) - i };
        self.data.get_unchecked_mut(ij as usize)
    }

    #[inline]
    pub fn timespan(&self, i:Instant, j:Instant) -> TimeSpan
    {
        #[allow(clippy::collapsible_else_if)]
        if i >= j {
            if i >= self.size() {
                TimeInterval::all()
            } else {
                TimeInterval {
                    lower: unsafe { *self.data.get_unchecked((i*i + j) as usize)},
                    upper: - unsafe { *self.data.get_unchecked((i*(i+2) - j) as usize) },
                }
            }
        } else {
            if j >= self.size() {
                TimeInterval::all()
            } else {
                TimeInterval {
                    lower: unsafe { *self.data.get_unchecked((j*(j+2) - i) as usize) },
                    upper: - unsafe { *self.data.get_unchecked( (j*j + i) as usize) },
                }
            }
        }
    }

    pub fn instant_cmp(&self, i:Instant, j:Instant) -> Option<Ordering>
    {
        if i >= self.size() || j >= self.size() {
            None
        } else {
            let ij = unsafe { self.lower(i,j) };
            if ij.is_strictly_positive() {
                Some(Ordering::Less)
            } else {
                let ji = unsafe { self.lower(j,i) };
                if ji.is_strictly_positive() {
                    Some(Ordering::Greater)
                } else if ij == TimeValue::default() && ji == TimeValue::default() {
                    Some(Ordering::Equal)
                } else {
                    None
                }
            }
        }
    }

    // Checks if two instants are necessarily distinct.
    #[inline]
    pub fn are_distinct_instants(&self, i:Instant, j:Instant) -> bool
    {
        self.constraint(i,j)
            .map(|k| {
                k.lower_bound().is_strictly_positive() || k.upper_bound().is_strictly_negative()
            })
            .unwrap_or(false)
    }
}

impl fmt::Debug for TimeGraph {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        writeln!(f)?;
        for i in 0..self.size() {
            write!(f,"[")?;
            for j in 0..self.size() {
                let k = format!("{:?}", unsafe { self.lower(i,j) });
                write!(f, " {:>7}", &k)?;
            };
            writeln!(f, " ]")?
        }
        Ok(())
    }
}


impl fmt::Display for TimeGraph {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut count = 0;
        writeln!(f, "[graph with {} instants]", self.size())?;
        self.iter().try_for_each(|k| { count += 1; writeln!(f, "   {}", k) })?;
        writeln!(f, "[with {} constraints]", count)
    }
}


#[cfg(test)]
mod tests {
    use crate::graph::*;
    use crate::graph::propagation::TimePropagation;

    #[test]
    pub fn init()
    {
        let mut graph = TimeGraph::with_size(3);
        graph.resize(6);
        graph.propagate(((0,1), TimeValue::from_ticks(5)..)).unwrap();
        graph.propagate(((1,3), TimeValue::from_ticks(7)..)).unwrap();
    //    graph.propagate(((1,3), ..=TimeValue::from_ticks(7))).unwrap();
        //graph.propagate(((3,2), TimeValue::from_ticks(7)..)).unwrap();
        graph.propagate(((2, 4), TimeValue::from_ticks(4)..)).unwrap();

      //  dbg!(&graph);
      //  println!("{}", graph);

        let mut graph2 = TimeGraph::with_size(7);
        assert_eq!( Ok(TimePropagation::Propagated), graph2.extend(graph.iter()) );

        graph2.shrink_to_fit();

    }
}