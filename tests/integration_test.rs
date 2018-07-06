extern crate dxtracker;

use std::{thread, time};


//use std::path::{Path, PathBuf};

/*
#[test]
fn check_path() {
    assert_eq!(dxtracker::get_directory(), PathBuf::from("/home/hendrik/.dxtool/calls.csv"));
}
*/
//FIXME: Tests have race condition!!!
//FIXME: Remove race condition for open_callsign_list , insert and remove!
#[test]
fn cluster_insert_remove() {
    //Hack
    let millis = time::Duration::from_millis(10);
    thread::sleep(millis);

    dxtracker::create_list().unwrap();
    let call = "TeStCaLl";
    let formated_call = call.to_uppercase(); //TODO: add assert for exists
    assert_eq!(dxtracker::insert_call(call), Ok(formated_call.clone()));
    assert_eq!(dxtracker::remove_call(call), Ok(formated_call));
}

#[test]
fn open_callsign_list() {
    dxtracker::create_list().unwrap();
    let list = dxtracker::get_call_path();

    assert_eq!(dxtracker::open_callsignlist(list), vec!["#######"]);
}

#[test]
fn create_config() {
    use dxtracker::cluster::Cluster;

    let pre = Cluster::new("cluster.dl9gtb.de:8000", "DD5HT").init_config();
    assert_eq!(pre, Ok("YOLO".to_owned()));
}

#[test]
fn load_config_test() {
    use dxtracker::cluster::Cluster;

    let pre = Cluster::new("cluster.dl9gtb.de:8000", "DD5HT");
    assert_eq!(Cluster::load_config(), Some(pre) );
}

/*
#[test]
fn cluster_connection_test() {
    use dxtracker::cluster::{connect};
    
    let call = "DD5HT";
    let cluster = "cluster.dl9gtb.de:8000";
    connect(cluster, call);

}
*/