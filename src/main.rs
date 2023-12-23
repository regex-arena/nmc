#[warn(clippy::pedantic)]

use std::net;
use std::io::prelude::*;

/// Runs all given commands
/// 
/// Possible arguments:
///
/// TODO
/// help - Prints help screen
/// discard - removes curently playing song
/// status - Same as no arguments: Prints mpd status screen
/// toggle - Toggles mpd playback
/// volume - changes mpd volume
/// playlist - outputs mpd playlist with index numbers
/// repeat/random/single/consume - toggles mpd 
/// add - adds given files: seperated by comma
///
/// -p/--port - changes mpd port from default 6600
/// -h/--host - changes mpd host from default 127.0.0.1
fn main() -> std::io::Result<()>{
    let mut host = "127.0.0.1".to_string();
    let mut port = "6600".to_string();
    let mut args: Vec<String> = vec![];

    // Parse inputs
    std::env::args().skip(1).for_each(|arg| {
        if host == "" {
            host = arg;
            return;
        } else if port == "" {
            port = arg;
            return;
        } else if arg == "-p" || arg == "--port" {
            port = "".to_string();
            return;
        } else if arg == "-h" || arg == "--host" {
            host = "".to_string();
            return;
        }
        args.push(arg);
    });

    // Connect to mpd
    let mut connection = net::TcpStream::connect(
        format!("{}:{}", host, port)
    ).expect("Unable to connect to mpd server");

    connection.write_all(b"command_list_begin\nstatus\ncommand_list_end").unwrap();
    connection.flush().unwrap();
    let mut buff = Vec::new();
    connection.read_to_end(&mut buff).unwrap();
    return Ok(());
}
