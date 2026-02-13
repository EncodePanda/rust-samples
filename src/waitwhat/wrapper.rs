#[derive(Debug)]
#[allow(dead_code)]
pub enum Wrapped<T> {
    Value(T),
}

#[allow(dead_code)]
impl<T> Wrapped<T> {
    pub fn new(v: T) -> Self {
        Wrapped::Value(v)
    }
}

impl<T> IntoIterator for Wrapped<T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, 1>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Wrapped::Value(value) => [value].into_iter(),
        }
    }
}
