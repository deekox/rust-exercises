#[derive(Debug)]
enum IpAddr {
    IPv4(String),
    IPv6(String, i32, i32),
}



fn main() {
    let a1 = IpAddr::IPv4(String::from("10.172.181.63"));
    let a2 = IpAddr::IPv6(String::from("0::1"), 21, 32);

    fun(a1);
    fun(a2);

    let a3 = IpAddr::IPv4(String::from("DUPA"));
    if let IpAddr::IPv4(x) = a3 {
	println!("extracted value: {x}");
    }

    let a4 = IpAddr::IPv4(String::from("10.172.181.63"));
    let a5 = IpAddr::IPv6(String::from("0::1"), 21, 32);

    let_else(a4);
    let_else(a5);
}

fn fun(ip: IpAddr) {
    match ip {
	IpAddr::IPv4(_) => println!("v4"),
	IpAddr::IPv6(a, ..) => ()
	// IpAddr::IPv4(_) => 1,
	// IpAddr::IPv6(_) => 2
    }
}


fn let_else(o: IpAddr) {
    let IpAddr::IPv4(adr) = o else {
	println!("It is not an IPv4 :(");
	return;
    };

    println!("IPv4 adr: {adr}");
}
