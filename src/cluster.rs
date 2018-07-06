use std::net::TcpStream;
use std::io::BufReader;
use std::io::prelude::{BufRead, Write};
use std::fs::File;
use toml;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Cluster {
    server: String,
    callsign: String,
}

impl Cluster{
    pub fn new (server: &str, call: &str) -> Cluster {
        Cluster {server: String::from(server), callsign: String::from(call)}
    }

    pub fn load_config() -> Option <Cluster> {
        let config_location = "config.toml";
        let mut config: String = String::from("");

        let file = BufReader::new(File::open(config_location).expect("ERROR reading file")); 
        for line in file.lines(){
            if let Ok(line) = line  {
                config.push_str(line.as_ref());
                config.push('\n');
            };
        };
        println!("Loaded following configuration: \n{}",config ); //FIXME: remove this println
        let loaded = match toml::from_str(&config){
            Ok(n) => Some(n),
            Err(_) => None,
        };

        loaded
    }
}


/// Starts the DX Cluster and connects to it via the given cluster address and call
/// It repeatedly callss the get_callsign function with the filtered buffer
/// entries
pub fn connect(cluster: Cluster) {
    //Connect to dx-cluster server
    let mut stream = TcpStream::connect(cluster.server).expect("Can't connect to Cluster");
    //Write callsign to telnet server to start getting cluster messages.
    let corrected_call = cluster.callsign.to_owned() + "\n";
    let _ = stream.write(&corrected_call.as_bytes());

    let mut reader = BufReader::new(stream);
    let callsigns = ::open_callsignlist(::get_directory());
    println!("Connection: Success");
    loop {
        let mut buffer = String::new(); // Create a new Buffer
        reader.read_line(&mut buffer).unwrap(); //Fill up the Buffer
        //println!("{:?}", filter_entry(buffer));
        if let Some(i) = ::get_callsign(&filter_entry(buffer), callsigns.clone()) {
            println!("{}",i );
        };
    }
}

///Filters the cluster entries and returns a cleaned up vector of strings
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_entry_test() {
        let sample0 = String::from("DX de EA5WU-#:    3508.0  IK3VUU       CW 19 dB 20 WPM CQ             2149Z\r\n");
        let expected0: Vec<&str> = vec!["EA5WU-#:", "3508.0", "IK3VUU", "CW", "19", "dB", "20", "WPM", "CQ", "2149Z"];
        assert_eq!(filter_entry(sample0),expected0);

        let sample1 = String::from("DX de K8WHA:     14081.0  TG9AHM       CQ DX RTTY Correction Freq     2150Z\r\n");
        let expected1: Vec<&str> = vec!["K8WHA:", "14081.0", "TG9AHM", "CQ", "RTTY", "Correction", "Freq", "2150Z"];
        assert_eq!(filter_entry(sample1),expected1);
    }
}