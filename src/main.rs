use std::process::Command;
use std::{
    io::{prelude::*},
    net::{TcpListener, TcpStream},
};

fn main()  {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let status_line = "HTTP/1.1 200 OK";

    let mut termux_battery_status = Command::new("termux-battery-status");
    let output: std::process::Output = termux_battery_status.output().expect("termux-battery-status output with phone stats");
    let contents = String::from_utf8(output.stdout).expect("Our bytes should be valid utf8");

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}