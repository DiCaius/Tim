/// Models properties for an applicative functor of arity 1.
pub trait Apply1<A>: hkt::HKT1 + Sized {
    /// Apply a lifted function to the functor context.
    fn apply<X, F>(f: Self::With<F>) -> impl Fn(Self::With<A>) -> Self::With<X>
    where
        F: Fn(A) -> X;
    /// Lifts a value to the functor context.
    fn pure<X>(value: X) -> Self::With<X>;
}

/// Models properties for an applicative functor of arity 2.
pub trait Apply2<A, B>: hkt::HKT2 + Sized {
    /// Apply a lifted function to the functor context.
    fn apply1<X, F>(f: Self::With<F, B>) -> impl Fn(Self::With<A, B>) -> Self::With<X, B>
    where
        F: Fn(A) -> X;
    /// Apply a lifted function to the functor context.
    fn apply2<X, F>(f: Self::With<A, F>) -> impl Fn(Self::With<A, B>) -> Self::With<A, X>
    where
        F: Fn(B) -> X;
    /// Lifts a value to the functor context.
    fn pure1<X>(value: X) -> Self::With<X, B>;
    /// Lifts a value to the functor context.
    fn pure2<X>(value: X) -> Self::With<A, X>;
}

/// Models properties for an applicative functor of arity 3.
pub trait Apply3<A, B, C>: hkt::HKT3 + Sized {
    /// Apply a lifted function to the functor context.
    fn apply1<X, F>(f: Self::With<F, B, C>) -> impl Fn(Self::With<A, B, C>) -> Self::With<X, B, C>
    where
        F: Fn(A) -> X;
    /// Apply a lifted function to the functor context.
    fn apply2<X, F>(f: Self::With<A, F, C>) -> impl Fn(Self::With<A, B, C>) -> Self::With<A, X, C>
    where
        F: Fn(B) -> X;
    /// Apply a lifted function to the functor context.
    fn apply3<X, F>(f: Self::With<A, B, F>) -> impl Fn(Self::With<A, B, C>) -> Self::With<A, B, X>
    where
        F: Fn(C) -> X;
    /// Lifts a value to the functor context.
    fn pure1<X>(value: X) -> Self::With<X, B, C>;
    /// Lifts a value to the functor context.
    fn pure2<X>(value: X) -> Self::With<A, X, C>;
    /// Lifts a value to the functor context.
    fn pure3<X>(value: X) -> Self::With<A, B, X>;
}

/// Models properties for an applicative functor of arity 4.
pub trait Apply4<A, B, C, D>: hkt::HKT4 + Sized {
    /// Apply a lifted function to the functor context.
    fn apply1<X, F>(f: Self::With<F, B, C, D>) -> impl Fn(Self::With<A, B, C, D>) -> Self::With<X, B, C, D>
    where
        F: Fn(A) -> X;
    /// Apply a lifted function to the functor context.
    fn apply2<X, F>(f: Self::With<A, F, C, D>) -> impl Fn(Self::With<A, B, C, D>) -> Self::With<A, X, C, D>
    where
        F: Fn(B) -> X;
    /// Apply a lifted function to the functor context.
    fn apply3<X, F>(f: Self::With<A, B, F, D>) -> impl Fn(Self::With<A, B, C, D>) -> Self::With<A, B, X, D>
    where
        F: Fn(C) -> X;
    /// Apply a lifted function to the functor context.
    fn apply4<X, F>(f: Self::With<A, B, C, F>) -> impl Fn(Self::With<A, B, C, D>) -> Self::With<A, B, C, X>
    where
        F: Fn(D) -> X;
    /// Lifts a value to the functor context.
    fn pure1<X>(value: X) -> Self::With<X, B, C, D>;
    /// Lifts a value to the functor context.
    fn pure2<X>(value: X) -> Self::With<A, X, C, D>;
    /// Lifts a value to the functor context.
    fn pure3<X>(value: X) -> Self::With<A, B, X, D>;
    /// Lifts a value to the functor context.
    fn pure4<X>(value: X) -> Self::With<A, B, C, X>;
}

/// Models properties for an applicative functor of arity 5.
pub trait Apply5<A, B, C, D, E>: hkt::HKT5 + Sized {
    /// Apply a lifted function to the functor context.
    fn apply1<X, F>(f: Self::With<F, B, C, D, E>) -> impl Fn(Self::With<A, B, C, D, E>) -> Self::With<X, B, C, D, E>
    where
        F: Fn(A) -> X;
    /// Apply a lifted function to the functor context.
    fn apply2<X, F>(f: Self::With<A, F, C, D, E>) -> impl Fn(Self::With<A, B, C, D, E>) -> Self::With<A, X, C, D, E>
    where
        F: Fn(B) -> X;
    /// Apply a lifted function to the functor context.
    fn apply3<X, F>(f: Self::With<A, B, F, D, E>) -> impl Fn(Self::With<A, B, C, D, E>) -> Self::With<A, B, X, D, E>
    where
        F: Fn(C) -> X;
    /// Apply a lifted function to the functor context.
    fn apply4<X, F>(f: Self::With<A, B, C, F, E>) -> impl Fn(Self::With<A, B, C, D, E>) -> Self::With<A, B, C, X, E>
    where
        F: Fn(D) -> X;
    /// Apply a lifted function to the functor context.
    fn apply5<X, F>(f: Self::With<A, B, C, D, F>) -> impl Fn(Self::With<A, B, C, D, E>) -> Self::With<A, B, C, D, X>
    where
        F: Fn(E) -> X;
    /// Lifts a value to the functor context.
    fn pure1<X>(value: X) -> Self::With<X, B, C, D, E>;
    /// Lifts a value to the functor context.
    fn pure2<X>(value: X) -> Self::With<A, X, C, D, E>;
    /// Lifts a value to the functor context.
    fn pure3<X>(value: X) -> Self::With<A, B, X, D, E>;
    /// Lifts a value to the functor context.
    fn pure4<X>(value: X) -> Self::With<A, B, C, X, E>;
    /// Lifts a value to the functor context.
    fn pure5<X>(value: X) -> Self::With<A, B, C, D, X>;
}
