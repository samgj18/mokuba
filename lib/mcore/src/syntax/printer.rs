pub mod syntax {
    pub trait PrettyPrint<A> {
        fn pretty_print(&self) -> String;
    }

    impl PrettyPrint<String> for String {
        fn pretty_print(&self) -> String {
            self.to_string()
        }
    }

    impl<A> PrettyPrint<Option<A>> for Option<A>
    where
        A: PrettyPrint<A>,
    {
        fn pretty_print(&self) -> String {
            match self {
                Some(a) => a.pretty_print(),
                None => "None".to_string(),
            }
        }
    }

    impl<A, E> PrettyPrint<Result<A, E>> for Result<A, E>
    where
        A: PrettyPrint<A>,
        E: PrettyPrint<E>,
    {
        fn pretty_print(&self) -> String {
            match self {
                Ok(a) => a.pretty_print(),
                Err(e) => e.pretty_print(),
            }
        }
    }

    impl<A> PrettyPrint<Vec<A>> for Vec<A>
    where
        A: PrettyPrint<A>,
    {
        fn pretty_print(&self) -> String {
            let mut s = String::new();
            s.push('[');

            for (i, a) in self.iter().enumerate() {
                s.push_str(a.pretty_print().as_str());

                if i < self.len() - 1 {
                    s.push_str(", ");
                }
            }
            s.push(']');
            s
        }
    }

    impl PrettyPrint<bool> for bool {
        fn pretty_print(&self) -> String {
            self.to_string()
        }
    }

    impl PrettyPrint<char> for char {
        fn pretty_print(&self) -> String {
            self.to_string()
        }
    }

    impl PrettyPrint<i8> for i8 {
        fn pretty_print(&self) -> String {
            self.to_string()
        }
    }

    impl PrettyPrint<i16> for i16 {
        fn pretty_print(&self) -> String {
            self.to_string()
        }
    }

    impl PrettyPrint<i32> for i32 {
        fn pretty_print(&self) -> String {
            self.to_string()
        }
    }

    impl PrettyPrint<i64> for i64 {
        fn pretty_print(&self) -> String {
            self.to_string()
        }
    }

    impl PrettyPrint<u32> for u32 {
        fn pretty_print(&self) -> String {
            self.to_string()
        }
    }
}
