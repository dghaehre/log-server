//! Displays all request and returns 200 if nothing else is specified
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use chrono::prelude::*;
use clap::{App, Arg};
use colored::*;
use serde_json::{Result, Value};

fn main() {
    let mut count = 0;

    let matches = App::new("The empty server")
        .version("1.0")
        .author("Daniel Hæhre <dghaehre@gmail.com>")
        .about("Displays all request and returns 200")
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .help("Port")
                .takes_value(true),
        )
        .get_matches();

    let port = matches.value_of("port").unwrap_or("3000");

    let address = format!("127.0.0.1:{}", &port);

    match TcpListener::bind(&address) {
        Ok(listener) => {
            println!("Listening on port: {}", &port);
            for stream in listener.incoming() {
                count = count + 1;
                let now: DateTime<Local> = Local::now();
                let c_now = format!("{}", now.format("%H:%M:%S")).yellow();
                let c_count = format!("({})", &count).blue();

                println!("\n{}  {}\n", c_now, c_count);
                let stream = stream.unwrap();
                handle_connection(stream);
            }
        },
        Err(_) => {
            println!("Could not bind to port {}", &port);
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let content = String::from_utf8_lossy(&buffer[..]);
    let arr = content.split("\n\r").collect::<Vec<&str>>();
    let headers = arr.get(0).unwrap();
    let body = arr.get(1);

    println!("{}", headers);

    if let Some(b) = body {
        println!("");
        let sanitized = b.trim_matches('\u{0}').trim_matches('\n');

        let json: Result<Value> = serde_json::from_str(&sanitized);

        match json {
            Ok(j) => {
                println!("{:#}", jsonxf::pretty_print(&j.to_string()).unwrap());
            }
            Err(_) => {
                println!("{}", b);
            }
        }
    }

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("");
    println!("---------- 200 OK ---------");
}
