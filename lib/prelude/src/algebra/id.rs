/// Models an algebraic structure represented by a set with an identity operation.
pub trait ID {
    /// The identity for an algebraic structure.
    const ID: Self;
}
