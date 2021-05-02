trait Trait1 {
}

trait Trait2 {
    fn trait2_func(&self) -> f32;
}

struct Test1;

struct Test2<T> {
    test1: T,
}

impl<T> Trait2 for Test2<T>
    where T: Trait1 {
    fn trait2_func(&self) -> f32 {
        42.
    }

}

fn get_val<T: Trait2>(obj: T) -> f32 {
    obj.trait2_func()
}

fn main() {
    let test1_obj = Test1;
    let test2_obj = Test2 { test1: test1_obj};
    println!("{}", get_val(test2_obj))
}