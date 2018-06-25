#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::net::TcpStream;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::OpenOptions;

static CLUSTER: &str = "cluster.dl9gtb.de:8000";
static CALL: [u8; 6] = *b"dd5ht\n";

lazy_static! {
    static ref CALLS: Vec<String> = open_callsignlist(); 
}


/**fn main() {
    println!("==========================================================================================================");
    println!("                                       RUST DX Tracker: ");
    println!("                                          By DD5HT");
    println!("==========================================================================================================");
    //TODO add samples to real tests
    insert_call("DL3LAR");
    insert_call("DP4B");
    insert_call("DD5HT");
    cluster();
}*/
//TODO
//Add function to clean obvious malformated entries

///Filters the cluster entries and returns a clean vector of strings
fn filter_entry(entry: String) -> Vec<String> {
    let mut output: Vec<String> = Vec::with_capacity(16); //Malfromated entries can lead to new memory allocation
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
        let spotter = entry[0].as_ref().trim_right_matches("-#:");
        let call = entry[2].as_ref();
        let freq = entry[1].as_ref();
        let mode = entry[3].as_ref();
        match sample.into_iter().find(|x| x == call) {
        Some(c) => println!("Spotted {} on {} by {} in {}",c, freq, spotter, mode),
        None => () ,
        }
    }
}

pub fn cluster() {
    //Connect to dx-cluster server
    let mut stream = TcpStream::connect(CLUSTER).unwrap();
    //Write callsign to telnet server to start getting cluster messages.
    let _ = stream.write(&CALL);

    let mut reader = BufReader::new(stream);
    loop {
        let mut buffer = String::new(); // Create a new Buffer
        reader.read_line(&mut buffer).unwrap(); //Fill up the Buffer
        get_callsign(&filter_entry(buffer));  //Put the Buffer into filter function

    }
}

fn open_callsignlist() -> Vec<String> {
    let file = BufReader::new(File::open("calls.csv").expect("ERROR reading file"));
    let mut calls: Vec<String> = Vec::new(); 
    for line in file.lines() {
        match line {
            Ok(l) =>  if !l.is_empty() {calls.push(l)},
            Err(e) => println!("Ups: {}",e ),
        }
    }
    println!("Loaded the following calls: {:?}", calls );
    calls
}

pub fn insert_call(call: &str) ->Option<&str>{ //ADD Result as return mabye?
    //let mut new_call = String::from("DD5HT");
    let mut new_call = String::from(call);
    let list = open_callsignlist();
    if list.contains(&new_call) {
        println!("{} is already in callsign list!", new_call );
        return None;
    }
    else {
        println!("Inserting: {}", new_call );
        new_call.push_str("\n");
        let mut file = OpenOptions::new()
        .append(true)
        .open("calls.csv")
        .unwrap();
        file.write_all(new_call.as_bytes()).expect("Cant write to file");
        return Some(call);
    }
}

fn remove_call(call: &str) {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_entry_test() {
        let sample0 = String::from("DX de EA5WU-#:    3508.0  IK3VUU       CW 19 dB 20 WPM CQ             2149Z");
        let expected0: Vec<&str> = vec!["EA5WU-#:", "3508.0", "IK3VUU", "CW", "19", "dB", "20", "WPM", "CQ", "2149Z"];
        assert_eq!(filter_entry(sample0),expected0);

        let sample1 = String::from("DX de K8WHA:     14081.0  TG9AHM       CQ DX RTTY Correction Freq     2150Z");
        let expected1: Vec<&str> = vec!["K8WHA:", "14081.0", "TG9AHM", "CQ", "RTTY", "Correction", "Freq", "2150Z"];
        assert_eq!(filter_entry(sample1),expected1);
    }
}