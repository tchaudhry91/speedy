use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

use std::time::{Duration, Instant};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Mode: Listen/Connect
    #[clap(short, long, default_value = "listen")]
    mode: String,

    /// Address
    #[clap(short, long, default_value = "localhost:10988")]
    address: String,
}

fn pretty_print_speeds(speed_bps: f64) {
    println!("{} kBps", speed_bps / 1024.0);
    println!("{} mBps", speed_bps / (1024.0 * 1024.0));
}

fn server(addr: String) {
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming() {
        let start = Instant::now();
        let mut total_written = 0;
        let mut stream = stream.unwrap();
        println!(
            "Connection established from {}",
            stream.peer_addr().unwrap()
        );
        println!("Running test for 5 seconds...");
        let chunk = [0; 1024];
        while start.elapsed() < Duration::from_secs(5) {
            let written = stream.write(&chunk).unwrap();
            total_written += written;
        }
        stream.flush().unwrap();
        let speed_bps = total_written as f64 / start.elapsed().as_secs_f64();
        pretty_print_speeds(speed_bps);
    }
}

fn client(addr: String) {
    let mut stream = TcpStream::connect(addr).unwrap();
    // Start reading
    let mut buffer: [u8; 1024] = [0; 1024];
    let mut total_read = 0;
    let start = Instant::now();
    println!("Connection Established to Server");
    loop {
        let read_bytes = stream.read(&mut buffer).unwrap();
        if read_bytes == 0 {
            break;
        }
        total_read += read_bytes;
    }
    println!("Total Read: {}", total_read);
    let speed_bps = total_read as f64 / start.elapsed().as_secs_f64();
    pretty_print_speeds(speed_bps);
}

fn main() {
    let args = Args::parse();

    if args.mode == "listen" {
        server(args.address);
    } else {
        client(args.address);
    }
}
