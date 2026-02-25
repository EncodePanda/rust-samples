use rust_samples::improveurrust::samemethod::*;

fn main() {
    let foo = Foo::new(1000);
    for i in foo {
        println!("value is {}", i);
    }
    let foo = Foo::new(1000);
    println!("value is {}", foo.into_iter().next().unwrap());

    let foo = Foo::new(1000);
    println!("value is {}", IntoIterator::into_iter(foo).next().unwrap());
}
