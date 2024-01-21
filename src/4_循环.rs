//loop
//while
//for
//这三种循环

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is : {}", a[index]);

        index = index + 1;
    }

    for element in a.iter(){
        println!("the value is: {}", element)
    }

    for number in (1..4).rev(){
        println!("{}!", number)
    }
}