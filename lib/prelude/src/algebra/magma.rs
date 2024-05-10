/// Models a algebraic structure modelling a set with a closed binary operation.
pub trait Magma: Sized {
    /// A binary operation of form S * S -> S.
    fn op(a: Self, b: Self) -> Self;
}
