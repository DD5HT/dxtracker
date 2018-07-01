extern crate dxtracker;



#[test]
fn cluster_insert_remove() {
    dxtracker::create_list("PLACEHOLDER").unwrap();
    let call = "TeStCaLl";
    let formated_call = call.to_uppercase(); //TODO: add assert for exists
    assert_eq!(dxtracker::insert_call(call), Ok(formated_call.clone()));
    assert_eq!(dxtracker::remove_call(call), Ok(formated_call));
}

#[test]
fn open_callsign_list() {
    let list = dxtracker::CALLS;

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