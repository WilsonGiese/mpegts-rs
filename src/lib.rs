//! mpegts is a parser library for the
//! [MPEG Transport Stream](https://en.wikipedia.org/wiki/MPEG_transport_stream) format

#[macro_use]
mod parser;

/// Partial Transport Stream Packet
#[derive(Eq,PartialEq,Debug,Clone)]
pub struct PTSPacket {
    sync_byte: u8,
    transport_error: bool,
    payload_unit_start: bool,
    transport_priority: bool,
    pid: u16,
    scrambling_control: u8,
    adaptation_field_control: u8,
    continuity_counter: u8,
    // Optional
    adaptation_field: Option<Box<AdaptationField>>,
    payload: Option<Box<Vec<u8>>>
}

#[derive(Eq,PartialEq,Debug,Clone)]
pub struct AdaptationField {
    field_length: u8,
    discontinuity: bool,
    random_access: bool,
    elementary_stream_priority: bool,
    pcr_flag: bool,
    opcr_flag: bool,
    splicing_point_flag: bool,
    transport_private_data_flag: bool,
    adaptation_field_extension: bool,
    // Optional
    pcr: u64,
    opcr: u64,
    splice_countdown: u8,
    transport_private_data_length: u8,
    transport_private_data: Option<Box<Vec<u8>>>,
    adaptation_extension: Option<Box<AdaptationFieldExtension>>,
    stuffing_bytes: Option<Box<Vec<u8>>>,
}

#[derive(Eq,PartialEq,Debug,Clone)]
pub struct AdaptationFieldExtension {
    adaptation_extension_length: u8,
    legal_time_window_flag: bool,
    piecewise_rate_flag: bool,
    seamless_rate_flag: bool,
    // Optional LTW Flag Set
    legal_time_window_valid_flag: bool,
    legal_time_window_offset: u16,
    // Optional Piecewise Flag Set
    piecewise_rate: u32,
    // Optional Seamless Splice Flag Set
    splice_type: u8,
    dts_next_access_unit: u64,
}
