use super::{AtCommand, AtExecute};

/// Cold start GPS.
pub struct Cgnscold;

impl AtCommand for Cgnscold {
    const COMMAND: &'static str = "AT+CGNSCOLD";
}

impl AtExecute for Cgnscold {
    type Output = ();
}
