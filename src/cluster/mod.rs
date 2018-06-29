use std::net::TcpStream;
use std::io::BufReader;
use std::io::prelude::{BufRead, Write};

use super::{CALLS, filter_entry, get_callsign};
/// Starts the DX Cluster and connects to it via the given cluster address and call
/// It repeatedly clals the get_callsign function with the filtered buffer
/// entries
pub fn connect(cluster: &str, call: &str) -> Result<String, String> {
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
        //TODO: add propper callsignlist instead of vec!["DD5HT"]
        get_callsign(&filter_entry(buffer),CALLS.to_vec());  //Put the Buffer into filter function
    }
    Ok(String::from("Worked"))
}
