
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {
    println!("ip_kind: {:#?}", ip_kind);
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),

    _V4(u8, u8, u8, u8),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("self: {:#?}", self);
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_enum() {
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        println!("four: {:#?}", four);
        println!("six: {:#?}", six);
    }

    #[test]
    fn test_route() {
        route(IpAddrKind::V4);
        route(IpAddrKind::V6);
    }

    #[test]
    fn test_enum_with_data() {
        let home = IpAddr::V4(String::from("127.0.01"));
        let loopback = IpAddr::V6(String::from("::1"));

        println!("home: {:#?}", home);
        println!("loopback: {:#?}", loopback);

        let home = IpAddr::_V4(127, 0, 0, 1);
        println!("home: {:#?}", home);
    }

    #[test]
    fn test_standard_ip() {
        use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

        let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

        assert_eq!("127.0.0.1".parse(), Ok(localhost_v4));
        assert_eq!("::1".parse(), Ok(localhost_v6));

        assert_eq!(localhost_v4.is_ipv6(), false);
        assert_eq!(localhost_v4.is_ipv4(), true);

        println!("localhost_v4: {:#?}", localhost_v4);
        println!("localhost_v6: {:#?}", localhost_v6);
    }

    #[test]
    fn test_message() {
        let m = Message::Write(String::from("hello"));
        m.call();
    }
}