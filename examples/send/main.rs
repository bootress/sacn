extern crate get_if_addrs;
extern crate sacn;

use get_if_addrs::get_if_addrs;
use sacn::DmxSource;
use std::io::{self, Write};
use std::sync::mpsc;
use std::{error, fmt, num, thread, time};

#[derive(Debug)]
enum GetInterfaceError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for GetInterfaceError {
    fn from(error: io::Error) -> Self {
        GetInterfaceError::IoError(error)
    }
}

impl From<num::ParseIntError> for GetInterfaceError {
    fn from(error: num::ParseIntError) -> Self {
        GetInterfaceError::ParseError(error)
    }
}

impl fmt::Display for GetInterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInterfaceError::IoError(ref e) => e.fmt(f),
            GetInterfaceError::ParseError(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for GetInterfaceError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            GetInterfaceError::IoError(ref e) => Some(e),
            GetInterfaceError::ParseError(ref e) => Some(e),
        }
    }
}

/// Enumerate the network interfaces on the system and prompt the user for
/// their selection.
fn get_interface_to_use() -> Result<String, GetInterfaceError> {
    // Find the interfaces
    let if_addrs = get_if_addrs()?;
    if if_addrs.is_empty() {
        return Err(GetInterfaceError::from(io::Error::new(
            io::ErrorKind::Other,
            "No network interfaces found",
        )));
    }

    // Print and prompt for selection
    println!("Found these network interfaces:");
    for (i, if_addr) in if_addrs.iter().enumerate() {
        println!("{}: {}", i, if_addr.ip().to_string());
    }
    print!("Select an interface: ");
    io::stdout().flush()?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    // Parse the result
    match line.trim().parse::<usize>() {
        Ok(index) => Ok(if_addrs[index].ip().to_string()),
        Err(e) => Err(GetInterfaceError::from(e)),
    }
}

fn main() {
    let int_ip = match get_interface_to_use() {
        Ok(ip) => ip,
        Err(e) => {
            println!("Error getting network interface: {}", e);
            std::process::exit(1);
        }
    };

    let dmx_source = DmxSource::with_ip("Controller", &int_ip).unwrap();

    let (tx, rx) = mpsc::channel();

    let dmx_thread = thread::spawn(move || {
        loop {
            match dmx_source.send(1, &[0, 1, 2]) {
                Ok(_) => {}
                Err(_) => {
                    println!("Send error... send thread terminating.");
                    break;
                }
            }

            thread::sleep(time::Duration::from_millis(22));

            match rx.try_recv() {
                Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
                    println!("Send thread terminating.");
                    break;
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }

        // terminate the stream for a specific universe
        match dmx_source.terminate_stream(1) {
            _ => (),
        }
    });

    println!("Press any key to exit...");
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        _ => (),
    }

    let _ = tx.send(());

    dmx_thread.join().unwrap();
}
