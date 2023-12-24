#[warn(clippy::pedantic)]

use std::net;
use std::{io::prelude::*, time::Duration};

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
/// repeat/random/single/consume - toggles mpd state
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
    // NOTE: Connection buffer reading times out but the contents are still
    //       read?
    connection.set_read_timeout(Some(Duration::from_millis(50)))?;

    type ArgAction = fn(String) -> Result<String, std::io::Error>;
    let arg_function: ArgAction = |_: String| {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Option expected handler function none provided"
        ));
    };
    let arg_input = false;

    for arg in args {
        if arg_input {
            arg_function(arg)?;
            continue;
        }
        match arg.as_str() {
            "help" => {
                todo!();
            },
            "help" => {
                todo!();
            },
            "help" => {
                todo!();
            },
            "help" => {
                todo!();
            },
            "help" => {
                todo!();
            },
            "help" => {
                todo!();
            },
            "help" => {
                todo!();
            },
            "help" => {
                todo!();
            },
            _ => {
                println!("Invalid argument: {}", arg);
            }
        }
    }
    connection.write(b"pause\n")?;
    let mut str_buff = String::new();
    let _ = connection.read_to_string(&mut str_buff);
    print!("{}", str_buff);
    return Ok(());
}
