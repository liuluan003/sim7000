use crate::at_command::{ATParseErr, ATParseLine};

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SmsDone;

impl ATParseLine for SmsDone {
    fn from_line(line: &str) -> Result<Self, ATParseErr> {
        line.eq("SMS DONE")
            .then(|| SmsDone)
            .ok_or_else(|| "Missing 'SMS Ready'".into())
    }
}
