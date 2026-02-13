#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Maybe<T> {
    Just(T),
    Nothing,
}

impl<T> Iterator for Maybe<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match std::mem::replace(self, Maybe::Nothing) {
            Maybe::Just(value) => Some(value),
            Maybe::Nothing => None,
        }
    }
}
