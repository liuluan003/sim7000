use crate::at_command::{stub_parser_prefix, ATParseErr, ATParseLine};

// stub type
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Cipopen;

impl ATParseLine for Cipopen {
    fn from_line(line: &str) -> Result<Self, ATParseErr> {
        stub_parser_prefix(line, "+CIPOPEN: ", Cipopen)
    }
}
