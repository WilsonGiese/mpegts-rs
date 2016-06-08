//! mpegts is a parser library for the
//! [MPEG Transport Stream](https://en.wikipedia.org/wiki/MPEG_transport_stream) format

#[macro_use]
mod parser;

/// Partial Transport Stream Packet
#[derive(Eq,PartialEq,Debug,Copy,Clone)]
pub struct PTSPacket {
    sync_byte: u8,
    transport_error_indicator: bool,
    payload_unit_start_indicator: bool,
}
