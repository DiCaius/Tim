use crate::category::apply::{Apply1, Apply2, Apply3, Apply4, Apply5};
use crate::category::functor::{Functor1, Functor2, Functor3, Functor4, Functor5};

/// Models an applicative functor of arity 1.
pub trait Applicative1<A>: Functor1<A> + Apply1<A> {}

/// Models an applicative functor of arity 2.
pub trait Applicative2<A, B>: Functor2<A, B> + Apply2<A, B> {}

/// Models an applicative functor of arity 3.
pub trait Applicative3<A, B, C>: Functor3<A, B, C> + Apply3<A, B, C> {}

/// Models an applicative functor of arity 4.
pub trait Applicative4<A, B, C, D>: Functor4<A, B, C, D> + Apply4<A, B, C, D> {}

/// Models an applicative functor of arity 5.
pub trait Applicative5<A, B, C, D, E>: Functor5<A, B, C, D, E> + Apply5<A, B, C, D, E> {}
