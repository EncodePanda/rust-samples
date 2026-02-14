use rust_samples::waitwhat::wrapper::Wrapped;

pub fn main() {
    let w = Wrapped::new(1000);
    for i in w {
        println!("{}", i);
    }
}
