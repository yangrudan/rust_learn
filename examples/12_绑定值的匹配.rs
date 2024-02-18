//绑定值的匹配
//Option<T> 有 None

fn main() {
    let c = Coin::Quarter(UsState::Alabama);
    println!("{}", value_in_cents(c));
    let _d = Coin::Penny;
    let _e = Coin::Quarter(UsState::Alaska);
}

#[warn(dead_code)]
enum Coin {
    Penny,
    Quarter(UsState), //关联一个数据
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Penny");
            1
        }
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
