pub struct Foo {
    x: i32,
}

impl Foo {
    pub fn into_iter(self) -> std::array::IntoIter<i32, 1> {
        [0].into_iter()
    }

    pub fn new(x: i32) -> Self {
        Foo { x }
    }
}

impl IntoIterator for Foo {
    type Item = i32;
    type IntoIter = std::array::IntoIter<i32, 1>;

    fn into_iter(self) -> Self::IntoIter {
        [self.x].into_iter()
    }
}
