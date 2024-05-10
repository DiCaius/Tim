use hkt::{HKT1, HKT2, HKT3, HKT4, HKT5};
use hkt_macro::{HKT1, HKT2, HKT3, HKT4, HKT5};

#[derive(HKT1)]
enum A<X> {
    T1(X),
}

#[derive(HKT2)]
enum B<X, Y> {
    T1(X),
    T2(Y),
}

#[derive(HKT3)]
enum C<X, Y, Z> {
    T1(X),
    T2(Y),
    T3(Z),
}

#[derive(HKT4)]
enum D<X, Y, Z, W> {
    T1(X),
    T2(Y),
    T3(Z),
    T4(W),
}

#[derive(HKT5)]
enum E<X, Y, Z, W, O> {
    T1(X),
    T2(Y),
    T3(Z),
    T4(W),
    T5(O),
}

fn main() {}
