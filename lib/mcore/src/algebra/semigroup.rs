use super::laws::Associativity;

/// Represents a `semigroup`. A semigroup is a set with an associative binary operation.
/// Examples of semigroups are integers under addition, or lists under concatenation.
pub trait Semigroup<A>
where
    A: Associativity<A>,
{
    fn combine(a: A, b: A) -> A;
}

impl<T> Semigroup<T> for T
where
    T: Associativity<T>,
{
    fn combine(a: T, b: T) -> T {
        T::associativity(a, b)
    }
}
