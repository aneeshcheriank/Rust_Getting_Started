fn main() {
    enum IpAddrKind{
        V4,
        V6
    }

    struct IpAddr{
        kind: IpAddrKind,
        address: String,
    };

    let home: IpAddr = IpAddr{
        kind: IpAddrKind::V4,
        address: "127.0.0.1".to_string()
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    fn route(ip_kind: IpAddrKind){
        {}
    }

    route(ip_kind: IpAddrKind::V4);
}

fn sample(){
    // we can define data inside the enum itself
    enum IpAddress{
        V4(String),
        V6(String),
    }

    let home = IpAddress::V4(String::from("127.0.0.1"));
}

