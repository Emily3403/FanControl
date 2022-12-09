use std::{fs, io};


use std::os::unix::net::{UnixListener, UnixStream};
use std::process::{Command, Stdio};
use std::time::Duration;
use log::{debug, info};
use nix::sys::signal;
use nix::sys::signal::Signal;
use nix::unistd::Pid;
use serde::{Serialize, Deserialize};
use crate::strategies::Strategy;
use crate::utils::{Percentage, Temperature};


const SOCKET_ADDR: &'static str = "/tmp/fwctrl.sock";
const READ_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ClientMessage {
    IsAlive,
    Status,
    Swap(Strategy),
    SetFanPercent(Percentage),
    Reset,
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ServerMessage {
    Ok,
    Status(Status),
    Err,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Status {
    temp: Temperature,
    fan_percent: Percentage,
}


pub fn get_message_from_client(stream: &UnixStream) -> Result<ClientMessage, bincode::Error> {
    let stream = stream.try_clone()?;
    bincode::deserialize_from(stream)
}

pub fn send_message_to_client(stream: &UnixStream, message: ServerMessage) -> Result<(), bincode::Error> {
    let stream = stream.try_clone()?;
    bincode::serialize_into(stream, &message)?;

    Ok(())
}

pub fn get_message_from_server(stream: &UnixStream) -> Result<ServerMessage, bincode::Error> {
    let stream = stream.try_clone()?;
    bincode::deserialize_from(stream)
}

pub fn send_message_to_server(stream: &UnixStream, message: ClientMessage) -> Result<(), bincode::Error> {
    let stream = stream.try_clone()?;
    bincode::serialize_into(stream, &message)?;

    Ok(())
}


fn get_pid_of_server() -> Option<i32> {
    let mut cmd = Command::new("lsof");
    cmd.args(["-t", SOCKET_ADDR]);
    cmd.stdout(Stdio::piped());

    let mut child = cmd.spawn().ok()?;
    child.wait().and_then(
        |_| {
            match cmd.status() {
                Err(it) => Err(it),

                Ok(status) => {
                    if status.success() {
                        Ok(status)
                    } else {
                        // Err(status)
                        todo!();
                    }
                }
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


fn check_server_responds_is_alive(stream: &mut UnixStream) -> bool {
    debug!("Checking if the server responds to IsAlive message within {:?}", READ_TIMEOUT);

    let Ok(()) = send_message_to_server(stream, ClientMessage::IsAlive) else {
        return false;
    };

    let Ok(response) = get_message_from_server(stream) else {
        return false;
    };

    response == ServerMessage::Ok
}


fn check_server_alive() -> bool {
    // First check if we can connect. If not, there is no chance that the server is active.
    let Ok(mut stream) = connect_as_client() else {
        debug!("Can't connect to server (connect failed) - aborting!");
        return false;
    };
    stream.set_read_timeout(Some(READ_TIMEOUT)).unwrap();

    // Get the PID of the running server (if there is one) for future use.
    let Some(pid) = get_pid_of_server() else {
        return false;
    };

    // Now check if the server responds to a IsAlive message
    let is_alive = check_server_responds_is_alive(&mut stream);

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
    debug!("binding failed, checking if the server is active");
    if !check_server_alive() {
        debug!("The server is not active, removing old socket file");
        fs::remove_file(SOCKET_ADDR).unwrap_or_default();
    }

    // Retry binding. This might cause a race condition assuming two programs were startet at the same time, but it should be fine.
    UnixListener::bind(SOCKET_ADDR)
}


pub fn connect_as_client() -> io::Result<UnixStream> {
    UnixStream::connect(SOCKET_ADDR)
}