// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo(String),
    Move(i32, i32),
    ChangeColor(String, String),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("hello world")));
    println!("{:?}", Message::Move(1, 2));
    println!("{:?}", Message::ChangeColor(String::from("Blue"), String::from("Yellow")));
}
