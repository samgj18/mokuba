pub trait Executable<A, B, C> {
    fn execute(&self) -> Result<B, C>;
}
