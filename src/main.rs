fn main() {
    let s = String::from("Hello World");

    let hello = &s[0..5];  // 引用一部分
    let world = &s[6..11];

    print!("{},{}", hello, world);
}