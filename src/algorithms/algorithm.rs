pub trait Algorithm {
    type Output;

    fn calculate(&mut self) -> Self::Output;

    fn step(&mut self);

    fn reset(&mut self);
}
