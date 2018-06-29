#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::OpenOptions;


pub mod cluster;

//remove static
lazy_static! {
    static ref CALLS: Vec<String> = open_callsignlist(); 
}

//TODO:
// use serde for serializing data
// At start of programm desizerlize all data
// serlize all data after each read and write
// solves 

/**fn main() {
   
}*/
//TODO:
//Add function to clean obvious malformated entries

//TODO: write TEST for function
//TODO: add return type
///Takes formated entries and filters them: entry and checks if callsign from
///searchlist is in entry
pub fn get_callsign<T: AsRef<str>>(entry: &[T], searchlist: Vec<String>) {
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
/// assert_eq!(insert_call(call),Ok(call));
/// ´´´
pub fn insert_call(call: &str) -> Result<&str, String> {
    if call.len() < 3 || call.len() > 20 {
        return Err(String::from("Invalid call format!"));
    }
    //TODO: add ascii filtering
    let mut new_call = String::from(call);
    let list = open_callsignlist();
    if list.contains(&new_call) {
        println!("{} is already in callsign list!", new_call );
        return Err(format!("{} is alread in callsign list!", new_call));
    }
    else {
        println!("Inserting: {}", new_call );
        new_call.push_str("\n");
        let mut file = OpenOptions::new()
            .append(true)
            .open("calls.csv")
            .unwrap(); //TODO: Add better error Handling here
        file.write_all(new_call.as_bytes()).expect("Cant write to file");
        return Ok(call);
    }
}
//FIXME:
fn remove_call(call: &str) -> Result<&str, &str> {
    let mut file = OpenOptions::new();
        
    unimplemented!();
}
//TODO: Remove TESTS?
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