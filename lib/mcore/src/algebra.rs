// Very basic first attempt at a somewhat functional traits system 🥹

pub mod laws {
    pub trait Associativity<A> {
        fn associativity(lhs: A, rhs: A) -> A;
    }

    pub trait Identity<A> {
        fn identity() -> A;
    }
}

pub mod typeclass {
    use super::laws::{Associativity, Identity};
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

    impl Associativity<u32> for u32 {
        fn associativity(lhs: u32, rhs: u32) -> u32 {
            lhs + rhs
        }
    }

    impl Identity<u32> for u32 {
        fn identity() -> u32 {
            0
        }
    }
}

pub mod ops {

    use super::laws::{Associativity, Identity};

    impl Associativity<i32> for i32 {
        fn associativity(lhs: i32, rhs: i32) -> i32 {
            lhs + rhs
        }
    }

    impl Identity<i32> for i32 {
        fn identity() -> i32 {
            0
        }
    }

    impl Associativity<String> for String {
        fn associativity(lhs: String, rhs: String) -> String {
            lhs + &rhs
        }
    }

    impl Identity<String> for String {
        fn identity() -> String {
            "".to_string()
        }
    }

    impl Associativity<bool> for bool {
        fn associativity(lhs: bool, rhs: bool) -> bool {
            lhs && rhs
        }
    }

    impl Identity<bool> for bool {
        fn identity() -> bool {
            true
        }
    }

    impl<T> Associativity<Vec<T>> for Vec<T> {
        fn associativity(lhs: Vec<T>, rhs: Vec<T>) -> Vec<T> {
            let mut v = lhs;
            v.extend(rhs);
            v
        }
    }

    impl<T> Identity<Vec<T>> for Vec<T> {
        fn identity() -> Vec<T> {
            Vec::new()
        }
    }

    impl<T, E> Associativity<Result<T, E>> for Result<T, E>
    where
        T: Associativity<T>,
    {
        fn associativity(lhs: Result<T, E>, rhs: Result<T, E>) -> Result<T, E> {
            match lhs {
                Ok(lhs) => match rhs {
                    Ok(rhs) => Ok(T::associativity(lhs, rhs)),
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            }
        }
    }
    impl<T, E> Identity<Result<T, E>> for Result<T, E>
    where
        T: Identity<T>,
    {
        fn identity() -> Result<T, E> {
            Ok(T::identity())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::typeclass::Semigroup;
    use crate::algebra::typeclass::Monoid;

    #[test]
    fn test_semigroup() {
        assert_eq!(i32::combine(2, 2), 4);
        assert_eq!(i32::combine(2, 3), 5);
        assert_eq!(
            String::combine("2".to_string(), "3".to_string()),
            "23".to_string()
        );
        let a = Result::<i32, String>::Ok(2);
        let b = Result::<i32, String>::Ok(3);
        assert_eq!(Result::combine(a, b).unwrap(), 5);
    }

    #[test]
    fn test_monoid() {
        assert_eq!(i32::empty(), 0);
        assert_eq!(u32::empty(), 0);
        assert_eq!(String::empty(), "".to_string());
    }
}
