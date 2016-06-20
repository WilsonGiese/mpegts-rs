//! Transport Stream

use stream::{Stream};  

/// Partial Transport Stream Packet
#[derive(Eq,PartialEq,Debug,Clone,Default)]
pub struct PTSPacket {
    sync_byte: u8,
    transport_error: bool,
    payload_unit_start: bool,
    transport_priority: bool,
    pid: u16,
    scrambling_control: u8,
    continuity_counter: u8,
    // Optional
    adaptation_field: Option<Box<AdaptationField>>,
    payload: Option<Box<Vec<u8>>>
}

#[derive(Eq,PartialEq,Debug,Clone,Default)]
pub struct AdaptationField {
    field_length: u8,
    discontinuity: bool,
    random_access: bool,
    elementary_stream_priority: bool,
    // Optional
    pcr: u64,
    opcr: u64,
    splice_countdown: u8,
    transport_private_data_length: u8,
    transport_private_data: Option<Box<Vec<u8>>>,
    adaptation_extension: Option<Box<AdaptationFieldExtension>>,
    stuffing_bytes: Option<Box<Vec<u8>>>,
}

#[derive(Eq,PartialEq,Debug,Clone,Default)]
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

impl PTSPacket {
    fn parse(data: &[u8]) -> Result<PTSPacket, &'static str> {
        let mut s = Stream::new(data);

        let mut packet = PTSPacket::default();
        packet.sync_byte             = try!(s.pull_byte());
        packet.transport_error       = try!(s.pull_bit());
        packet.payload_unit_start    = try!(s.pull_bit());
        packet.transport_priority    = try!(s.pull_bit());
        packet.pid                   = try!(s.pull_bits_u16(13));
        packet.scrambling_control    = try!(s.pull_bits(2));
        let adaptation_field_flag    = try!(s.pull_bit());
        let payload_flag             = try!(s.pull_bit());
        packet.continuity_counter    = try!(s.pull_bits(4));

        if adaptation_field_flag {
            let mut adaptation_field = AdaptationField::default();
            adaptation_field.field_length               = try!(s.pull_byte());
            adaptation_field.discontinuity              = try!(s.pull_bit());
            adaptation_field.random_access              = try!(s.pull_bit());
            adaptation_field.elementary_stream_priority = try!(s.pull_bit());
            let pcr_flag                                = try!(s.pull_bit());
            let opcr_flag                               = try!(s.pull_bit());
            let splicing_point_flag                     = try!(s.pull_bit());
            let transport_private_data_flag             = try!(s.pull_bit());
            let adaptation_field_extension              = try!(s.pull_bit());

            if pcr_flag {
                // pcr = Read 48 bits
            }

            if opcr_flag {
                // opcr = Read 48 bits
            }

            if splicing_point_flag {
                // splice_countdown = Read byte
            }

            if transport_private_data_flag {
                // transport_private_data_length = Read byte
                // transport_private_data = Read transport_private_data_length bytes
            }

            packet.adaptation_field = Some(Box::new(adaptation_field));
        }

        Ok(packet)
    }
}

#[test]
fn test_parse() {
    let data: [u8; 8] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    let packet = PTSPacket::parse(&data[..]).unwrap();

    assert_eq!(packet.sync_byte, 0xFF);
    assert!(packet.transport_error);
    assert!(packet.payload_unit_start);
    assert!(packet.transport_priority);
    assert_eq!(packet.pid, 0b0001111111111111);
    assert_eq!(packet.scrambling_control, 0b00000011);
    assert_eq!(packet.continuity_counter, 0b00001111);
}
