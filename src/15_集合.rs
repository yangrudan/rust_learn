fn main() {
    let mut v = Vec::new();

    //更新vector
    v.push(1);

    for i in &v {
        println!("{}", i);
    }

    //Excel单元格, 利用enum存放多种数据类型
    let row = vec![
        SpreadSheetCell::Int(4),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];

    for i in &row {
        println!("{:?}", i);
    }
}

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

////string
fn main() {
    let s = String::new();

    let mut s = "initial ".to_string();

    s.push_str("data");

    println!("{s}");
}
