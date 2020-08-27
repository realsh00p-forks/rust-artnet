use crate::r#async::FromRaw;

const VERSION_LOCATION: usize = 10;
const TIMECODE_LOCATION: usize = 14;

fn validate_version(data: &[u8]) -> Option<()> {
	let offset_data = &data[VERSION_LOCATION..];
	if offset_data[0] == 0 && offset_data[1] == 14 {
		Some(())
	} else {
		None
	}
}

#[derive(Clone, Copy)]
pub struct OpTimeCode {
    frames: u8,
    seconds: u8,
    minutes: u8,
    hours: u8,
    typee: u8,
}

impl FromRaw<OpTimeCode> for OpTimeCode {
	fn from_raw(raw: &[u8]) -> Option<Self> {
		validate_version(raw)?;

        let frames = raw[TIMECODE_LOCATION + 0];
        let seconds = raw[TIMECODE_LOCATION + 1];
        let minutes = raw[TIMECODE_LOCATION + 2];
        let hours = raw[TIMECODE_LOCATION + 3];
        let typee = raw[TIMECODE_LOCATION + 4];

		Some(OpTimeCode { frames, seconds, minutes, hours, typee })
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const PACKET: [u8; 19] = [
        65, 114, 116, 45, 78, 101, 116, 0, // art-net
        0x00, 0x97,                        // op-code lo, hi
        0, 14,                             // protocol. ver lo, hi
        0, 0,                              // filler 1, 2
        1,2,3,4,5                          // frames, seconds, minutes, hours, type
    ];

	#[test]
	fn test_valid_version() {
		assert_eq!(validate_version(&PACKET), Some(()));
	}

	#[test]
	fn test_from_raw() {
		let packet = OpTimeCode::from_raw(&PACKET).expect("Unable to parse packet");
        assert_eq!(packet.frames, 1);
        assert_eq!(packet.seconds, 2);
        assert_eq!(packet.minutes, 3);
        assert_eq!(packet.hours, 4);
        assert_eq!(packet.typee, 5);
	}
}
