extern crate getopts;
use getopts::getopts;
use std::process::Command;
use std::net::{TcpListener,TcpStream};
use std::env;
use std::thread;


fn main() {
    let args: Vec<String> = env::args().collect();
    let opts = [
        optflag("d", "daemon", "conver this into a daemon"),
    ];
    let matches = match getopts(args.tail(), &opts) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) }
    };

    // Create a daemon? if necessary
    /*if matches.opt_present("d") {
        let child = Command::new(args[0].as_slice())
                            .detached().spawn().unwrap();
        println!("Created child: {}", child.id());
        child.forget();
        return;
    }*/

   let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    fn handle_client(stream: TcpStream) {
        // ...
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