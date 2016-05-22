use nom::{IResult};
use super::{PTSPacket};

named!(take_byte, take!(1));

named!(parse<&[u8],PTSPacket>,
  chain!(
    sync_byte: take_byte ,
    || { PTSPacket { sync_byte: sync_byte[0] } }
  )
);

#[test]
fn test_parse() {
    assert_eq!(parse(b"ab"), IResult::Done(&b"b"[..], PTSPacket { sync_byte: b'a' }));
}
