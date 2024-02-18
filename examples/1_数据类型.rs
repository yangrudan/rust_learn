fn main() {
    const MAX_POINT: i32 = 100; // 常量
    println!("{MAX_POINT}");

    let x = 5;
    let x = x + 1;
    println!("{}", x);

    let usize = "    ";
    let space = usize.len();
    println!(" aaa {space}");

    let num = 43u8;
    println!("num {}", num);

    let tup = (100, "hello");
    // 模式匹配destructure一个Tuple
    let (_a, _b) = tup;

    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _b = [3; 5];
    let index = _b[1];
    println!("{index}");

    println!("yes");
}
