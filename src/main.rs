fn main() {
    let mut s = String::from("Hello World! ");

    let s2 = s.clone();

    takes_ownership(s2);

    s.push_str("BABY");

    println!("{}", s)
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string)
}
