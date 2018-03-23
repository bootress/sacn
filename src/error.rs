// Copyright 2018 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Errors used through the sacn crate.

use core::str::Utf8Error;
use core::fmt;

#[cfg(feature = "std")]
use std::error::Error;

use uuid::ParseError as UUidParseError;

/// Errors for parsing of sACN network packets.
#[derive(Debug)]
pub enum ParseError<'a> {
    /// Error parsing the received UUID.
    Uuid(UUidParseError),

    /// Error parsing received UTF8 string.
    Utf8(Utf8Error),

    /// Received PDU flags are invalid.
    PduInvalidFlags(u8),

    /// Received PDU length is invalid.
    PduInvalidLength(usize),

    /// Received PDU vector is invalid.
    PduInvalidVector(u32),

    /// Other invalid data received.
    InvalidData(&'a str),

    /// Buffer does not contain enough data.
    NotEnoughData,
}

impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::Uuid(ref err) => write!(f, "UUID parsing error: {}", err),
            ParseError::Utf8(ref err) => write!(f, "UTF8 error: {}", err),
            ParseError::PduInvalidFlags(flags) => write!(f, "Flags {:#b} are invalid", flags),
            ParseError::PduInvalidLength(len) => write!(f, "Length {} is invalid", len),
            ParseError::PduInvalidVector(vec) => write!(f, "Vector {:#x} not supported", vec),
            ParseError::InvalidData(ref msg) => write!(f, "Invalid data: {}", msg),
            ParseError::NotEnoughData => write!(f, "Not enough data supplied"),
        }
    }
}

#[cfg(feature = "std")]
impl<'a> Error for ParseError<'a> {
    fn description(&self) -> &str {
        match *self {
            ParseError::Uuid(ref err) => err.description(),
            ParseError::Utf8(ref err) => err.description(),
            ParseError::PduInvalidFlags(_) => "PDU invalid flags",
            ParseError::PduInvalidLength(_) => "PDU invalid length",
            ParseError::PduInvalidVector(_) => "PDU vector not supported",
            ParseError::InvalidData(ref msg) => msg,
            ParseError::NotEnoughData => "Not enough data supplied",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ParseError::Uuid(ref err) => Some(err),
            ParseError::Utf8(ref err) => Some(err),
            _ => None,
        }
    }
}

impl<'a> From<UUidParseError> for ParseError<'a> {
    fn from(err: UUidParseError) -> ParseError<'a> {
        ParseError::Uuid(err)
    }
}

impl<'a> From<Utf8Error> for ParseError<'a> {
    fn from(err: Utf8Error) -> ParseError<'a> {
        ParseError::Utf8(err)
    }
}

/// Errors for packing of sACN network packets.
#[derive(Debug)]
pub enum PackError<'a> {
    /// Packet contains invalid data.
    InvalidData(&'a str),

    /// Supplied buffer is not large enough.
    BufferNotLargeEnough,
}

impl<'a> fmt::Display for PackError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PackError::InvalidData(ref msg) => write!(f, "Invalid data: {}", msg),
            PackError::BufferNotLargeEnough => write!(f, "Supplied buffer is not large enough"),
        }
    }
}

#[cfg(feature = "std")]
impl<'a> Error for PackError<'a> {
    fn description(&self) -> &str {
        match *self {
            PackError::InvalidData(ref msg) => msg,
            PackError::BufferNotLargeEnough => "Supplied buffer is not large enough",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            _ => None,
        }
    }
}