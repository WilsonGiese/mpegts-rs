use super::{ PTSPacket };

#[derive(Debug)]
struct Stream<'a> {
    data: &'a[u8],
    position :usize,  // Current byte position in data
    bit_marker: u8,   // Current bit position in data
    bit_position: u8, // Position of marked bit (Basically tracking log2(bit_marker))
}

impl<'a> Stream<'a> {

    fn new(data: &[u8]) -> Stream {
        Stream {
            data: data,
            position: 0,
            bit_marker: 1,
            bit_position: 0,
        }
    }

    /// Pull a single byte from the stream (only allowed if bit position is alligned)
    fn pull_byte(&mut self) -> Result<u8, &'static str> {
        if self.bit_marker > 1 {
            Err("Requested byte, but bits have already been pulled from the current byte")
        } else if self.position >= self.data.len() {
            Err("No data remaining")
        } else {
            let v = self.data[self.position];
            self.position += 1;
            Ok(v)
        }
    }

    /// Pull a single bit from the stream
    fn pull_bit(&mut self) -> Result<bool, &'static str> {
        if self.position >= self.data.len() {
            Err("No data remaining")
        } else {
            let v = (self.data[self.position] & self.bit_marker) > 0;

            if self.bit_marker == (1 << 7) {
                self.position += 1;
                self.bit_marker = 1;
                self.bit_position = 0;
            } else {
                self.bit_marker <<= 1;
                self.bit_position += 1;
            }
            Ok(v)
        }
    }

    /// Pull n bits from the stream (from current byte position only)
    /// Cannot pull more than 8 bits
    fn pull_bits(&mut self, n: u8) -> Result<u8, &'static str> {
        if n == 8 {
            self.pull_byte()
        } else if self.bit_position + n > 8 {
            Err("Requested more bits than what remains in the current byte")
        } else {
            // Bit twiddling ahead! It's dangerous to go alone, take these notes.
            // Produce a mask to extract the desired bits from the current marked position
            // Example:
            // bit_marker = 00000100, bit_position = 3, byte = 01001110, n = 3
            //
            // First get mask for unwanted least significant bits:
            //      bit_marker - 1 = 00000011
            // Next get mask for unwanted most siginifcant bits:
            //      bit_marker << n = 00100000
            //      00100000 - 1 = 00011111
            //      !00011111 = 11100000
            // Next combine the two results to get the mask for unwanted bits
            //        00000011
            //      | 11100000
            //      = 11100011
            // Finally, obtain the mask
            //      !11100011 = 00011100
            // This mask can be used extract n bits from the current byte
            //      byte | 00011100 = 00011000
            //      00011000 >> bit_position = 00000011
            let mask = !((self.bit_marker - 1) | !(((self.bit_marker as u16) << n) - 1) as u8);
            let v = (self.data[self.position] & mask) >> self.bit_position;

            if self.bit_position + n == 8 {
                self.position += 1;
                self.bit_marker = 1;
                self.bit_position = 0;
            } else {
                self.bit_marker <<= n;
                self.bit_position += n;
            }
            Ok(v)
        }
    }

    /// Assumes BE at the moment, which is how MPEG-TS packs its bytes
    fn pull_bits_u16(&mut self, n: u8) -> Result<u16, &'static str> {
        if n > 16 {
            Err("Requested more than what exists in a u16")
        } else {
            let n1 = 8 - self.bit_position;
            let n2 = n - n1;
            Ok((try!(self.pull_bits(n1)) as u16) << n2 | try!(self.pull_bits(n2)) as u16)
        }
    }
}

impl PTSPacket {
    fn parse(data: &[u8]) -> Result<PTSPacket, &'static str> {
        let mut s = Stream::new(data);

