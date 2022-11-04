/*use futures_util::future::OrElse;

use crate::at_command::{ATParseErr, ATParseLine};

use super::{ATResponse, ResponseCode};

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CifsrResult {
    //pub str ip_address,
    pub ip_address: u8,
    /* 
    pub country: u8,
    pub issuer: u8,
    pub account: u64,
    */
    // pub checksum: u8,
}

impl ATParseLine for CifsrResult {
    fn from_line(line: &str) -> Result<Self, ATParseErr> {
        //  "89 88 28 0666001104843 8"
        //  "89 01 26 0862291477114 f"
        //+CCID: 89883030000002746920
        let line = line.strip_prefix("+IPADDR: ").ok_or("Invalid prefix")?;
        if line.len() <= 6 {
            return Err("Invalid IP address".into());
        }
        else
        {
            defmt::info!("catch{}",line);

        }

        //let ip_address = line;
        let ip_address= line[0..line.len()].parse()?;

        defmt::info!("IP{}",ip_address);
        defmt::info!("no IP");
        Ok(CifsrResult {
            ip_address
            /*
            country,
            issuer,
            account,
            */
        })
    }
}

impl ATResponse for CifsrResult {
    fn from_generic(code: ResponseCode) -> Result<Self, ResponseCode> {
        match code {
            ResponseCode::CifsrResult(cifsr) => Ok(cifsr),
            _ => Err(code),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_cifsr() {
        let valid_cifsrs = ["109.333.320.12", "109.333.320.12"];

        for cifsr in valid_iccds {
            assert!(Iccid::from_line(cifsr).is_ok());
        }
    }
}
*/



use crate::{
    at_command::{ATParseErr, ATParseLine},
    util::collect_array,
};

use super::{ATResponse, ResponseCode};

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CifsrResult {
    pub addr: [u8; 4],
}

impl ATParseLine for CifsrResult {
    fn from_line(line: &str) -> Result<Self, ATParseErr> {
        let addr = line
            .strip_prefix("+IPADDR: ")
            .ok_or("Missing '+IPADDR: '")?;
        let addr = collect_array(addr.splitn(4, '.').filter_map(|seg| seg.parse().ok()))
            .ok_or("Failed to parse IP segment")?;

        Ok(CifsrResult { addr })
    }
}

impl ATResponse for CifsrResult {
    fn from_generic(code: ResponseCode) -> Result<Self, ResponseCode> {
        match code {
            ResponseCode::CifsrResult(ip_ext) => Ok(ip_ext),
            _ => Err(code),
        }
    }
}