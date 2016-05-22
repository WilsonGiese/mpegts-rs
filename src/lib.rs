//! mpegts is a parser library for the
//! [MPEG Transport Stream](https://en.wikipedia.org/wiki/MPEG_transport_stream) format

/// Partial Transport Stream Packet
#[derive(Eq,PartialEq,Debug,Copy,Clone)]
pub struct PTSPacket {
    sync_byte: u8,
}
