#![deny(unsafe_code)]
#![deny(warnings)]

use std::fs::File;
use std::fs::{DirBuilder, OpenOptions};
use std::io::prelude::*;
use std::io::BufReader;
use std::io::ErrorKind::AlreadyExists;
use std::path::PathBuf;

pub mod cluster;
pub mod dxcc_filter;

mod call_filter;

///Takes a formated dxcluster str vector and the list of all callsigns
///looks if callsign from spotted cluster is in list
/// # Example:
/// ```
///use dxtracker::get_callsign;
///
///let entry: Vec<&str> = vec!["EA5WU-#:", "3508.0", "IK3VUU", "CW", "19", "dB", "20", "WPM", "CQ", "2149Z"];
///let searchlist: Vec<String> = vec![String::from("IK3VUU")];
///assert_eq!(get_callsign(&entry, &searchlist).is_some(), true);
/// ```
pub fn get_callsign<T: AsRef<str>>(entry: &[T], searchlist: &[String]) -> Option<String> {
    if entry.len() > 3 {
        let spotter = entry[0].as_ref().trim_end_matches("-#:");
        let call = entry[2].as_ref();
        let freq = entry[1].as_ref();
        let mode = entry[3].as_ref();
        match searchlist.iter().find(|&x| x == call) {
            Some(c) => Some(format!(
                "Spotted {} on {} by {} in {}",
                c, freq, spotter, mode
            )),
            None => None,
        }
    } else {
        None
    }
}

///Opens the given list file and return a Vector with all the Callsigns
pub fn open_callsignlist(list: PathBuf) -> Vec<String> {
    let file = BufReader::new(File::open(list).expect("ERROR reading file"));
    let mut calls: Vec<String> = Vec::new();
    for line in file.lines() {
        match line {
            Ok(l) => {
                if !l.is_empty() {
                    calls.push(l)
                }
            }
            Err(e) => println!("Ups: {}", e),
        }
    }
    calls
}

///Inserts a call into the Callsign csv list returns the call if it was successful.
pub fn insert_call(call: &str) -> Result<String, String> {
    check_call(call)?;

    let mut new_call = call.to_uppercase();
    let list = open_callsignlist(get_call_path());
    if list.contains(&new_call) {
        Err(format!("{} is already in the callsign list!", new_call))
    } else {
        new_call.push_str("\n");
        let mut file = OpenOptions::new()
            .append(true)
            .open(get_call_path())
            .expect("Can't insert Callsign: Can't open file");
        file.write_all(new_call.as_bytes())
            .expect("Can't insert Callsign: Can't write to file");
        Ok(call.to_uppercase())
    }
}

///Removes a given call and returns it if it was successful.
pub fn remove_call(call: &str) -> Result<String, String> {
    let list = open_callsignlist(get_call_path());
    let newcall = call.to_string().to_uppercase();

    if list.contains(&newcall) {
        let newlist = list.clone();
        let output = newlist.iter().filter(|&x| x == &newcall);

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(get_call_path())
            .expect("Can't open file");

        for i in output {
            let content = format!("{}\n", i);
            file.write_all(content.as_bytes()).unwrap();
        }
        return Ok(call.to_uppercase());
    }
    Err("Can not remove the callsign!".to_string())
}

///Creates the callsign list at the default location: ~/.dxtool/calls.csv
pub fn create_list() -> Result<usize, String> {
    if get_call_path().exists() {
        Ok(1)
    } else {
        match File::create(get_call_path()) {
            Ok(mut file) => Ok(file.write(b"#######\n").unwrap()),
            Err(err) => Err(err.to_string()),
        }
    }
}

//TODO: Add arguments for the dir build
///Creates a directory for the given path
pub fn dir_build() -> Result<String, String> {
    match DirBuilder::new().create(get_tool_path()) {
        Ok(_) => Ok(get_tool_path().to_str().unwrap().to_owned()),
        Err(err) => match err.kind() {
            AlreadyExists => Ok(get_tool_path().to_str().unwrap().to_owned()),
            _ => Err(err.to_string()),
        },
    }
}

///Checks if call is invalid
fn check_call(call: &str) -> Result<&str, String> {
    if call.len() < 3 || call.len() > 20 {
        Err(String::from("Invalid call format!"))
    } else {
        Ok(call)
    }
}

///Returns the PathBuf for the default Path
fn get_tool_path() -> PathBuf {
    let mut path = PathBuf::new();
    match dirs::home_dir() {
        Some(x) => path.push(x),
        None => panic!("Can't fine home directory"),
    };
    path.push(".dxtool/");
    path
}

///Returns the PathBuf for the calls.csv
pub fn get_call_path() -> PathBuf {
    let mut path = get_tool_path();
    path.push("calls.csv");
    path
}

///Return the PathBuf for the config.toml
pub fn get_config_path() -> PathBuf {
    let mut path = get_tool_path();
    path.push("config.toml");
    path
}
