extern crate dxtracker;

use dxtracker::*;

fn main() {
    println!("==========================================================================================================");
    println!("                                       RUST DX Tracker: ");
    println!("                                          By DD5HT");
    println!("==========================================================================================================");
    //TODO: add samples to real tests
    insert_call("DL3LAR").unwrap();
    insert_call("DP4B").unwrap();
    insert_call("DD5HT").unwrap();
    cluster::connect("", "").unwrap();
}