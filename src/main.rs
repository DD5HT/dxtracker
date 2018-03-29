//TODO:
//Write tests
use std::io::prelude::*;
use std::net::TcpStream;

static CLUSTER: &str = "cluster.dl9gtb.de:8000";
static CALL: [u8; 6] = *b"dd5ht\n";

fn main() {
    //let haha = filter_entry("DX de CT1BOH-#:   7008.9  S51KD        CW 13 dB 23 WPM CQ             2153Z");
    //println!("{:?}",haha );
    cluster();
}

fn filter_entry(entry: &str) -> Vec<&str> {
    // malformated entry possible length could cause faults
    let mut output: Vec<&str> = Vec::with_capacity(16); // add vector size?
    entry.split(" ")
         .filter(|&t| match t {
             "de" => false,
             "DX" => false,
             _ => true,
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
    //readbuffer
    let mut output = [0;256];

    loop {
        let _ = stream.read(&mut output); //why is there not read until?
        let mut text = String::new();

        output.iter().for_each(|x| text.push(*x as char)); //filter here for even faster code

        let mut messages = text.split("\r\n").filter(|y| y.contains("DX de"));
        
        match messages.next(){
            None => (),
            Some(x) => println!("{:?}",filter_entry(x) )
        }
    }
}