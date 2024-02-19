fn give_me<T: std::fmt::Display>(value: T) {
    let a = value;
    println!("{}", a);
}

struct Container<T> {
    item: T,
}

impl<T> Container<T> {
    fn new(item: T) -> Self {
        Container { item }
    }
}

impl Container<u32> {
    fn sum(item: u32) -> Self {
        println!("hello");
        Container { item }
    }
}

fn main() {
    let a = "aaa";
    let b = 1;
    give_me(a);
    give_me(b);

    let _c = Container::sum(32u32);
    let _d = Container::new(true);
}
