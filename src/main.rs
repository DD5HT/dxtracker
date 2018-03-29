//TODO:
//Write tests
use std::io::prelude::*;
use std::net::TcpStream;
use std::io::BufReader;
static CLUSTER: &str = "cluster.dl9gtb.de:8000";
static CALL: [u8; 6] = *b"dd5ht\n";

fn main() {
    cluster();
}

fn filter_entry(entry: String) -> Vec<String> {
    // malformated entry possible length could cause faults
    let mut output: Vec<String> = Vec::with_capacity(16); // add vector size?
    entry.trim_right_matches("\r\n")
         .split(" ")
         .filter(|&t| match t {
             "de" => false,
             "DX" => false,
             _    => true,
             })
         .filter(|t| !t.is_empty())
         .for_each(|x| output.push(String::from(x)));
    output
}
fn cluster() {
    //Connect to dx-cluster server
    let mut stream = TcpStream::connect(CLUSTER).unwrap();
    //Write callsign to telnet server to start getting cluster messages.
    let _ = stream.write(&CALL);

    let mut reader = BufReader::new(stream);
    loop {
        let mut output = String::new();
        reader.read_line(&mut output).unwrap();
        println!("{:?}",output)
    }
}
