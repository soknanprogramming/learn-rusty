#[derive(PartialEq, Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit(QuitMessage),
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn some_function(){
        println!("some_function working!");
    }
}

struct QuitMessage;  // unit struct
struct MoveMessage{
    x: i32,
    y: i32
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

struct IpAddr{
    kind: IpAddrKind,
    address: String
}

fn main(){
    let four = IpAddrKind::V4(123, 0, 0, 1);

    if four == IpAddrKind::V4(123, 0, 0, 0){
        println!("four == IpAddrKind::V4(123, 0, 0, 0)");
    } else {
        println!("four not == IpAddrKind::V4(123, 0, 0, 0)");
    }

    let six = IpAddrKind::V6(String::from("v6"));
    println!("six is: {:#?}", six);

    Message::some_function();

    let a = QuitMessage;
    let b = Message::Quit(QuitMessage);

    let color = ChangeColorMessage(1, 1, 1);
}