use core::fmt::Write;
use heapless::String;

use super::ATRequest;
use crate::at_command::response::GenericOk;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum RiPinMode {
    Off = 0,
    On = 1,
    OnTcpIp = 2,
}

/// AT+CFGRI=...
pub struct ConfigureRiPin(pub RiPinMode);

impl ATRequest for ConfigureRiPin {
    type Response = GenericOk;
    fn encode(&self) -> String<256> {
        let mut buf = String::new();
        write!(buf, "AT+CFGRI={}\r", self.0 as u8).unwrap();
        buf
    }
}