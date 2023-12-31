use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::time::Duration;
use std::collections::HashMap;
use std::net::TcpStream;

/// Runs all given commands
/// 
/// Possible arguments:
/// help - Prints help screen
/// toggle - Toggles mpd playback
/// discard - removes curently playing song
/// status - Same as no arguments: Prints mpd status screen
/// playlist - outputs mpd playlist with index numbers
/// repeat/random/single/consume - toggles mpd state
/// update - updates mpd database
/// volume - changes mpd volume
/// add - adds given files: seperated by comma
/// remove - removes items at given indecies
///
/// -p/--port - changes mpd port from default 6600
/// -h/--host - changes mpd host from default 127.0.0.1

// Allows for defining methods for BufWriter struct\
trait MPDRead {
    fn read_mpd_end(&mut self, buffer: &mut String) -> Result<(), std::io::Error>;
}
impl MPDRead for BufReader<&TcpStream> {
    /// Read mpd responses - only to use for queries that mpd returns extra info
    /// on
    fn read_mpd_end(&mut self, buffer: &mut String) -> Result<(), std::io::Error> {
        loop {
            let tmp_buffer = &mut String::new();
            self.read_line(tmp_buffer)?;
            buffer.push_str(tmp_buffer.as_str());
            if tmp_buffer == "OK\n" {
                return Ok(());
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    // Default host and port
    let mut host = std::env::var_os("MPD_HOST")
        .unwrap_or("127.0.0.1".into()).to_str().unwrap_or("127.0.0.1").to_string();
    let mut port = std::env::var_os("MPD_PORT")
        .unwrap_or("6600".into()).to_str().unwrap_or("6600").to_string();
    let mut args: Vec<String> = vec![];

    // TODO: Set host and port if ENV variables are set

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
    if args.len() == 0 {
        args.push("status".to_string());
    }

    // Connect to mpd
    let connection = TcpStream::connect(
        format!("{}:{}", host, port)
    ).expect("Unable to connect to mpd server");
    // NOTE: Connection buffer reading times out but the contents are still read?
    connection.set_read_timeout(Some(Duration::from_millis(50)))?;
    let mut reader = BufReader::new(&connection);
    let mut writer = BufWriter::new(&connection);

    type ArgAction = fn (
        String,
        &mut BufReader<&TcpStream>,
        &mut BufWriter<&TcpStream>
    ) -> Result<(), Box<dyn std::error::Error>>;
    let mut arg_function: ArgAction = |_: _, _: _, _: _| {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Option expected handler function none provided"
        )));
    };
    let mut arg_input = false;

