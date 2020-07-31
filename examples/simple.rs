extern crate futures;
extern crate tokio;

extern crate tokio_ping;

use futures::{Future, Stream};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::mpsc::{channel, Sender};
use std::thread;

async fn run() {
    println!("Run");
}

fn main() {
    println!("Begning");
    let mut ips: Vec<IpAddr> = Vec::new();
    ips = check_private_range();
    ips.push(IpAddr::V4(Ipv4Addr::new(217, 182, 252, 147)));
    //ips.push(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 123)));
    let (tx, rx) = channel();
    for ip in ips {
        let tx = tx.clone();
        thread::spawn(move || {
            ping(tx, ip);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{}", v);
    }
    //Ok(())

    println!("End");
}

// Write buffer like healing vision
fn ping(tx: Sender<String>, addr: IpAddr) {
    // let addr = std::env::args().nth(1).unwrap().parse().unwrap();
    let pinger = tokio_ping::Pinger::new();
    let stream = pinger.and_then(move |pinger| Ok(pinger.chain(addr).stream()));
    let future = stream.and_then(move |stream| {
        stream.take(1).for_each(move |mb_time| {
            match mb_time {
                Some(time) => {
                    tx.send(addr.to_string()).unwrap();
                    //println!("{} time={:?}", addr, time);
                },
                None => {}, /*println!("timeout")*/
            }
            Ok(())
        })
    });

    tokio::run(future.map_err(|err| eprintln!("Error: {}", err)))
}

/// Scan private intranet
///
/// The private address ranges are defined in IETF RFC 1918 and include:
///
/// 10.0.0.0/8
/// 172.16.0.0/12
/// 192.168.0.0/16
///
/// https://doc.rust-lang.org/std/net/struct.Ipv4Addr.html#method.is_private
fn check_private_range() -> Vec<IpAddr> {
    let mut range: Vec<[Ipv4Addr; 2]> = Vec::new();
    /*
    range.push([Ipv4Addr::new(10, 0, 0, 0), Ipv4Addr::new(10, 255, 255, 255)]);
    range.push([
        Ipv4Addr::new(172, 16, 0, 0),
        Ipv4Addr::new(172, 31, 255, 255),
    ]);
    range.push([
        Ipv4Addr::new(192, 168, 0, 0),
        Ipv4Addr::new(192, 168, 255, 255),
    ]);
    */
    range.push([
        Ipv4Addr::new(192, 168, 1, 110),
        Ipv4Addr::new(192, 168, 1, 123),
    ]);
    let mut ip: Vec<IpAddr> = Vec::new();
    for r in range {
        let mut pos: [u8; 4] = r[0].octets();
        let pos_final: [u8; 4] = r[1].octets();
        loop {
            let current_addr = Ipv4Addr::new(pos[0], pos[1], pos[2], pos[3]);
            ip.push(IpAddr::V4(current_addr));
            let compare_1 = i64::from_str_radix(
                hex::encode(vec![pos[0], pos[1], pos[2], pos[3]]).as_str(),
                16,
            )
            .unwrap();
            let compare_2 = i64::from_str_radix(
                hex::encode(vec![
                    pos_final[0],
                    pos_final[1],
                    pos_final[2],
                    pos_final[3],
                ])
                .as_str(),
                16,
            )
            .unwrap();
            if compare_1 >= compare_2 {
                break;
            };
            // Algo for Ipv4Addr
            if pos[3] == 255 {
                pos[3] = 0;
                if pos[2] == 255 {
                    pos[2] = 0;
                    if pos[1] == 255 {
                        pos[1] = 0;
                        pos[0] = pos[0] + 1;
                    //if pos[0] == 255 {
                    // }
                    } else {
                        pos[1] = pos[1] + 1;
                    }
                } else {
                    pos[2] = pos[2] + 1;
                }
            } else {
                pos[3] = pos[3] + 1;
            }
        }
    }
    ip
}
