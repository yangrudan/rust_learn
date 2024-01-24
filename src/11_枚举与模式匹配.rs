fn main() {
    let four = IpAddKind::V4(127,0,0,1);
    let six = IpAddKind::V6(String::from("::1"));

    route(four);

    let home = IpAddr{
        kind: IpAddKind::V4(127,0,0,1),
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
        kind: IpAddKind::V6(String::from("::1")),
        address: String::from("::1"),
    };
}

enum IpAddKind{
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr{
    kind: IpAddKind,
    address: String,
}

fn route(ip_kind: IpAddKind){

}