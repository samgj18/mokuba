pub trait Execute<A, B, C> {
    fn execute(&self) -> Result<B, C>;
}
