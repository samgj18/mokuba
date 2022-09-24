/**
## Commutativity

Represents the commutative property. This is a marker trait that ensures
that `associativity(x, associativity(y, z)) == associativity(associativity(x, y), z`
*/
pub trait Associativity<A> {
    fn associativity(lhs: A, rhs: A) -> A;
}

/**
## Identity

Represents the identity property. This is a marker trait that ensures
that `identity(x) == x` and `identity(x) == identity(y)`
*/
pub trait Identity<A> {
    fn identity() -> A;
}
