fn main() {
    //=========================枚举====================================
    let four = IpAddKind::V4(127, 0, 0, 1);
    let six = IpAddKind::V6(String::from("::1"));

    route(four);

    let home = IpAddr {
        kind: IpAddKind::V4(127, 0, 0, 1),
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddKind::V6(String::from("::1")),
        address: String::from("::1"),
    };

    //=========================Option<T>====================================
    let some_number = Some(5);
    let some_string = Some("A String");

    let absent_number: Option<i32> = None;
}

enum IpAddKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddKind,
    address: String,
}

fn route(ip_kind: IpAddKind) {}
