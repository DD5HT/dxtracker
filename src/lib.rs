#![feature(vec_remove_item)]

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::{OpenOptions, DirBuilder};
use std::env;
use std::io::ErrorKind::AlreadyExists;
use std::path::PathBuf;

pub mod cluster;

/*
struct SPOT {
    call: String,
    freq: f64,
    mode: String,
    spotter: String,
}
*/
///Takes a formated dxcluster str vector and the list of all callsigns
///looks if callsign from spotted cluster is in list
pub fn get_callsign<T: AsRef<str>>(entry: &[T], searchlist: Vec<String>) -> Option<String>{
    if entry.len() > 3 {
        let spotter = entry[0].as_ref().trim_right_matches("-#:");
        let call = entry[2].as_ref();
        let freq = entry[1].as_ref();
        let mode = entry[3].as_ref();
        match searchlist.into_iter().find(|x| x == call) {
            Some(c) => Some(format!("Spotted {} on {} by {} in {}",c, freq, spotter, mode)),
            None => None ,
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
            Ok(l) =>  if !l.is_empty() {calls.push(l)},
            Err(e) => println!("Ups: {}",e ),
        }
    }
    calls
}

///Inserts a call into the Callsign csv list returns the call if it was successful.
pub fn insert_call(call: &str) -> Result<String, String> {
    
    check_call(call)?;

    let mut new_call = String::from(call.to_uppercase());
    let list = open_callsignlist(get_directory());
    if list.contains(&new_call) {
        return Err(format!("{} is alread in the callsign list!", new_call));
    }else {
        new_call.push_str("\n");
        let mut file = OpenOptions::new()
            .append(true)
            .open(get_directory())
            .expect("Can't open file"); //TODO: Add better error Handling here
        file.write_all(new_call.as_bytes()).expect("Cant write to file");
        return Ok(call.to_uppercase());
    }
}

///Removes a given call and returns it if it was successful.
pub fn remove_call(call: &str) -> Result<String, String> {   
    let list = open_callsignlist(get_directory());
    let newcall = call.to_string().to_uppercase();
    
    if list.contains(&newcall) {
        let mut newlist = list.clone();
        newlist.remove_item(&newcall).unwrap();

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(get_directory())
            .expect("Can't open file");

        for i in newlist {
            let content = i + "\n";
            file.write(content.as_bytes()).unwrap();
        }
        return Ok(call.to_uppercase());
    }
    Err("Can not remove the callsign!".to_string())
}

fn reset_list() -> Result<String, String> {
    unimplemented!()
}
//maybe result IO error?
pub fn create_list(listname: &str) -> Result<&str, String> {
    //TODO: Check if list already exists and just skipp all steps here
    let mut default_path = String::from("");
    match env::home_dir() {
        Some(path) => default_path =  path.to_str().unwrap().to_string() + "/.dxtool",
        None => println!("Impossible to get your home dir!"),
    }
    match DirBuilder::new().create(default_path.clone()){
        Ok(_) => {},
        Err(err) => match err.kind() {
            AlreadyExists => println!("File Already exists, skipping!"),
            _ => return Err(err.to_string()),
        },
    };

    let mut file = File::create(get_directory()).unwrap();
        file.write(b"#######\n").unwrap();

    Ok("Created default folder")
}

///Checks if call invalid
fn check_call(call:&str) -> Result<&str, String> {
    if call.len() < 3 || call.len() > 20 {
        return Err(String::from("Invalid call format!"));
    } else {
        Ok(call)
    }
}

pub fn get_directory() -> PathBuf {
    let mut path = PathBuf::new();
    path.push(env::home_dir().unwrap());
    path.push(".dxtool/calls.csv");
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_callsign_found() {
        let entry: Vec<&str> = vec!["EA5WU-#:", "3508.0", "IK3VUU", "CW", "19", "dB", "20", "WPM", "CQ", "2149Z"];
        let searchlist: Vec<String> = vec![String::from("IK3VUU")];
        assert_eq!(get_callsign(&entry, searchlist).is_some(), true);
    }
    
    #[test]
    fn get_callsign_not_found() {
        let entry: Vec<&str> = vec!["EA5WU-#:", "3508.0", "IK3VUU", "CW", "19", "dB", "20", "WPM", "CQ", "2149Z"];
        let searchlist: Vec<String> = vec![String::from("NOCALL")];
        assert_eq!(get_callsign(&entry, searchlist).is_none(), true);
    }
    /*
    #[test]
    fn open_callsignlist_test() {
    }
    */
}