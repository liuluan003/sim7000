use heapless::String;

use super::ATRequest;
use crate::at_command::response::GenericOk;

pub struct Cgdcount;

impl ATRequest for Cgdcount {
    type Response = GenericOk;
    fn encode(&self) -> String<256> {
        "AT+CGDCONT=1,\"IP\",\"EVERYWHERE\"\r".into()
    }
}
