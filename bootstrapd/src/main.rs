use if_addrs::{get_if_addrs, IfAddr};
use socket2::{Domain, Protocol, Socket, Type};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::thread;
use std::time::Duration;

const PORT: u16 = 50000;

fn main() -> std::io::Result<()> {

    let socket = Socket::new(Domain::IPV4, Type::DGRAM Some(Protocol::UDP),)?;

    socket.set_broadcast(true)?;

    loop {
        let interfaces = get_if_addrs()?;

        for iface in interfaces {
            let IfAddr::V4(v4) = iface.addr else {
                continue;
            };

            if v4.ip.is_loopback() {
                continue;
            }

            let ip = u32::from(v4.ip);
            let mask = u32::from(v4.netmask);

            let broadcast = Ipv4Addr::from(ip | !mask);

            let addr = SocketAddrV4::new(broadcast, PORT);

            let payload = format!("my-beacon:{}", hostname::get().unwrap_or_default().to_string_lossy());

            match socket.send_to(payload.as_bytes(), &addr.into(),) {
                Ok(n) => {
                    println!("sent {} bytes to {} via {}", n, addr, iface.name);
                }
                Err(e) => {
                    eprintln!("send failed on {}: {}", iface.name, e);
                }
            }
        }

        thread::sleep(Duration::from_secs(5));
    }
}