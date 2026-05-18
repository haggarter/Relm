use if_addrs::{get_if_addrs, IfAddr};
use socket2::{Domain, Protocol, Socket, Type};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::thread;
use std::time::Duration;
use std::mem::MaybeUninit;

const PORT: u16 = 50000;

fn beacon() -> std::io::Result<()> {

    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP),)?;

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

            let payload = format!("my-beacon:{}",hostname::get().unwrap_or_default().to_string_lossy());

            match socket.send_to(payload.as_bytes(), &addr.into()) {
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

fn listener() -> std::io::Result<()> {
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP),)?;

    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), PORT,);

    socket.bind(&addr.into())?;

    let mut buf = [MaybeUninit::<u8>::uninit(); 2048];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((n, addr)) => {
                let bytes = unsafe {
                    std::slice::from_raw_parts(buf.as_ptr() as *const u8, n,)
                };

                let msg = String::from_utf8_lossy(bytes);

                println!("received {} bytes from {}: {}", n, addr.as_socket().unwrap(), msg);
            }
            Err(e) => {
                eprintln!("failed to receive data: {}", e);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    thread::spawn(|| {
        beacon().unwrap();
    });

    thread::spawn(|| {
        listener().unwrap();
    });

    loop {
        thread::sleep(Duration::from_secs(60));
    }
}