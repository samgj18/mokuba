use super::{
    laws::{Associativity, Identity},
    semigroup::Semigroup,
};

/// Represents a `monoid`. A monoid is a semigroup with an identity element.
pub trait Monoid<A>: Semigroup<A>
where
    A: Associativity<A> + Identity<A>,
{
    fn empty() -> A;
}

impl<T> Monoid<T> for T
where
    T: Associativity<T> + Identity<T>,
{
    fn empty() -> T {
        T::identity()
    }
}
