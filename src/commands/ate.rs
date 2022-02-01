use super::{AtCommand, AtExecute};

/// Turn echo off
pub struct Ate0;

impl AtCommand for Ate0 {
    const COMMAND: &'static str = "ATE0";
}

impl AtExecute for Ate0 {
    type Output = ();
}
