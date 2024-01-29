// 栈内存 堆内存

// Stack 栈后进先出(LIFO), 使用内存大小确定

// Heap 编译时内存大小会发生变化的内存, 操作系统找到足够大的一块空间并标记为在用
//
fn main() {
    let s = String::from("Hello World! ");

    takes_ownership(s);

    // s.push_str("BABY");
    //
    // println!("{}", s)

    let x = 6;

    makes_copy(x);

    println!("{}", x);
}


fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

;