    let mut str_buff = String::new();
    // Run user commands
    for arg in args {
        if arg_input {
            arg_function(arg, &mut reader, &mut writer)?;
            arg_input = !arg_input;
            continue;
        }
        match arg.as_str() {
            "help" => {
                println!("Runs all given commands, from left to right");
                println!("");
                println!("Arguments:");
                println!("-p/--port - changes mpd port from default 6600");
                println!("-h/--host - changes mpd host from default 127.0.0.1");
                println!("");
                println!("Commands");
                println!("help      - Prints help screen");
                println!("toggle    - Toggles mpd playback");
                println!("discard   - removes curently playing song");
                println!("status    - Same as no arguments: Prints mpd status screen");
                println!("playlist  - outputs mpd playlist with index numbers");
                println!("repeat/random/single/consume - toggles mpd state");
                println!("update    - updates mpd database");
                println!("volume    - changes mpd volume");
                println!("add       - adds given files from mpd music directiory");
                println!("            seperate list of files with commas");
                println!("remove    - removes items at given indecies");
                println!("            seperate list of indecies with commas");
            },
            "toggle" => {
                writer.write(b"pause\n")?;
                writer.flush()?;

                // Needed for command to be recognised?
                reader.read_mpd_end(&mut str_buff)?;
            },
            "discard" => {
                // Query consume state
                writer.write(b"status\n")?;
                writer.flush()?;
                reader.read_mpd_end(&mut str_buff)?;
                // Preform regex to get current consume state
                let mut discard_command: &[u8] = &[];
                for line in str_buff.lines() {
                    if !line.starts_with("consume: ") {
                        continue;
                    }
                    if line.contains("1") {
                        discard_command = b"next\n";
                    } else {
                        // If not consuming add toggle before and after next command
                        discard_command =
                            b"command_list_begin\n\
                                consume 1\n\
                                next\n\
                                consume 0\n\
                                command_list_end\n\
                                ";
                    }
                    break;
                }
                writer.write(discard_command)?;
                writer.flush()?;
            },
            "status" => {
                // Info about mpd status
                writer.write(b"status\n")?;
                writer.flush()?;
                let _ = reader.read_mpd_end(&mut str_buff);
                let mut items: HashMap<String, String> = HashMap::new();

                // Parse return into key value pairs
                str_buff.lines().for_each(|line| {
                    let (key, value) = line.split_once(':').unzip();
                    if let Some(key_value) = key {
                        items.insert(
                            key_value.to_string(),
                            if let Some(value_value) = value {
                                value_value.trim().to_string()
                            } else {
                                "".to_string()
                            }
                        );
                    }
                });
                // Get info about current song
                writer.write(
                    format!("playlistid {}\n", items["songid"]).as_bytes()
                )?;
                writer.flush()?;
                let _ = reader.read_mpd_end(&mut str_buff);

                // Parse return into key value pairs again
                str_buff.lines().for_each(|line| {
                    let (key, value) = line.split_once(':').unzip();
                    if let Some(key_value) = key {
                        items.insert(
                            key_value.to_string(),
                            if let Some(value_value) = value {
                                value_value.trim().to_string()
                            } else {
                                "".to_string()
                            }
                        );
                    }
                });

                let format_time = |t: &str| {
                    let seconds = t
                        .split_once('.').unwrap_or_else(|| ("0", "0"))
                        .0.parse::<i32>().unwrap_or_else(|_| 0);
                    // More then 1 hour
                    if seconds > 60*60 {
                        return format!("{}:{:02}:{:02}",
                            seconds/3600,
                            (seconds/60)%60,
                            seconds%60,
                        );
                    }
                    // ensures last digit is at least 2 characters
                    return format!("{}:{:02}",
                        seconds/60,
                        seconds%60,
                    );
                };
                // Output status
                println!(
                    "{}\n\
                    [{}] #{}/{} {}/{}\n\
                    volume: {}%  repeat: {}  random: {}  single: {}  consume: {}",
                    items["file"],
                    if items["state"] == "pause" {
                        "paused"
                    } else {
                        "playing"
                    },
                    items["Pos"].parse::<i32>()? + 1,
                    items["playlistlength"],
                    format_time(items["elapsed"].as_str()),
                    format_time(items["duration"].as_str()),
                    items["volume"],
                    if items["repeat"] == "1" {
                        "on"
                    } else {
                        "off"
                    },
                    if items["random"] == "1" {
                        "on"
                    } else {
                        "off"
                    },
                    if items["single"] == "1" {
                        "on"
                    } else {
                        "off"
                    },
                    if items["consume"] == "1" {
                        "on"
                    } else {
                        "off"
                    },
                );
            },
            "playlist" => {
                writer.write(b"playlistinfo\n")?;
                writer.flush()?;
                reader.read_mpd_end(&mut str_buff)?;
                let mut index = 1;
                str_buff.lines().for_each(|line| {
                    if line.starts_with("file: ") {
                        if let Some (value) = line.split_once(": ") {
                            println!("{}: {}", index, value.1);
                            index += 1;
                        }
                    }
                });
            },
            "repeat" => {
                let mut state = 0;
                writer.write(b"status\n")?;
                writer.flush()?;
                reader.read_mpd_end(&mut str_buff)?;
                // Preform regex to get current consume state
                for line in str_buff.lines() {
                    if !line.starts_with("repeat: ") {
                        continue;
                    }
                    // Switches the value for the currently set
                    if line.contains("0") {
                        state = 1;
                    }
                    break;
                }
                writer.write(format!("repeat {}\n", state).as_bytes())?;
                writer.flush()?;
            },
            "random" => {
                let mut state = 0;
                writer.write(b"status\n")?;
                writer.flush()?;
                reader.read_mpd_end(&mut str_buff)?;
                // Preform regex to get current consume state
                for line in str_buff.lines() {
                    if !line.starts_with("random: ") {
                        continue;
                    }
                    // Switches the value for the currently set
                    if line.contains("0") {
                        state = 1;
                    }
                    break;
                }
                writer.write(format!("random {}\n", state).as_bytes())?;
                writer.flush()?;
            },
            "single" => {
                let mut state = 0;
                writer.write(b"status\n")?;
                writer.flush()?;
                reader.read_mpd_end(&mut str_buff)?;
                // Preform regex to get current consume state
                for line in str_buff.lines() {
                    if !line.starts_with("single: ") {
                        continue;
                    }
                    // Switches the value for the currently set
                    if line.contains("0") {
                        state = 1;
                    }
                    break;
                }
                writer.write(format!("single {}\n", state).as_bytes())?;
                writer.flush()?;
            },
            "consume" => {
                let mut state = 0;
                writer.write(b"status\n")?;
                writer.flush()?;
                reader.read_mpd_end(&mut str_buff)?;
                // Preform regex to get current consume state
                for line in str_buff.lines() {
                    if !line.starts_with("consume: ") {
                        continue;
                    }
                    // Switches the value for the currently set
                    if line.contains("0") {
                        state = 1;
                    }
                    break;
                }
                writer.write(format!("consume {}\n", state).as_bytes())?;
                writer.flush()?;
            },
            "update" => {
                writer.write(b"update\n")?;
                writer.flush()?;
                // Required at least once for mpd to register input
                reader.read_mpd_end(&mut str_buff)?;
            },
            "volume" => {
                arg_input = true;
                arg_function = volume;
            },
            "add" => {
                arg_input = true;
                arg_function = add;
            },
            "remove" => {
                arg_input = true;
                arg_function = remove;
            },
            _ => {
                println!("Invalid argument: {}", arg);
            },
        }
        str_buff = "".to_string();
    }
    return Ok(());
}

