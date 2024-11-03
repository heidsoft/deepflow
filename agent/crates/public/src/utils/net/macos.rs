use nix::ifaddrs::getifaddrs;
use std::net::IpAddr;

fn link_list() -> Vec<String> {
    let mut interfaces = Vec::new();
    if let Ok(ifaddrs) = getifaddrs() {
        for ifaddr in ifaddrs {
            interfaces.push(ifaddr.interface_name);
        }
    }
    interfaces
}

fn addr_list() -> Vec<(String, IpAddr)> {
    let mut addresses = Vec::new();
    if let Ok(ifaddrs) = getifaddrs() {
        for ifaddr in ifaddrs {
            if let Some(address) = ifaddr.address {
                if let Some(ip_addr) = address.as_sockaddr_in().map(|sockaddr| sockaddr.ip()) {
                    addresses.push((ifaddr.interface_name, IpAddr::V4(ip_addr)));
                }
            }
        }
    }
    addresses
}

fn main() {
    println!("Network Interfaces: {:?}", link_list());
    println!("IP Addresses: {:?}", addr_list());
}
