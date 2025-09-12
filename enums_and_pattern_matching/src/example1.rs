#[derive(PartialEq, Debug)]
enum IpAddrKind{
    V4,
    V6
}


fn main(){
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    if four == IpAddrKind::V4{
        println!("four is == IpAddrKind");
    }

    println!("six is : {:#?}", six);
    
}

