// Copyright (c) Cookie Yang. All right reserved.
fn main() {
    let double = |x| x * 2;
    let value = 5;
    let twice = double(value);
    println!("{} twice", twice);

    let big_closere = |b, c| {
        let z = b + c;
        z * twice
    };
    let d = big_closere(1, 2);
    println!("{} d", d);
}
