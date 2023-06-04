use crate::r#async::FromRaw;

use self::op_poll::OpPoll;
use self::op_poll:: OpPollReply;
use self::op_timecode::OpTimeCode;

use std::fmt::Debug;

pub(crate) mod header;
pub(crate) mod op_poll;
pub(crate) mod op_timecode;

pub union Payload {
    unset: (),
    poll: OpPoll,
    poll_reply: OpPollReply,
    time_code: OpTimeCode
}
pub struct Packet {
	pub header: header::Header,
    pub payload: Payload
}

impl Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            match self.header.opcode {
                header::Opcode::OpPoll => write!(f, "Poll {:?}", self.payload.poll),
                header::Opcode::OpPollReply => write!(f, "PollReply {:?}", self.payload.poll_reply),
                header::Opcode::OpTimeCode => write!(f, "TimeCode {:?}", self.payload.time_code),
                header::Opcode::Unknown => todo!(),
            }
        }
    }
}

impl FromRaw<Packet> for Packet {
	fn from_raw(raw: &[u8]) -> Option<Packet> {
		use header::Opcode;

		let header = header::Header::from_raw(raw)?;
        let mut payload = Payload{unset: ()};
		match header.opcode {
			Opcode::OpPoll => payload.poll = op_poll::OpPoll::from_raw(raw).unwrap(),
			Opcode::OpPollReply => payload.poll_reply = op_poll::OpPollReply{},
            Opcode::OpTimeCode => payload.time_code = op_timecode::OpTimeCode::from_raw(raw).unwrap(),
            Opcode::Unknown => (),
		};

		Some(Packet{
			header: header,
            payload: payload
		})
	}
}

fn read_little_endian(data: &[u8]) -> u16 {
	use byteorder::{LittleEndian, ReadBytesExt};
	use std::io::Cursor;
	let mut rdr = Cursor::new(data);

	rdr.read_u16::<LittleEndian>().unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	const PACKET: [u8; 14] = [65, 114, 116, 45, 78, 101, 116, 0, 0, 32, 0, 14, 0, 0];

	#[test]
	fn test_op_code_detection() {
		let remaining = &PACKET[8..];
		let op_code = read_little_endian(remaining);
		assert_eq!(op_code, 0x2000);
	}

	#[test]
	fn test_from_raw() {
		let packet = Packet::from_raw(&PACKET).expect("Unable to parse packet");
	}
}
