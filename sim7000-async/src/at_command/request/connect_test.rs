use heapless::String;

use super::ATRequest;
use crate::at_command::response::GenericOk;


pub struct Connect_test;

impl ATRequest for Connect_test {
    type Response = GenericOk;
    fn encode(&self) -> String<256> {
        "AT+CIPOPEN=0,\"TCP\",\"46.39.103.234\",20000\r".into()
    }
}
