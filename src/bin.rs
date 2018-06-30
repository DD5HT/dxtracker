extern crate dxtracker;
extern crate clap;

use dxtracker::cluster;
use clap::{Arg, App};

fn main() {
    let matches = App::new("DX Tool")
                          .version("0.1")
                          .author("Hendrik, DD5HT, <hendrik@dd5ht.de>")
                          .about("Connects to the dxcluster an reports filtered calls")
                          .arg(Arg::with_name("START")
                               .short("s")
                               .long("start")
                               .help("Starts the DXTOOL"))
                          .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("FILE")
                               .help("Sets a custom config file")
                               .takes_value(true))
                          .arg(Arg::with_name("ADD")
                               .short("a")
                               .long("add")
                               .value_name("CALL")
                               .help("Adds a new callsign to the list.")
                               .takes_value(true))
                          .arg(Arg::with_name("REMOVE")
                               .short("r")
                               .long("remove")
                               .value_name("CALL")
                               .help("Remove a callsign for the list.")
                               .takes_value(true))
                          .get_matches();


    if matches.is_present("START") {
        let call = "DD5HT";
        let cluster = "cluster.dl9gtb.de:8000";
        println!("Connecting to {} with callsign: {}", cluster.to_uppercase(), call);
        cluster::connect(cluster, call);
        println!("{}","jo" );
    }
   
    //let config = matches.value_of("config").unwrap_or("default.conf");
    //println!("Value for config: {}", config);

    if let Some(call) = matches.value_of("ADD") {
        //TODO: Add error handling for failing to insert a callsign
        match dxtracker::insert_call(call){
            Ok(i) => println!("Added {} to callsign list", i ),
            Err(e) => println!("{}",e),
        }
    };

    if let Some(call) = matches.value_of("REMOVE") {
        //TODO: Add error handling for failing to insert a callsign
        match dxtracker::remove_call(call){
            Ok(i) => println!("Removed {} from the callsign list", i ),
            Err(e) => println!("{}",e),
        }
    };

}