pub mod syntax {
    #[macro_export]
    macro_rules! mapt {
        ($trait: ident,
            $($value: ident)+, // + means one or more, * means zero or more, ? means zero or one
            $($item: ident)+,
            $($self: ident)+,
            $($that: ident)+) => {
                pub trait $trait {
                    type Item;

                    fn map<B, F>(self, f: F) -> ($($that, )*)
                    where
                        F: FnMut(Self::Item) -> B;
                }

                impl<T> $trait for ($($self, )*) {
                    type Item = T;

                    fn map<B, F>(self, mut f: F) -> ($($that, )*)
                    where
                        F: FnMut(Self::Item) -> B
                    {
                        let ($($value,)*) = self;
                        ($(f($value),)*)
                    }
                }
            }
    }

    mapt! {
        TupleMap1,
        v1,
        Item,
        T,
        B
    }

    mapt! {
        TupleMap2,
        v1 v2,
        Item Item,
        T T,
        B B
    }
}