        Ok(PTSPacket {
            sync_byte:                    try!(s.pull_byte()),
            transport_error_indicator:    try!(s.pull_bit()),
            payload_unit_start_indicator: try!(s.pull_bit()),
            transport_priority:           try!(s.pull_bit()),
            pid:                          try!(s.pull_bits_u16(13)),
            scrambling_control:           try!(s.pull_bits(2)),
            adaptation_field_control:     try!(s.pull_bits(2)),
            continuity_counter:           try!(s.pull_bits(4)),
        })
    }
}

#[test]
fn test_parse() {
    let data: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];
    let packet = PTSPacket::parse(&data[..]).unwrap();

    assert_eq!(packet.sync_byte, 0xFF);
    assert!(packet.transport_error_indicator);
    assert!(packet.payload_unit_start_indicator);
    assert!(packet.transport_priority);
    assert_eq!(packet.pid, 0b0001111111111111);
    assert_eq!(packet.scrambling_control, 0b00000011);
    assert_eq!(packet.adaptation_field_control, 0b00000011);
    assert_eq!(packet.continuity_counter, 0b00001111);
}

#[test]
fn test_pull_byte() {
    let data: [u8; 6] = [0x1, 0x1, 0x2, 0x3, 0x5, 0x8];
    let mut stream = Stream::new(&data[..]);

    assert_eq!(stream.pull_byte().unwrap(), 0x1);
    assert_eq!(stream.pull_byte().unwrap(), 0x1);
    assert_eq!(stream.pull_byte().unwrap(), 0x2);
    assert_eq!(stream.pull_byte().unwrap(), 0x3);
    assert_eq!(stream.pull_byte().unwrap(), 0x5);
    assert_eq!(stream.pull_byte().unwrap(), 0x8);
}

#[test]
fn test_pull_bit() {
    let data: [u8; 2] = [0b10010110, 0b10100101];
    let mut stream = Stream::new(&data[..]);

    assert_eq!(stream.pull_bit().unwrap(), false);
    assert_eq!(stream.pull_bit().unwrap(), true);
    assert_eq!(stream.pull_bit().unwrap(), true);
    assert_eq!(stream.pull_bit().unwrap(), false);
    assert_eq!(stream.pull_bit().unwrap(), true);
    assert_eq!(stream.pull_bit().unwrap(), false);
    assert_eq!(stream.pull_bit().unwrap(), false);
    assert_eq!(stream.pull_bit().unwrap(), true);
    assert_eq!(stream.pull_bit().unwrap(), true);
    assert_eq!(stream.pull_bit().unwrap(), false);
    assert_eq!(stream.pull_bit().unwrap(), true);
    assert_eq!(stream.pull_bit().unwrap(), false);
    assert_eq!(stream.pull_bit().unwrap(), false);
    assert_eq!(stream.pull_bit().unwrap(), true);
    assert_eq!(stream.pull_bit().unwrap(), false);
    assert_eq!(stream.pull_bit().unwrap(), true);
}

#[test]
fn test_pull_bits() {
    let data: [u8; 2] = [0b10101010, 0b10010011];
    let mut stream = Stream::new(&data[..]);

    assert_eq!(stream.pull_bits(2).unwrap(), 0b10);
    assert_eq!(stream.pull_bits(3).unwrap(), 0b010);
    assert_eq!(stream.pull_bits(3).unwrap(), 0b101);
    assert_eq!(stream.pull_bits(4).unwrap(), 0b0011);
    assert_eq!(stream.pull_bits(1).unwrap(), 0b1);
    assert_eq!(stream.pull_bits(3).unwrap(), 0b100);
}

#[test]
fn test_pull_bits_u16() {
    let data: [u8; 3] = [0b10010011, 0b10101010, 0b11110000];
    let mut stream = Stream::new(&data[..]);

    assert_eq!(stream.pull_bits_u16(9).unwrap(), 0b100100110);
    assert_eq!(stream.pull_bits_u16(12).unwrap(), 0b101010110000);
    assert_eq!(stream.pull_bits(3).unwrap(), 0b111);
}
