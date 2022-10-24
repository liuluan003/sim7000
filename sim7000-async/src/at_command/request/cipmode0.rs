use heapless::String;

use super::ATRequest;
use crate::at_command::response::GenericOk;

pub struct Cipmode0;

impl ATRequest for Cipmode0 {
    type Response = GenericOk;
    fn encode(&self) -> String<256> {
        "AT+CIPMODE=0\r".into()
    }
}
