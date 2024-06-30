#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?}, {:?}", four, six);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}, {:?}", home, loopback);

    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);
}

// fn route(ip_type: IpAddrKind) {}