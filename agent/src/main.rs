extern crate getopts;
use getopts::Options;
use std::process::Command;
use std::net::{TcpListener,TcpStream};
use std::env;
use std::thread;
use std::io::prelude::*;
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
    opts.optopt("m", "master", "Connect to", "CONNECT");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let listen_on = match matches.opt_str("l") {
        Some(x) => string_to_static_str(x),
        None => "127.0.0.1:37565"
    };

    let master_connection = match matches.opt_str("m") {
        Some(x) => string_to_static_str(x),
        None => panic!("Please Specify your Missioncontrol master host".to_string())
    };

    // Create a daemon? if necessary
    if matches.opt_present("d") {
        let child = Command::new(program).arg("-l").arg(listen_on)
            .arg("-m").arg(master_connection).spawn().unwrap();
        return;
    }

    println!("Listening on {:?}", listen_on);
    println!("Connecting to Missioncontrol master on {:?}", master_connection);

    let listener = TcpListener::bind(listen_on).unwrap();
    let mut master_stream = TcpStream::connect(master_connection);

    fn handle_client_command(stream: TcpStream) {
        //TODO
    }

    fn handle_server_command(mut sstream: TcpStream) {
        let buf : &mut String = &mut "".to_string();
        //sstream.read_to_string(buf);
        sstream.read_to_string(buf).unwrap();
        println!("{:?}", buf);
    }

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client_command(stream)
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }

    match master_stream {
        Ok(master_data) => {
            thread::spawn(move|| {
                // connection succeeded
                handle_server_command(master_data)
            });
        }
        Err(e) => { panic!("Master connection interrupted".to_string()) }
    }


    // close the socket server
    drop(listener);
    //close the stream
    drop(master_stream);

}