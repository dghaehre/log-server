//! Displays all request and returns 200 if nothing else is specified
extern crate log;
extern crate simplelog;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use clap::{App, Arg};
use log::{error, info};
use serde_json::{Result, Value};
use simplelog::*;

fn main() {
    let matches = App::new("log-server")
        .version("1.1")
        .author("Daniel Hæhre <dghaehre@gmail.com>")
        .about("A simple log server, that logs all requests and always returns 200")
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .help("Port")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .help("Write to file instead of stdout/stderr")
                .takes_value(true),
        )
        .get_matches();

    let port = matches.value_of("port").unwrap_or("3000");
    let file = matches.value_of("file");
    let address = format!("127.0.0.1:{}", &port);

    // Init logger
    match file {
        Some(f) => {
            let _ = WriteLogger::init(
                LevelFilter::Debug,
                Config::default(),
                File::create(f).unwrap(),
            );
        }
        None => {
            let _ = SimpleLogger::init(LevelFilter::Debug, Config::default());
        }
    };

    match TcpListener::bind(&address) {
        Ok(listener) => {
            info!("Listening on port: {}", &port);
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                handle_connection(stream);
            }
        }
        Err(_) => {
            error!("Could not bind to port {}", &port);
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

    info!("{}", headers);

    if let Some(b) = body {
        let sanitized = b.trim_matches('\u{0}').trim_matches('\n');

        let json: Result<Value> = serde_json::from_str(&sanitized);

        match json {
            Ok(j) => {
                info!("{}", jsonxf::pretty_print(&j.to_string()).unwrap());
            }
            Err(_) => {
                info!("{}", sanitized);
            }
        }
    }

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
