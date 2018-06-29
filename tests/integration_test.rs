extern crate dxtracker;

#[test]
fn cluster_insertion_test() {
    //TODO: WIPE DATA before first insert
    let call = "TESTCALL";
    assert_eq!(dxtracker::insert_call(call), Ok(call));

}
#[test]
fn cluster_connection_test(){
    use dxtracker::cluster::{connect};
    
    let call = "DD5HT";
    let cluster = "cluster.dl9gtb.de:8000";
    connect(cluster, call);

}