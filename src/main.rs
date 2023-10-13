use ipcalculator::IPAddress;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let ipaddr_arg = &args[1];

    IPAddress::parse_ip(ipaddr_arg)
}
