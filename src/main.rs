mod use_macro;

use use_macro::type_name_of;

pub type Sum = u8;

struct MyStruct(i32);

fn main() {
    let it = (vec![()], 42);
    drop(it.0);
    dbg!(type_name_of!(it));
    let a: u8 = 1;
    dbg!(type_name_of!(a));
    let b: Sum = 8;
    dbg!(type_name_of!(b));
    let c = MyStruct(1);

    type_name_dyn(&"abcd".to_owned()); // alloc::string::String
    type_name_dyn(&100u8); // u8
    type_name_dyn(&10.0f32); // f32
    type_name_dyn(&a);
    type_name_dyn(&b);
    type_name_dyn(&c);

    println!("{}", type_name_of_val(&c));

    println!("====end")
}

trait AnyNamed {
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

impl<T: ?Sized> AnyNamed for T {}

fn type_name_dyn(val: &dyn AnyNamed) {
    //指向动态trait
    println!("{}", val.name());
}

fn type_name_of_val<T: ?Sized>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
