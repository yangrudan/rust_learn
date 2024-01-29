fn main() {
    let mut s = String::from("Hello World");

    let hello = &s[0..5];  // 引用一部分
    let world = &s[6..];

    print!("{},{}", hello, world);

    let wordIndex = first_world(&s);
    println!("{}", wordIndex)
}

fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }
    &s[..]
}