extern crate dxtracker;

#[test]
fn cluster_check() {
    //TODO WIPE DATA before first insert
    let call = "TESTCALL";
    assert_eq!(dxtracker::insert_call(call), Some(call));

}