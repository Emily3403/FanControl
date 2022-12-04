use std::io::Write;
use std::os::unix::net::{UnixListener, UnixStream};
use serde::{Serialize, Deserialize};

const SOCKET_ADDR: &'static str = "/tmp/fwctrl.sock";

#[derive(Serialize, Deserialize)]
enum MessageTypes {
    IsAlive,
    Test,
}


fn is_server_active() -> bool {
    let client = UnixStream::connect(SOCKET_ADDR);

    let Ok(mut client) = client else {
        return false;
    };


    let encoded = bincode::serialize(&MessageTypes::Test).unwrap();
    client.write_all(&encoded);
    println!("Sending {:?}", &encoded);
    false
}

pub fn connect_as_server() -> Result<UnixListener, std::io::Error> {
    let mut listener = UnixListener::bind(SOCKET_ADDR);

    // TODO: Is the current program active?
    if !is_server_active() {
        std::fs::remove_file(SOCKET_ADDR).unwrap();

    }



    listener
}

pub fn connect_as_client() -> Result<UnixStream, std::io::Error> {
    UnixStream::connect(SOCKET_ADDR)
    // TODO: Is the current program active?
}