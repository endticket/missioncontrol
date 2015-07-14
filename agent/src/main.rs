extern crate getopts;
use getopts::Options;
use std::process::Command;
use std::net::{TcpListener,TcpStream};
use std::env;
use std::thread;


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("d", "daemon", "convert this into a daemon");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    // Create a daemon? if necessary
    if matches.opt_present("d") {
        let child = Command::new(program).spawn().unwrap();
        //println!("Created child: {}", child.id());
        //child.forget();
        return;
    }

    let listener = TcpListener::bind("127.0.0.1:37565").unwrap();

    fn handle_client(stream: TcpStream) {

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