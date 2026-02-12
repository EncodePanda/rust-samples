mod feature {
    #[allow(dead_code)]
    #[derive(Debug)]
    // used to be Foo
    pub enum Bar {
        One,
        Two,
    }

    // used to a re-export the enum under the old name Foo, but it does not work
    // when imported explicitly via use feature::Foo.
    // pub type Foo = Bar;

    // this works instead
    pub use Bar as Foo;
}

#[allow(dead_code)]
pub fn use_it() {
    use feature::Foo::*;
    println!("{:?}", One);
}
