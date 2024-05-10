/// Models a functor of arity 1.
pub trait Functor1<A>: hkt::HKT1 + Sized {
    /// Applies a function to the functor context.
    fn map<X, F>(f: F) -> impl Fn(Self::With<A>) -> Self::With<X>
    where
        F: Fn(A) -> X;
}

/// Models a functor of arity 2.
pub trait Functor2<A, B>: hkt::HKT2 + Sized {
    /// Applies a function to the functor context.
    fn map1<X, F>(f: F) -> impl Fn(Self::With<A, B>) -> Self::With<X, B>
    where
        F: Fn(A) -> X;
    /// Applies a function to the functor context.
    fn map2<X, F>(f: F) -> impl Fn(Self::With<A, B>) -> Self::With<A, X>
    where
        F: Fn(B) -> X;
}

/// Models a functor of arity 3.
pub trait Functor3<A, B, C>: hkt::HKT3 + Sized {
    /// Applies a function to the functor context.
    fn map1<X, F>(f: F) -> impl Fn(Self::With<A, B, C>) -> Self::With<X, B, C>
    where
        F: Fn(A) -> X;
    /// Applies a function to the functor context.
    fn map2<X, F>(f: F) -> impl Fn(Self::With<A, B, C>) -> Self::With<A, X, C>
    where
        F: Fn(B) -> X;
    /// Applies a function to the functor context.
    fn map3<X, F>(f: F) -> impl Fn(Self::With<A, B, C>) -> Self::With<A, B, X>
    where
        F: Fn(C) -> X;
}

/// Models a functor of arity 4.
pub trait Functor4<A, B, C, D>: hkt::HKT4 + Sized {
    /// Applies a function to the functor context.
    fn map1<X, F>(f: F) -> impl Fn(Self::With<A, B, C, D>) -> Self::With<X, B, C, D>
    where
        F: Fn(A) -> X;
    /// Applies a function to the functor context.
    fn map2<X, F>(f: F) -> impl Fn(Self::With<A, B, C, D>) -> Self::With<A, X, C, D>
    where
        F: Fn(B) -> X;
    /// Applies a function to the functor context.
    fn map3<X, F>(f: F) -> impl Fn(Self::With<A, B, C, D>) -> Self::With<A, B, X, D>
    where
        F: Fn(C) -> X;
    /// Applies a function to the functor context.
    fn map4<X, F>(f: F) -> impl Fn(Self::With<A, B, C, D>) -> Self::With<A, B, C, X>
    where
        F: Fn(D) -> X;
}

/// Models a functor of arity 5.
pub trait Functor5<A, B, C, D, E>: hkt::HKT5 + Sized {
    /// Applies a function to the functor context.
    fn map1<X, F>(f: F) -> impl Fn(Self::With<A, B, C, D, E>) -> Self::With<X, B, C, D, E>
    where
        F: Fn(A) -> X;
    /// Applies a function to the functor context.
    fn map2<X, F>(f: F) -> impl Fn(Self::With<A, B, C, D, E>) -> Self::With<A, X, C, D, E>
    where
        F: Fn(B) -> X;
    /// Applies a function to the functor context.
    fn map3<X, F>(f: F) -> impl Fn(Self::With<A, B, C, D, E>) -> Self::With<A, B, X, D, E>
    where
        F: Fn(C) -> X;
    /// Applies a function to the functor context.
    fn map4<X, F>(f: F) -> impl Fn(Self::With<A, B, C, D, E>) -> Self::With<A, B, C, X, E>
    where
        F: Fn(D) -> X;
    /// Applies a function to the functor context.
    fn map5<X, F>(f: F) -> impl Fn(Self::With<A, B, C, D, E>) -> Self::With<A, B, C, D, X>
    where
        F: Fn(E) -> X;
}
