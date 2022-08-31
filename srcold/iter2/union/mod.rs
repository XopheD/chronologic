use crate::TimeWindow;

pub mod iter;


/// Make union in place
pub trait TimeUnion<Rhs>
    where
        Self: TimeWindow
{
    type Output;
    fn union(self, rhs: Rhs) -> Self::Output;
}
