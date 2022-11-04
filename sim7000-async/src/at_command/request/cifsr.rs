/*

use heapless::String;

use super::ATRequest;
//use crate::at_command::response::GenericOk;
use crate::at_command::response::{GenericOk, CifsrResult};
pub struct GetCifsrResult;

impl ATRequest for GetCifsrResult {
    type Response = GenericOk;
    fn encode(&self) -> String<256> {
        "AT+IPADDR\r".into()
    }
}
*/


use heapless::String;

use super::ATRequest;
use crate::at_command::response::{GenericOk, CifsrResult};

/// AT+CIFSREX
pub struct GetCifsrResult;

impl ATRequest for GetCifsrResult {
    type Response = (CifsrResult, GenericOk);
    fn encode(&self) -> String<256> {
        "AT+IPADDR\r".into()//niklas 2022
    }
}