extern crate dxtracker;

#[test]
fn cluster_insert_remove() {

    open_callsign_list();

    dxtracker::dir_build().expect("Can't create directory!");
    dxtracker::create_list().unwrap();
    let call = "TeStCaLl";
    let formated_call = call.to_uppercase(); //TODO: add assert for exists
    assert_eq!(dxtracker::insert_call(call), Ok(formated_call.clone()));
    assert_eq!(dxtracker::remove_call(call), Ok(formated_call));
}


#[test]
fn load_config() {
    use dxtracker::cluster::Cluster;
    
    create_config();
    
    let pre = Cluster::new("cluster.dl9gtb.de:8000", "DD5HT");
    assert_eq!(Cluster::load_config(), Some(pre) );
}


// Helper functions

///Wrapper function for system specific test 
/// FIXME: system unspecific
fn create_directory() {
    dxtracker::dir_build().unwrap();
    //assert_eq!(dxtracker::dir_build(), Ok("/home/hendrik/.dxtool/".to_owned()));
}

fn open_callsign_list() {
    create_directory();
    
    dxtracker::create_list().unwrap();
    let list = dxtracker::get_call_path();

    assert_eq!(dxtracker::open_callsignlist(list), vec!["#######"]);
}

fn create_config() {
    use dxtracker::cluster::Cluster;
    create_directory();

    let pre = Cluster::new("cluster.dl9gtb.de:8000", "DD5HT").init_config();
    assert_eq!(pre, Ok("YOLO".to_owned()));
}