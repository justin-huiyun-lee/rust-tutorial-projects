#[derive(Debug)]
struct A {
    one: i32,
    two: i32,
    three: i32,
    four: i32,
    five: String,
}

fn main() {
    let a1 = A {
        one: 1,
        two: 2,
        three: 3,
        four: 4,
        five: String::from("hello"),
    };

    let a2 = A { one: 5, ..a1 };

    println!("{:#?}", a1);
}
