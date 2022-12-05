use std::{fs, io};
use std::error::Error;
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::process::{Command, Stdio};
use std::time::Duration;
use log::{debug, info, warn};
use nix::sys::signal;
use nix::sys::signal::Signal;
use nix::unistd::Pid;
use serde::{Serialize, Deserialize};
use crate::EXPECTED_MESSAGE_SIZE;

const SOCKET_ADDR: &'static str = "/tmp/fwctrl.sock";
const READ_TIMEOUT: Duration = Duration::from_secs(5);

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


fn get_pid_of_server() -> Option<i32> {
    let mut cmd = Command::new("lsof");
    cmd.stdout(Stdio::piped());
    cmd.args(["-t", SOCKET_ADDR]);

    let child = cmd.spawn().ok()?;
    let output = child.wait_with_output().and_then(
        |_| {
            match cmd.status() {
                Err(it) => Err(it),

                Ok(status) => {
                    if status.success() {
                        Ok(status)
                    }
                    else {
                        // Err(status)
                        todo!();
                    }
                },
            }
        }).ok()?;

    let Ok(status) = cmd.status() else {
        debug!("Can't get PID of server (lsof failed) - aborting!");
        return None;
    };

    if !status.success() {
        debug!("Can't get PID of server (lsof exit-code failed) - aborting!");
        return None;
    }

    let output = String::from_utf8(cmd.output().ok()?.stdout).ok()?;
    let output = output.strip_suffix('\n')?;

    Some(output.parse().ok()?)
}


fn check_server_responds_is_alive(connection: &mut UnixStream) -> bool {
    debug!("Checking if the server responds to IsAlive message withing {:?}", READ_TIMEOUT);

    let encoded = bincode::serialize(&ClientMessage::IsAlive).unwrap();
    connection.write_all(&encoded).unwrap();

    let mut response = vec![0; *EXPECTED_MESSAGE_SIZE];

    if let Err(_) = connection.read_exact(&mut response) {
        debug!("Can't connect to server (timeout)!");
        return false;
    }

    let response: ServerMessage = bincode::deserialize(&response).unwrap();
    debug!("Server is OK");
    response == ServerMessage::Ok
}


fn is_server_active() -> bool {
    // First check if we can connect. If not, there is no chance that the server is active.
    let connection = connect_as_client();
    let Ok(mut connection) = connection else {
        debug!("Can't connect to server (connect failed) - aborting!");
        return false;
    };

    // Set Read timeout
    connection.set_read_timeout(Some(READ_TIMEOUT)).unwrap();

    // Get the PID of the running server (if there is one) for future use.
    let Some(pid) = get_pid_of_server() else {
        return false;
    };

    // Now check if the server responds to a IsAlive message
    let is_alive = check_server_responds_is_alive(&mut connection);

    if !is_alive {
        debug!("Can't connect to server (wrong response)! Killing the process and taking its place");
        signal::kill(Pid::from_raw(pid.into()), Signal::SIGKILL).unwrap();
    }

    is_alive
}

pub fn connect_as_server() -> io::Result<UnixListener> {
    info!("Trying to connect as a server");
    let listener = UnixListener::bind(SOCKET_ADDR);

    if let Ok(it) = listener {
        return Ok(it);
    }

    // Either there is a program running or the socket file was not properly cleaned up.
    warn!("Found existing socket file! Checking if the server is active");
    if !is_server_active() {
        info!("The server is not active, removing old socket file");
        fs::remove_file(SOCKET_ADDR).unwrap_or_default();
    }

    // Retry binding. This might cause a race condition assuming two programs were startet at the same time, but it should be fine.
    UnixListener::bind(SOCKET_ADDR)
}


pub fn connect_as_client() -> io::Result<UnixStream> {
    UnixStream::connect(SOCKET_ADDR)
}