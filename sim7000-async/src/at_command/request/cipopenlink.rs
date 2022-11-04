use core::fmt::Write;
use heapless::String;

use super::ATRequest;
use crate::at_command::response::GenericOk;

pub enum CipopenlinkMode {
    Tcp,
    Udp,
}

/// AT+CIPSTART=...
pub struct Cipopenlink {
    /// Which connection slot to use (Multi-IP mode)
    pub number: usize,

    /// TCP or UDP
    pub mode: CipopenlinkMode,

    /// IP or domain name
    pub destination: String<100>,

    pub port: u16,
}

impl ATRequest for Cipopenlink {
    type Response = GenericOk; // TODO: should have its own type
    fn encode(&self) -> String<256> {
        let mode = match self.mode {
            CipopenlinkMode::Tcp => "TCP",
            CipopenlinkMode::Udp => "UDP",
        };

        let mut buf = String::new();
        write!(
            buf,
            "AT+CIPOPEN={},{:?},{:?},{}\r", //niklas 20221024
            self.number, mode, self.destination, self.port
        )
        .unwrap();
        buf
    }
}
