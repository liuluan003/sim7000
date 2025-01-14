use core::fmt::Write;
use heapless::String;

use super::{AtRequest, GenericOk};

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum NetworkMode {
    Automatic = 2,
    Gsm = 13,
    Lte = 38,
    GsmAndLts = 51,
}

/// AT+CNMP=...
pub struct SetNetworkMode(pub NetworkMode);

impl AtRequest for SetNetworkMode {
    type Response = GenericOk;
    fn encode(&self) -> String<256> {
        let mut buf = String::new();
        write!(buf, "AT+CNMP={}\r", self.0 as u8).unwrap();
        buf
    }
}
