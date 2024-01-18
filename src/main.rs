use std::io;

fn main() {
    println!("猜数游戏!");

    let mut guess = String::new();
    match io::stdin().read_line(&mut guess) {
        Ok(n) => {
            println!("{n} bytes read");
            println!("{guess}");
        }
        Err(error) => println!("error: {error}"),
    }
}
