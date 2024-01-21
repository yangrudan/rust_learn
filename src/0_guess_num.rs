use rand::Rng;
use std::cmp::Ordering;
use std::io;


fn main() {
    println!("猜数游戏!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("神秘数字是: {}", secret_number);

    loop{
        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(n) => {
                println!("{n} bytes read");
                println!("{guess}");
            }
            Err(error) => {
                println!("error: {error}");
                continue;
            },
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你猜测的数字是:{}", guess);
        match  guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
