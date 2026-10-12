// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.


// 这种写法，叫做枚举成员的关联类型
// 比如这个 Move 成员关联类型就是结构体
#[derive(Debug)]
enum Message {
    Move{
        x:i32,
        y:i32,
    },
    Echo(String),
    ChangeColor(i32,i32,i32),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
