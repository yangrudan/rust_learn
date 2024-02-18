#[derive(Debug)]

struct User {
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
        active: true,
        sign_in_count: 556,
    };

    println!("{}", user.email);
    println!("{:#?}", user); // :? :#?

    user.email = String::from("154765@qq.com");
    user.active = false;
    user.sign_in_count = 32;

    let _color = Color(0, 0, 0);
    let _position = Point(1, 1, 1);

    user.change_name(String::from("cookie"));
    println!("{:#?}", user);

    let default_user = User::default_user(String::from("XU"));
    println!("{:#?}", default_user);
}

impl User {
    fn change_name(&mut self, new_name: String) {
        self.username = new_name;
    }

    fn default_user(name: String) -> User {
        User {
            username: name,
            email: String::from("xxx"),
            active: true,
            sign_in_count: 32,
        }
    }
}
