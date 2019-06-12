extern crate sacn;
extern crate get_if_addrs;

use sacn::DmxSource;
use std::io::{self, Write};
use get_if_addrs::get_if_addrs;

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

/// Enumerate the network interfaces on the system and prompt the user
/// for their selection.
fn get_interface_to_use() -> Result<String, GetInterfaceError> {
  // Find the interfaces
  let if_addrs = get_if_addrs()?;
  if if_addrs.is_empty() {
    return Err(io::Error::new(io::ErrorKind::Other, "No network interfaces found"));
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
  match line.parse::<usize>() {
    Ok(index) => Ok(if_addrs[index].ip().to_string()),
    Err(e) => Err(e)
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

  let dmx_source = DmxSource::new("Controller").unwrap();

  match dmx_source.send(1, &[0, 1, 2]) {
    Ok(_) => {
      println!("DMX transmission started.")
    }
    Err(_) => {
      println!("Error sending DMX to universe 1!");
    }
  }

  println!("Press any key to exit...");
  let mut line = String::new();
  match io::stdin().read_line(&mut line) {
    _ => ()
  }

  // terminate the stream for a specific universe
  match dmx_source.terminate_stream(1) {
    _ => ()
  }
}