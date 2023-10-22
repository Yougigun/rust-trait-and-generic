fn main() {
    // let to = Box::new(Struct);
}

trait Trait<T> {
    fn foo(&self, t: T);
    fn bar<U>(&self, t: U);
}
use trait_object_with_generic_trait::B as Ba;

mod B;