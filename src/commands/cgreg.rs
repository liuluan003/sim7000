use crate::{Error, SerialReadTimeout, SerialWrite};

use super::{AtCommand, AtDecode, AtEncode, AtRead, AtWrite, Decoder, Encoder, RegistrationStatus};

pub struct Cgreg;

impl AtCommand for Cgreg {
    const COMMAND: &'static str = "AT+CGREG";
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GRegistrationMode {
    Disable = 0,
    EnableReg = 1,
    EnableRegLac = 2,
    EnableRegLacTime = 4,
}

#[derive(Copy, Clone)]
pub struct RegistrationResponse {
    pub mode: GRegistrationMode,
    pub stat: RegistrationStatus,
}

impl AtDecode for RegistrationResponse {
    fn decode<B: SerialReadTimeout>(
        decoder: &mut Decoder<B>,
        timeout_ms: u32,
    ) -> Result<Self, Error<B::SerialError>> {
        decoder.expect_str("+CGREG: ", timeout_ms)?;

        let mode = match decoder.decode_scalar(timeout_ms)? {
            0 => GRegistrationMode::Disable,
            1 => GRegistrationMode::EnableReg,
            2 => GRegistrationMode::EnableRegLac,
            4 => GRegistrationMode::EnableRegLacTime,
            _ => return Err(crate::Error::DecodingFailed),
        };

        decoder.expect_str(",", timeout_ms)?;
        let stat = match decoder.decode_scalar(timeout_ms)? {
            0 => RegistrationStatus::NotRegistered,
            1 => RegistrationStatus::RegisteredHome,
            2 => RegistrationStatus::Searching,
            3 => RegistrationStatus::RegistrationDenied,
            4 => RegistrationStatus::Unknown,
            5 => RegistrationStatus::RegisteredRoaming,
            _ => return Err(crate::Error::DecodingFailed),
        };

        decoder.end_line();
        decoder.expect_str("OK", timeout_ms)?;

        Ok(RegistrationResponse { mode, stat })
    }
}

impl AtEncode for GRegistrationMode {
    fn encode<B: SerialWrite>(
        &self,
        encoder: &mut Encoder<B>,
    ) -> Result<(), Error<B::SerialError>> {
        encoder.encode_scalar(*self as i32)
    }
}

impl AtRead for Cgreg {
    type Output = RegistrationResponse;
}

impl AtWrite<'_> for Cgreg {
    type Input = GRegistrationMode;
    type Output = ();
}
