//!This is the command line client for the dxtracker
//! use dxtool -h for help
use clap::{App, Arg};
use dxtracker::cluster::*;
use dxtracker::*;

fn main() {
    let matches = App::new("DX Tool")
        .version("0.1.2")
        .author("Hendrik, DD5HT, <hendrik@dd5ht.de>")
        .about("Connects to the DX Cluster via telnet and and filters it via a custom list")
        .arg(
            Arg::with_name("START")
                .short("s")
                .long("start")
                .help("Starts the DXTOOL"),
        )
        .arg(
            Arg::with_name("LIST")
                .short("l")
                .long("list")
                .help("Shows you all callsigns contained in the list"),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ADD")
                .short("a")
                .long("add")
                .value_name("CALL")
                .help("Adds a new callsign to the list")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("REMOVE")
                .short("r")
                .long("remove")
                .value_name("CALL")
                .help("Removes a callsign from the list")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INIT")
                .short("i")
                .long("init")
                .help("Init the program //Temporary solution"),
        )
        .get_matches();

    if matches.is_present("LIST") {
        println!("Following callsigns are in the list: ");
        open_callsignlist(get_call_path())
            .into_iter()
            .skip(1)
            .for_each(|x| println!("{}", x));
    }

    if matches.is_present("START") {
        match Cluster::load_config() {
            Some(x) => connect(x),
            None => panic!("Can't load config. \n Please create a config file first!"),
        }
    }
    /*
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);
    */

    if let Some(call) = matches.value_of("ADD") {
        match insert_call(call) {
            Ok(i) => println!("Added {} to the callsign list.", i),
            Err(e) => println!("{}", e),
        }
    };

    if let Some(call) = matches.value_of("REMOVE") {
        match remove_call(call) {
            Ok(i) => println!("Removed {} from the callsign list.", i),
            Err(e) => println!("{}", e),
        }
    };

    if matches.is_present("INIT") {
        init();
    };

    // TODO: ADD INIT for first boot up?
    //Set default server and callsign
    // Create Folder, Create Callsign list, Create default config?
    // Read in Config
}

fn init() {
    let servername = "cluster.dl9gtb.de:8000";
    let callsign = "DD5HT";
    match dxtracker::dir_build() {
        Ok(_) => println!("Successfully created the directory!"),
        Err(err) => println!("Failed to create dir: {}", err),
    }

    match dxtracker::cluster::Cluster::new(servername, callsign).init_config() {
        Ok(_) => println!("Init Successful"),
        Err(err) => println!("Error: {}", err),
    };
}
