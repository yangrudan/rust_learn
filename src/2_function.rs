fn another_function(x: i32) -> i32 {
    println!("Hello another {x}");
    return 5;
}

fn main() {
    println!("Hi");
    println!("{}", another_function(55));
}
