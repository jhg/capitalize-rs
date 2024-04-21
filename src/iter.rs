/// It's implemented for all types that implement [`Iterator<Item=char>`].
pub trait CapitalizeIter: Iterator<Item = char> {
    fn capitalize(&mut self) -> impl Iterator<Item = char>;
}

impl<T: Iterator<Item = char>> CapitalizeIter for T {
    #[inline]
    fn capitalize(&mut self) -> impl Iterator<Item=char> {
        self.next().into_iter()
            .flat_map(char::to_uppercase)
            .chain(self.flat_map(char::to_lowercase))
    }
}
