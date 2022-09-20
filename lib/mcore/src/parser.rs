pub trait Parser<A> {
    fn parse(input: &str) -> Result<A, String>;
}
