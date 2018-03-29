//TODO:
//Write tests
use std::io::prelude::*;
use std::net::TcpStream;

static CLUSTER: &str = "cluster.dl9gtb.de:8000";
static CALL: [u8; 6] = *b"dd5ht\n";

fn main() {
    cluster();
}

fn filter_entry(entry: &str) -> Vec<&str> {
    // malformated entry possible length could cause faults
    let mut output: Vec<&str> = Vec::with_capacity(16); // add vector size?
    entry.trim_right_matches("\r")
         .split(" ")
         .filter(|&t| match t {
             "de" => false,
             "DX" => false,
             _    => true,
             })
         .filter(|t| !t.is_empty())
         .for_each(|x| output.push(x));
    output
}

fn cluster() {
    //Connect to dx-cluster server
    let mut stream = TcpStream::connect(CLUSTER).unwrap();
    //Write callsign to telnet server to start getting cluster messages.
    let _ = stream.write(&CALL);
    //Create String to save data:
    let mut dx_data = String::with_capacity(128);
    // Open a byte stream and write bytes to String
    for byte in stream.bytes() {
        match byte {
            Ok(10) => dx_data = print_reset(dx_data),//reset String so it wont grow indefinitely
            Ok(_)  => dx_data.push(byte.unwrap() as char),
            Err(e) => println!("{}",e ),
        }
    }
}

fn print_reset(input: String) -> String {
    //prints the String and resets it by returning an empty String.
    println!("{:?}", filter_entry(&input));
    String::from("")
}