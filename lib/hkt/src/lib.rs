/// Emulates a Higher-Kinded Type of arity 1.
pub trait HKT1 {
    type A;
    #[rustfmt::skip]
    type With<T>: HKT1<A = T>
        + HKT1<With<Self::A> = Self>
        + HKT1<With<T> = Self::With<T>>;
}

/// Emulates a Higher-Kinded Type of arity 2.
pub trait HKT2 {
    type A;
    type B;
    type With<T1, T2>: HKT2<A = T1, B = T2>
        + HKT2<With<Self::A, Self::B> = Self>
        + HKT2<With<T1, T2> = Self::With<T1, T2>>;
}

/// Emulates a Higher-Kinded Type of arity 3.
pub trait HKT3 {
    type A;
    type B;
    type C;
    type With<T1, T2, T3>: HKT3<A = T1, B = T2, C = T3>
        + HKT3<With<Self::A, Self::B, Self::C> = Self>
        + HKT3<With<T1, T2, T3> = Self::With<T1, T2, T3>>;
}

/// Emulates a Higher-Kinded Type of arity 4.
pub trait HKT4 {
    type A;
    type B;
    type C;
    type D;
    type With<T1, T2, T3, T4>: HKT4<A = T1, B = T2, C = T3, D = T4>
        + HKT4<With<Self::A, Self::B, Self::C, Self::D> = Self>
        + HKT4<With<T1, T2, T3, T4> = Self::With<T1, T2, T3, T4>>;
}

/// Emulates a Higher-Kinded Type of arity 5.
pub trait HKT5 {
    type A;
    type B;
    type C;
    type D;
    type E;
    type With<T1, T2, T3, T4, T5>: HKT5<A = T1, B = T2, C = T3, D = T4, E = T5>
        + HKT5<With<Self::A, Self::B, Self::C, Self::D, Self::E> = Self>
        + HKT5<With<T1, T2, T3, T4, T5> = Self::With<T1, T2, T3, T4, T5>>;
}
