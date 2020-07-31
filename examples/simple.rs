extern crate futures;
extern crate tokio;

extern crate tokio_ping;

use futures::{Future, Stream};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::mpsc::{channel, Sender};
use std::thread;

fn main() {
    println!("Begning");
    let mut ips: Vec<IpAddr> = Vec::new();
    ips.push(IpAddr::V4(Ipv4Addr::new(217, 182, 252, 147)));
    ips.push(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 123)));

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
        stream.take(3).for_each(move |mb_time| {
            match mb_time {
                Some(time) => {
                    tx.send(addr.to_string()).unwrap();
                    println!("{} time={:?}", addr, time);
                },
                None => println!("timeout"),
            }
            Ok(())
        })
    });

    tokio::run(future.map_err(|err| eprintln!("Error: {}", err)))
}
