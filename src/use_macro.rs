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
