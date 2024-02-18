fn main() {
    let v = Some(0u8);
    match v {
        Some(3) => println!("three"),
        Some(4) => println!("four"),
        Some(5) => println!("five"),
        Some(6) => println!("six"),
        _ => (),
    }

    if let Some(3) = v {
        println!("three");
    } else {
        println!("others")
    }
}
