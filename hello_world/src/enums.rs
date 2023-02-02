#[derive(Debug)]
// enum ip_type{
//     ipv4(u8, u8, u8, u8),
//     ipv6(String)
// }

enum message{
    quit,
    position{ x: i32, y: i32},
    write(String),
    color(u32, u32, u32)
}

impl message{
    fn call(&self){
        println!("{:?}", self);
    }
}

pub fn run(){
    // let home = ip_type::ipv4(192, 168, 0 ,1);
    // let loopback = ip_type::ipv6(String::from("::1"));

    let m = message::write(String::from("hello"));
    m.call(); 
}