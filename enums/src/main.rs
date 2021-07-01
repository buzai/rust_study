// enum IPTypes {
//     V4, V6,
// }

// struct IpAddr {
//     kind: IPTypes,
//     address: String,
// }

// #[derive(Debug)]
// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// enum Msg {
//     Quit,
//     Move {x: u32, y: u32, z: u32 },
//     Write(String),
//     ChangeColor(u32, u32, u32),
// }

// impl Msg {
//     fn call(&self) {
//         println!("call")
//     }
// }



fn main() {

    // let home = IpAddr {
    //     kind: IPTypes::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // pip(home);
    // let loopback = IpAddr {
    //     kind: IPTypes::V6,
    //     address: String::from("::1"),
    // };
    // pip(loopback);

    // let home = IpAddrKind::V4(0,0,0,0);
    // let loopback = IpAddrKind::V6(String::from("::1"));

    // pip2(home);

    // let q = Msg::Quit;
    // let m = Msg::Move {x: 1, y:2, z:3};
    // let w = Msg::Write(String::from("::1"));
    // let c = Msg::ChangeColor(1,2,3);

    // c.call()

    let x: i8 = 1;
    let y: Option<i8> = Some(12);
    println!("{}", x);
    println!("{:?}", y);


    // let z = x + y;

}

// fn pip2(address : IpAddrKind) {
//     println!("{:?}", address);
// }

// fn pip(address : IpAddr) {
//     println!("{}", address.address)
// }
