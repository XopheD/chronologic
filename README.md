# chronologic


 This crate is dedicated to reasoning about time.
 It deals with time constraints, propagate them and
 maintain an agenda of all the possible dates consistent
 with the user constraints.

 ## Time structures
 Several time structures (interval, sets) are provided
 to make easier time manipulation.

 This time data defines several operators for union, intersection,
 translation in two ways:
 * by using standard operators (`&` for intersection, `|` for unsion, `+/-` for translation)
 * by using iterator traits (see module [`iter`]) which allows time manipulation with
   saving memory allocation (no intermediate structures needed)

If we want to check that three time sets I, J, K verifies (I u J)&K is empty,
we can do it by using the operators
```rust
if ((I | J) & K).is_empty() { ... }
```
But using the iterator traits could be more efficient since no intermediate time sets need to be built:
```rust
I.into_iter().union(J).intersection(K).is_none()
```


 The module [`graph`] deals with time constraints graph and mainly provides two structures:
 * [`graph::TimeGraph`]: the time constraints graph, a time constraint is defined as an interval
 of duration between two instants, a graph could be considered as a collection of time constraints
 * [`graph::Agenda`]: the agenda maintains a set of slots (one for each instant) according to
   its time graph

## Time constraint management

The graph is represented as a (Max,+) square matrix.

This matrix is used to implement a time constraint graph as follows:
the cell (i,j) represents the lower bound of the time constraint from
this instant i to the instant j. Notice that, in this particular case,
the diagonal is filled with 0 element.

### Global Propagation Algorithm
We use a Floyd-Warshall path consistency algorithm[3]: we compute the smallest distance
between two nodes by exploring every path between them. In other words,
we extract the more constrained path.<p>
Actually, the task is not so hard because of the completeness of the graph: in this case, we
know that a local consistency ensures the global consistency. So we only study all the paths
of three nodes (the ends of the constraints and any intermediate one)[2].<p>

A first algorithm can be to iterate this calculus untill the constraints remain
stable. Another one is proposed to do the propagation is O(n<sup>3</sup>) where
n is the size of the graph[1].

### Incremental Propagation Algorithm
In order to provide a useful feedback to the user, we use a derivated algorithm
which propagate the constraints one by one, with a complexity of O(n<sup>2</sup>) at each step.
So, in the worst case, we reach a complexity of O(n<sup>4</sup>) (since
the worst case is when we have a constraint for each couple of nodes, so n<sup>2</sup> constraints).


## References
1. C. Dousson. _"Evolution Monitoring and Chronicle Recognition."_
   PhD thesis (in french), computer sciences, A.I., Université Paul Sabatier, Toulouse (1994)
1. U. Montanari. _"Networks of constraints: fundamental properties and applications to picture
     processing"_, Information sciences 7, 1974, pp 95-132.
1. C.H. Papadimitriou and K. Steiglitz. _"Combinatorial optimization: algorithms and complexity."_
	Prentice-Hall, Englewood Cliffs, N.J. 1982.</li>