fn main() {

}

enum Coin{
    Penny,
    Nickle,
    Dime,
    Quarter,
}


fn value_in_cents(coin: Coin) -> u8{
    match coin {
        Coin::Penny => {
            println!("Penny");
            1
        },
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}