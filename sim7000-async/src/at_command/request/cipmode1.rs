use heapless::String;

use super::ATRequest;
use crate::at_command::response::GenericOk;

pub struct Cipmode1;

impl ATRequest for Cipmode1 {
    type Response = GenericOk;
    fn encode(&self) -> String<256> {
        "AT+CIPMODE=1\r".into()
    }
}
