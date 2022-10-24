use heapless::String;

use super::ATRequest;
use crate::at_command::response::GenericOk;

pub struct Netopen;

impl ATRequest for Netopen {
    type Response = GenericOk;
    fn encode(&self) -> String<256> {
        "AT+NETOPEN\r".into()
    }
}
