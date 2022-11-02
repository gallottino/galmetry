pub trait Algorithm {
    type Output;

    fn calculate(&mut self) -> Self::Output;

    fn reset(&mut self);
}
