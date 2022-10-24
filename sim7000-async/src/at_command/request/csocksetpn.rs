use heapless::String;

use super::ATRequest;
use crate::at_command::response::GenericOk;

pub struct Csocksetpn1;

impl ATRequest for Csocksetpn1 {
    type Response = GenericOk;
    fn encode(&self) -> String<256> {
        "AT+CSOCKSETPN=1\r".into()
    }
}
