fn main() {
    let mut v = Vec::new();

    //更新vector
    v.push(1);

    for i in &v{
        println!("{}", i);
    }
}
