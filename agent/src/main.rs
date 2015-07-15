extern crate getopts;
use getopts::Options;
use std::process::Command;
use std::net::{TcpListener,TcpStream};
use std::env;
use std::thread;
use std::mem;
mod configparser;


fn string_to_static_str(s: String) -> &'static str {
    unsafe {
        let ret = mem::transmute(&s as &str);
        mem::forget(s);
        ret
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("d", "daemon", "convert this into a daemon");
    opts.optopt("l", "listen", "Listen on", "LISTEN");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let listen_on = match matches.opt_str("l") {
        Some(x) => string_to_static_str(x),
        None => "127.0.0.1:37565"
    };

    // Create a daemon? if necessary
    if matches.opt_present("d") {
        let child = Command::new(program).arg("-l").arg(listen_on).spawn().unwrap();
        return;
    }

    println!("Listening on {:?}", listen_on);

    let listener = TcpListener::bind(listen_on).unwrap();

    fn handle_client(stream: TcpStream) {
        //TODO
    }

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }

    // close the socket server
    drop(listener);

}