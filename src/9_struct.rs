#[derive(Debug)]

struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

//Tuple Struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//unit-like struct  (没有field content)


fn main() {
    let mut user = User {
        email: String::from("154765@qq.com"),
        username: String::from("Yang"),
        active:true,
        sign_in_count: 556,
    };

    println!("{}", user.email);
    //println!("{:#?}", user); // :? :#?


    user.email = String::from("154765@qq.com");
}

