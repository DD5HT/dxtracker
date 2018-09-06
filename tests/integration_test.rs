//! update tests to use a tempdir so it will not conflict with user dir
//! add tempdir as dev dependency
extern crate dxtracker;

#[test]
fn cluster_insert_remove() {
    open_callsign_list();
    let call = "TeStCaLl";
    let formated_call = call.to_uppercase();
    
    assert!(dxtracker::dir_build().is_ok());
    assert!(dxtracker::create_list().is_ok());
    assert_eq!(dxtracker::insert_call(call), Ok(formated_call.clone()));
    assert_eq!(dxtracker::remove_call(call), Ok(formated_call));
}

#[test]
fn load_config() {
    use dxtracker::cluster::Cluster;

    create_config();

    let pre = Cluster::new("cluster.dl9gtb.de:8000", "DD5HT");
    assert_eq!(Cluster::load_config(), Some(pre));
}

// Helper functions

fn create_directory() {
    assert!(dxtracker::dir_build().is_ok());
}

fn open_callsign_list() {
    create_directory();

    assert!(dxtracker::create_list().is_ok());
    let list = dxtracker::get_call_path();

    assert_eq!(dxtracker::open_callsignlist(list), vec!["#######"]);
}

fn create_config() {
    use dxtracker::cluster::Cluster;
    create_directory();

    let pre = Cluster::new("cluster.dl9gtb.de:8000", "DD5HT").init_config();
    assert!(pre.unwrap() > 0); //unwrap is safe here
}
