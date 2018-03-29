fn cluster_old() {
    //Connect to dx-cluster server
    let mut stream = TcpStream::connect(CLUSTER).unwrap();
    //Write callsign to telnet server to start getting cluster messages.
    let _ = stream.write(&CALL);
    //readbuffer
    let mut output = [0;256];

    loop {
        let _ = stream.read(&mut output); //why is there not read until?
        let mut text = String::new();

        output.iter().for_each(|x| text.push(*x as char)); 

        let mut messages = text.split("\r\n").filter(|y| y.contains("DX de"));

        match messages.next(){
            None => (),
            Some(x) => println!("{:?}",filter_entry(x) )
        }
    }
}