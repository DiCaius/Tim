use crate::algebra::id::ID;
use crate::algebra::semigroup::Semigroup;

/// Models a algebraic structure represented by a semigroup and am identity operation.
pub trait Monoid: Semigroup + ID {}
