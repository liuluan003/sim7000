use heapless::String;

use super::ATRequest;
use crate::at_command::response::GenericOk;

pub struct Cifsr;

impl ATRequest for Cifsr {
    type Response = GenericOk;
    fn encode(&self) -> String<256> {
        "AT+IPADDR\r".into()

    }
}
