pub mod syntax {
    pub trait OptionOps<A> {
        fn some(self) -> Option<A>;
        fn none(self) -> Option<A>;
    }

    impl OptionOps<String> for String {
        fn some(self) -> Option<String> {
            Some(self)
        }

        fn none(self) -> Option<String> {
            None
        }
    }

    impl<A, E> OptionOps<Result<A, E>> for Result<A, E> {
        fn some(self) -> Option<Result<A, E>> {
            Some(self)
        }

        fn none(self) -> Option<Result<A, E>> {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::syntax::*;

    #[test]
    fn test_some() {
        let ok: Result<i32, String> = Ok(1);
        assert_eq!(Some("hello".to_string()), "hello".to_string().some());
        assert_eq!(Some(ok), Ok(1).some());
    }

    #[test]
    fn test_none() {
        let ok: Result<i32, String> = Ok(1);

        assert_eq!(None, "hello".to_string().none());
        assert_eq!(None, ok.none());
    }
}
