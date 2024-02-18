fn main() {
    let number = 3;

    if number < 5 {
        println!("number less 5");
    } else if number > 5 {
        println!("num over 5");
    } else {
        println!("num is 5");
    }

    let condition = true;
    let number = if condition { 6 } else { 5 }; // 三元运算符
    println!(" the value is {}", number)
}
