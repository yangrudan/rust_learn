# RUST手册


## 1.数据类型

### 1.1 原生类型

**整数** 1、**浮点数** 1.2、**字符** 'a'、**字符串** "abc"、**布尔值** true 和**单元类型** () 可以用数字、文字或符号之类的 “字面量”（literal）来表示;

通过加前缀 0x、0o、0b，数字可以用十六进制、八进制或二进制记法表示。

```Rust
const MAX_POINT: i32 = 100;  // 常量
let num = 43u8;
```

**元组**是一个可以包含各种类型值的组合。元组使用括号 () 来构造（construct），而每个元组自身又是一个类型标记为 (T1, T2, ...) 的值，其中 T1、T2 是每个元素的类型。函数可以使用元组来返回多个值，因为元组可以拥有任意多个值。

```Rust
// 包含各种不同类型的元组
let long_tuple = (1u8, 2u16, 3u32, 4u64,
                 -1i8, -2i16, -3i32, -4i64,
                 0.1f32, 0.2f64, 'a', true);
// 通过元组的下标来访问具体的值
println!("long tuple first value: {}", long_tuple.0);

// 元组也可以充当元组的元素
let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

// 创建单元素元组需要一个额外的逗号，这是为了和被括号包含的字面量作区分。
println!("one element tuple: {:?}", (5u32,));
println!("just an integer: {:?}", (5u32));

// 元组可以被解构（deconstruct），从而将值绑定给变量
let tuple = (1, "hello", 4.5, true);

```

**数组array**是一组拥有相同类型 T 的对象的集合，在内存中是连续存储的。数组使用中括号 [] 来创建，且它们的大小在编译时会被确定。数组的类型标记为 [T; length]（译注：T 为元素类型，length 表示数组大小）。

**切片slice**类型和数组类似，但其大小在编译时是不确定的。相反，切片是一个双字对象（two-word object），第一个字是一个指向数据的指针，第二个字是切片的长度。这个 “字” 的宽度和 usize 相同，由处理器架构决定，比如在 x86-64 平台上就是 64 位。slice 可以用来借用数组的一部分。slice 的类型标记为 &[T]。

```Rust
use std::mem;

// 此函数借用一个 slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // 定长数组（类型标记是多余的）
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // 所有元素可以初始化成相同的值
    let ys: [i32; 500] = [0; 500];

    // 下标从 0 开始
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` 返回数组的大小
    println!("array size: {}", xs.len());

    // 数组是在栈中分配的
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // 数组可以自动被借用成为 slice
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // slice 可以指向数组的一部分
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // 越界的下标会引发致命错误（panic）
    println!("{}", xs[5]);
}
```

📖 **[examples/1_数据类型.rs](examples/1_数据类型.rs)**


### 1.2 自定义类型



**结构体（structure，缩写成 struct**有 3 种类型，使用 struct 关键字来创建：

元组结构体（tuple struct），事实上就是具名元组而已。

经典的 C 语言风格结构体（C struct）。

单元结构体（unit struct），不带字段，在泛型中很有用。

📖 **[examples/9_struct.rs](examples/9_struct.rs)**

**enum** 关键字允许创建一个从数个不同取值中选其一的枚举类型（enumeration）。任何一个在 struct 中合法的取值在 enum 中也合法。