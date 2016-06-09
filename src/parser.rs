use super::{ PTSPacket };

struct Stream<'a> {
    data: &'a[u8],
    position :usize,
    bit_marker: u8,
}

impl<'a> Stream<'a> {

    fn new(data: &[u8]) -> Stream {
        Stream {
            data: data,
            position: 0,
            bit_marker: 1,
        }
    }

    fn pull_byte(&mut self) -> Result<u8, &'static str> {
        if self.bit_marker == 1 && self.position < self.data.len() {
            let v = self.data[self.position];
            self.position += 1;
            Ok(v)
        } else if self.bit_marker > 1 && self.position < self.data.len() - 1 {
            // Get ms bits
            let mut v = self.data[self.position] & !(self.bit_marker - 1);
            self.position += 1;
            // Get ls bits
            v |= self.data[self.position] & (self.bit_marker - 1);
            Ok(v)
        } else {
            Err("Requested byte, but not enough data remains")
        }
    }

    fn pull_bit(&mut self) -> Result<bool, &'static str> {
        if self.position < self.data.len() {
            let byte = self.data[self.position];
            let v = (byte & self.bit_marker) > 0;

            println!("{:b}", self.bit_marker);

            if self.bit_marker == (1 << 7) {
                self.bit_marker = 1;
                self.position += 1;
            } else {
                self.bit_marker <<= 1;
            }
            Ok(v)
        } else {
            Err("Requested bit, but not enough data remains")
        }
    }
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
fn test_pull_byte_not_alligned() {
    let data: [u8; 2] = [0b10000000, 0b11111101];
    let mut stream = Stream::new(&data[..]);

    assert_eq!(stream.pull_bit().unwrap(), false);
    assert_eq!(stream.pull_byte().unwrap(), 0b10000001);
    assert_eq!(stream.pull_bit().unwrap(), false);
    assert_eq!(stream.pull_bit().unwrap(), true);
}
