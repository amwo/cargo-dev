extern crate mime;
extern crate regex;

use mime::Mime;
use regex::Regex;

use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::thread;

fn handle(mut stream: TcpStream) {
    let mut buf = [0u8; 4096];
    //let re = Regex::new(r"(?<=^GET\s).*?(?=\sHTTP)").unwrap();
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);

            println!("============================\n{}\n====================================", req_str);

            let res = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello</body></html>\r\n";

            match stream.write(res) {
                Ok(_) => println!("Response sent"),
                Err(e) => println!("Failed sending res: {}", e),
            }
        }
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn body(p: &PathBuf) -> (&str, &str) {
    println!("{:?}", p);
    ("", "")
}

fn find_mimetype(filename: &String) -> Mime {
    let parts: Vec<&str> = filename.split('.').collect();

    let res = match parts.last() {
        Some(v) => match *v {
            "png" => mime::IMAGE_PNG,
            "jpg" => mime::IMAGE_JPEG,
            "json" => mime::APPLICATION_JSON,
            &_ => mime::TEXT_PLAIN,
        },
        None => mime::TEXT_PLAIN,
    };
    return res;
}

fn main() {
    let p = env::current_dir().unwrap();
    println!("{}", p.display());

    let listener = TcpListener::bind("127.0.0.1:1111").unwrap();
    println!("Listening for connections on port {}", 1111);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle(stream));
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
