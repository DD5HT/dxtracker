extern crate dxtracker;

#[test]
fn cluster_insertion_test() {
    //TODO: WIPE DATA before first insert
    let call = "TESTCALL";
    assert_eq!(dxtracker::insert_call(call), Some(call));

}
#[test]
fn cluster_connection_test(){
    let call = "DD5HT";
    let cluster = "cluster.dl9gtb.de:8000";
    assert_eq!(dxtracker::connect_to_cluster(cluster, call).is_ok(), true);
}