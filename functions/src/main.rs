fn main() {
    println!("Hello, world!");
    println!("{}", another_function(1, 2));
}

fn five() -> i32 {
    5
}

fn another_function(x: i32, y: i32) -> i32 {
    println!("x: {}, y: {}", x, y);
    five()
}