fn volume(
    ammount: String,
    reader: &mut BufReader<&TcpStream>,
    writer: &mut BufWriter<&TcpStream>,
) -> Result<(), Box<dyn std::error::Error>> {
    if !(ammount.starts_with("+") || ammount.starts_with("-")) {
        writer.write(format!("setvol {}\n", ammount).as_bytes())?;
        writer.flush()?;
        // At least one read call must be made for mpd to register writes
        let mut ignore = "".to_string();
        reader.read_mpd_end(&mut ignore)?;
    } else {
        let change = ammount.parse::<i32>()?;
        let mut current = "".to_string();
        writer.write(b"getvol\n")?;
        writer.flush()?;
        reader.read_mpd_end(&mut current)?;
        let val = current.lines().skip(1).next()
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unexpected data from mpd",
            ))?
            .split_once(": ").ok_or(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unexpected data from mpd"
            ))?
            .1.parse::<i32>()?;
        writer.write(format!("setvol {}\n", val + change).as_bytes())?;
        writer.flush()?;
    }
    Ok(())
}


fn add(
    files: String,
    reader: &mut BufReader<&TcpStream>,
    writer: &mut BufWriter<&TcpStream>,
) -> Result<(), Box<dyn std::error::Error>> {
    for file in files.split(',') {
        let mut str_buff = String::new();
        writer.write(
            format!("searchadd \"(file == \\\"{}\\\")\"\n", file).as_bytes()
        )?;
        writer.flush()?;
        // Required at least once for mpd to register input
        reader.read_mpd_end(&mut str_buff)?;
    }
    Ok(())
}

fn remove(
    targets: String,
    reader: &mut BufReader<&TcpStream>,
    writer: &mut BufWriter<&TcpStream>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut indecies = targets
        .split(',')
        .map(|x| x.parse::<usize>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap() - 1)
        .collect::<Vec<usize>>();
    indecies.sort();
    for (adjust, index) in indecies.into_iter().enumerate() {
        let mut str_buff = String::new();
        writer.write(
            format!("delete {}\n", index - adjust).as_bytes()
        )?;
        writer.flush()?;
        // Required at least once for mpd to register input
        reader.read_mpd_end(&mut str_buff)?;
    }
    Ok(())
}
