#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::net::TcpStream;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::OpenOptions;

//remove static
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
//TODO write TEST for function
//TODO add return type
///Takes formated entries and filters them: entry and checks if callsign from
///searchlist is in entry
fn get_callsign<T: AsRef<str>>(entry: &[T], searchlist: Vec<String>) {
    if entry.len() > 3 {
        let spotter = entry[0].as_ref().trim_right_matches("-#:");
        let call = entry[2].as_ref();
        let freq = entry[1].as_ref();
        let mode = entry[3].as_ref();
        match searchlist.into_iter().find(|x| x == call) {
        Some(c) => println!("Spotted {} on {} by {} in {}",c, freq, spotter, mode),
        None => () ,
        }
    }
}
/// Starts the DX Cluster and connects to it via the given cluster address and call
/// It repeatedly clals the get_callsign function with the filtered buffer
/// entries
pub fn connect_to_cluster(cluster: &str, call: &str) -> Result<String, String> {
    //Connect to dx-cluster server
    let mut stream = TcpStream::connect(cluster).unwrap();
    //Write callsign to telnet server to start getting cluster messages.
    let _ = stream.write(&call.as_bytes());

    let mut reader = BufReader::new(stream);
    //Write no function for cluster
    //Add multithreading
    for _ in 0..10 {
        let mut buffer = String::new(); // Create a new Buffer
        reader.read_line(&mut buffer).unwrap(); //Fill up the Buffer
        //TODO add propper callsignlist instead of vec!["DD5HT"]
        get_callsign(&filter_entry(buffer),CALLS.to_vec());  //Put the Buffer into filter function
    }
    Ok(String::from("Worked"))
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
///Inserts a call into the Callsign csv list
/// ´´´
/// let call = "TESTCALL";
/// assert_eq!(insert_call(call),Some(call));
/// ´´´
pub fn insert_call(call: &str) -> Option<&str> {
    //TODO Filter callsign if it's to short or to long
    //2 < Callsign < 20
    //TODO Better error handling switch to result?
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
//FIXME
fn remove_call(call: &str) {
    unimplemented!();
}
//TODO Remove TESTS?
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