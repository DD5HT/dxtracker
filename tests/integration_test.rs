extern crate dxtracker;

#[test]
fn cluster_insertion_test() {
    let call = "TESTCALL";

    assert_eq!(dxtracker::insert_call(call), Ok(call));
    assert_eq!(dxtracker::remove_call(call), Ok(call));
}

#[test]
fn open_callsign_list_test() {
    let list = "calls.csv";

    assert_eq!(dxtracker::open_callsignlist(list), vec!["#######"]);
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