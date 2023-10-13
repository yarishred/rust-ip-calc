
pub enum IPType {
    IpV4(Vec<u8>),
    IpV6(Vec<u16>),
}

//This is the struct for the IP Adress
pub struct IPAddress {
    _ip_address: Result<Vec<IPType>, String>,
}

impl IPAddress {
    pub fn parse_ip(ip_address: &str) {
        if ip_address.contains(".") {
            Self::parse_ipv4(ip_address);
        } else if ip_address.contains(":") {
            Self::parse_ipv6(ip_address);
        }
    }

    pub fn parse_ipv4(ip_add: &str) -> Vec<u16> {
        let ipv4 = ip_add
            .split(".")
            .map(|x| {
                x.parse::<u8>()
                    .map(|n| n as u16)
                    .map_err(|err| err.to_string())
            })
            .collect();
        let result = match ipv4 {
            Ok(vec) => vec,
            Err(err) => {
                eprintln!("Error: {}", err);
                Vec::new()
            }
        };
        println!("{:?}", result);
        result
    }

    pub fn parse_ipv6(ip_add: &str) {
        let ipv6_input: Vec<&str> = ip_add.split(":").collect();
        let mut ipv6_parsed: Vec<String> = Vec::new();

        for &hextet in &ipv6_input {
            if hextet == "" {
                for _ in 0..(8 - ipv6_input.len() + 1) {
                    ipv6_parsed.push("0x0000".to_string());
                }
            } else {
                let variable = format!("0x{}", hextet);
                ipv6_parsed.push(variable);
            }
        }

        println!("{:?}", ipv6_parsed);

    }

    // pub fn parse_ipv6(ip_add: &str) {
    //     ip_add
    //         .split(".")
    //         .map(|x| x.parse::<u16>().map_err(|err| err.to_string()))
    //         .collect();
    // }
    // pub fn convert_to_vec(ip_add: &str) {

    //     // let result = match ip_to_vec {
    //     //     Ok(vec) => vec,
    //     //     Err(err) => {
    //     //         eprintln!("Error: {}", err);
    //     //         Vec::new()
    //     //     }
    //     // };
    // }
}
