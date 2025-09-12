enum IpAddrKind {
    V4(String),
    V6(String)
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main(){
    // let localhost = IpAddr {
    //     kind: V4,
    //     address: String::from("127.0.0.1")
    // };

    let localhost = IpAddrKind::V4(String::from("127.0.0.1"));
}