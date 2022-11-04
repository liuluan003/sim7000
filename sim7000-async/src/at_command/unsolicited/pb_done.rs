
use crate::at_command::{ATParseErr, ATParseLine};

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PbDone;

impl ATParseLine for PbDone {
    fn from_line(line: &str) -> Result<Self, ATParseErr> {
        line.eq("PB DONE")
            .then(|| PbDone)
            .ok_or_else(|| "Missing 'SMS Ready'".into())
    }
}
