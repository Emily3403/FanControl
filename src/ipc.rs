use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::process::Command;
use std::time::Duration;
use log::{debug, info, warn};
use nix::sys::signal;
use nix::sys::signal::Signal;
use nix::unistd::Pid;
use serde::{Serialize, Deserialize};
use crate::EXPECTED_MESSAGE_SIZE;

const SOCKET_ADDR: &'static str = "/tmp/fwctrl.sock";
const READ_TIMEOUT: u64 = 5;  // In s

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ClientMessage {
    IsAlive,
    Test,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ServerMessage {
    Ok,
    Err,
}


fn is_server_active() -> bool {
    // First check if we can connect. If not, there is no chance that the server is active.
    let client = UnixStream::connect(SOCKET_ADDR);
    let Ok(mut client) = client else {
        debug!("Can't connect to server (connect failed) - aborting!");
        return false;
    };

    let duration = Duration::from_secs(READ_TIMEOUT);
    client.set_read_timeout(Some(duration)).unwrap();


    // Find out if a program is connected to the socket and if so, get the process ID.
    let mut cmd = Command::new("lsof");
    cmd.args(["-t", SOCKET_ADDR]);

    let Ok(status) = cmd.status() else {
        debug!("Can't connect to server (lsof failed) - aborting!");
        return false;
    };

    if !status.success() {
        debug!("Can't connect to server (lsof exit-code failed) - aborting!");
        return false;
    }

    let output = String::from_utf8(cmd.output().unwrap().stdout).unwrap();
    let output = output.strip_suffix('\n').unwrap();
    debug!("{:?}", output);
    let pid: i32 = output.parse().unwrap();


    // Now check if the server responds to a IsAlive message
    debug!("Server passed all sanity checks, sending IsAlive message");
    let encoded = bincode::serialize(&ClientMessage::IsAlive).unwrap();
    client.write_all(&encoded).unwrap();

    let mut response = vec![0; *EXPECTED_MESSAGE_SIZE];

    if let Err(_) = client.read_exact(&mut response) {
        debug!("Can't connect to server (timeout)! Killing the process and taking its place");
        signal::kill(Pid::from_raw(pid), Signal::SIGKILL).unwrap();
        return false;
    }

    let response: ServerMessage = bincode::deserialize(&response).unwrap();
    let is_alive = response == ServerMessage::Ok;

    if !is_alive {
        debug!("Can't connect to server (wrong response)! Killing the process and taking its place");
        signal::kill(Pid::from_raw(pid), Signal::SIGKILL).unwrap();
    }

    is_alive
}

pub fn connect_as_server() -> Result<UnixListener, std::io::Error> {
    info!("Starting to connect as a server");
    let listener = UnixListener::bind(SOCKET_ADDR);

    match listener {
        Ok(it) => { Ok(it) }

        Err(_) => {
            // Either there is a program running or the socket file was not properly cleaned up.
            warn!("Found existing socket file! Checking if the server is active");
            if !is_server_active() {
                info!("The server is not active, removing old socket file");
                std::fs::remove_file(SOCKET_ADDR).unwrap_or_default();
            }

            UnixListener::bind(SOCKET_ADDR)
        }
    }
}

pub fn connect_as_client() -> Result<UnixStream, std::io::Error> {
    UnixStream::connect(SOCKET_ADDR)
}