extern crate rayon;
#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::net::TcpStream;
use std::io::prelude::*;
use std::io::BufReader;

use rayon::prelude::*;

static CLUSTER: &str = "cluster.dl9gtb.de:8000";
static CALL: [u8; 6] = *b"dd5ht\n";

lazy_static! {
    static ref CALLS: Vec<String> = open_callsignlist(); 
}

fn main() {
    println!("   RUST DX Tracker: ");
    println!("      By DD5HT");
    println!("=======================");
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

fn get_callsign<T: AsRef<str>>(entry: &[T]) {
    //let sample: Vec<&str> = vec!["DD5HT","EI9KF","DL3LAR","HA3FTV","HA0NAR","SM0RRX","RA6QM","RU3X"];
    let sample = CALLS.clone();
    if entry.len() > 3 {
        let spotter = entry[0].as_ref();
        let call = entry[2].as_ref();
        let freq = entry[1].as_ref();
        match sample.into_par_iter().find_any(|x| x == call) {
        Some(c) => println!("Spotted {} on {} by {}",c, freq, spotter),
        None => () ,
        }
    }
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
        //println!("{:?}",filter_entry(output));
        //let x = output.clone();
           get_callsign(&filter_entry(output));
           //println!("", );
    }
}

fn open_callsignlist() -> Vec<String> {
    let file = BufReader::new(File::open("calls.csv").expect("ERROR reading file"));
    let mut calls: Vec<String> = Vec::new(); 
    for line in file.lines() {
        match line {
            Ok(l) => calls.push(l),
            Err(e) => println!("Ups: {}",e ),
        }{}
    }
    println!("Loaded following calls: {:?}", calls );
    calls
}