use std::io::BufRead;

fn main() {
    println!("Hello, world!");
}

struct Animal<T>(T);

impl Animal<i32> {
    // cannot define a method with the same name as the method in the generic impl
    // fn speak(&self) {}
}

impl<T> Animal<T> {
    fn speak(&self) {}
}
