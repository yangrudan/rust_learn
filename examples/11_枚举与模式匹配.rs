fn main() {
    //=========================枚举====================================
    let four = IpAddKind::V4(127, 0, 0, 1);
    let _six = IpAddKind::V6(String::from("::1"));

    four.show_type();

    let _home = IpAddr {
        kind: IpAddKind::V4(127, 0, 0, 1),
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddKind::V6(String::from("::1")),
        address: String::from("::1"),
    };

    //=========================Option<T>====================================
    let some_number = Some(5);
    let some_string = Some("A String");

    let absent_number: Option<i32> = None;
}

#[derive(Debug)]
enum IpAddKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddKind,
    address: String,
}

impl IpAddKind {
    fn show_type(&self) {
        match self {
            IpAddKind::V4(_, _, _, _) => println!("This is type 4 {:?}", *self), //TODO enum里的元组怎么打印第一个元素?
            IpAddKind::V6(_) => println!("This is type 6"),
        }
    }
}
