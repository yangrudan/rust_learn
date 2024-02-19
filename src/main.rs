fn give_me<T: std::fmt::Display>(value: T) {
    let a = value;
    println!("{}", a);
}

fn main() {
    let a = "aaa";
    let b = 1;
    give_me(a);
    give_me(b);
}
