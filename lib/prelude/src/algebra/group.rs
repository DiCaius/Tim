use crate::algebra::monoid::Monoid;

/// Models an algebraic structure represented by a monoid and an inverse operation that yields the inverse for a given element.
pub trait Group: Monoid {
    /// An operation that given an element yields the element that when applied with it on the monoid's operation results in the monoid identity.
    fn inverse(a: Self) -> Self;
}
