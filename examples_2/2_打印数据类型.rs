// Copyright (c) Cookie Yang. All right reserved.
use std::any::type_name;

/*
//use_macro.rs
// Copyright (c) Cookie Yang. All right reserved.
macro_rules! type_name_of {
    //定义了一个宏 type_name_of，宏的作用是获取表达式的类型名称。
    ( $e:expr $(,)? ) => {{
        // 宏的匹配规则，接受一个表达式作为参数。
        let it = []; //创建一个空数组 it，用于占位。
        #[allow(unreachable_code)] //禁止编译器报告不可达代码的警告。
        {
            if false {  //这是一个始终为假的条件，目的是禁用变量 it 的借用检查和析构检查。因为在 Rust 中，如果一个变量没有被使用，编译器会警告或报错。通过这种方式，可以绕过编译器的检查，达到获取表达式类型的目的。
                loop {} // disables borrowck and dropck
                (&mut { it })[0] = &$e; // nudges type inference 这行代码实际上没有被执行，因为它位于永远不会执行的代码块中。它的目的是“提示”编译器推断表达式 $e 的类型，从而在编译时确定类型，并将其绑定到 it 变量上。
            }
        }
        $crate::use_macro::__helper__(it)
    }};
}
pub(crate) use type_name_of;

#[doc(hidden)] //隐藏此项文档。
pub fn __helper__<T>(_: [&T; 0]) -> &'static str {
    ::core::any::type_name::<T>()
}

 */

mod use_macro;

use use_macro::type_name_of;

pub type Sum = u8;

struct MyStruct(i32);

fn main() {
    let a: Sum = 2;
    println!("{}", type_name::<Sum>()) //这个函数打印好像没有 yiyi

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

// 方法2
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

// 方法3
fn type_name_of_val<T: ?Sized>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
