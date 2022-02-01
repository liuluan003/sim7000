use crate::Error;

use super::{AtCommand, AtEncode, AtWrite};

pub struct Cedrx;

impl AtCommand for Cedrx {
    const COMMAND: &'static str = "AT+CEDRX";
}

impl AtWrite<'_> for Cedrx {
    type Input = DetailedEdrxParams;
    type Output = ();
}

#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum DetailedEdrxBand {
    Gsm = 0,
    Lte = 1,
    NbIot = 2,
    CatM = 3,
}

impl AtEncode for DetailedEdrxBand {
    fn encode<B: crate::SerialWrite>(
        &self,
        encoder: &mut super::Encoder<B>,
    ) -> Result<(), Error<B::SerialError>> {
        encoder.encode_scalar(*self as i32)
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum DetailedEdrxMode {
    Disable = 0,
    Enable = 1,
}

impl AtEncode for DetailedEdrxMode {
    fn encode<B: crate::SerialWrite>(
        &self,
        encoder: &mut super::Encoder<B>,
    ) -> Result<(), Error<B::SerialError>> {
        encoder.encode_scalar(*self as i32)
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum DetailedEdrxInterval {
    S5_12 = 0,
    S10_24 = 1,
    S20_48 = 2,
    S40_96 = 3,
    S61_44 = 4,
    S81_92 = 5,
    S102_40 = 6,
    S122_88 = 7,
    S143_36 = 8,
    S163_84 = 9,
    S327_68 = 10,
    S655_36 = 11,
    S1310_72 = 12,
    S2621_44 = 13,
    S5242_88 = 14,
    S10485_76 = 15,
}

impl AtEncode for DetailedEdrxInterval {
    fn encode<B: crate::SerialWrite>(
        &self,
        encoder: &mut super::Encoder<B>,
    ) -> Result<(), Error<B::SerialError>> {
        encoder.encode_scalar(*self as i32)
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum PageTimeWindow {
    S1_28 = 0,
    S2_56 = 1,
    S3_84 = 2,
    S5_12 = 3,
    S6_40 = 4,
    S7_68 = 5,
    S8_96 = 6,
    S10_24 = 7,
    S11_52 = 8,
    S12_80 = 9,
    S14_08 = 10,
    S15_36 = 11,
    S16_64 = 12,
    S17_92 = 13,
    S19_20 = 14,
    S20_48 = 15,
}

impl AtEncode for PageTimeWindow {
    fn encode<B: crate::SerialWrite>(
        &self,
        encoder: &mut super::Encoder<B>,
    ) -> Result<(), Error<B::SerialError>> {
        encoder.encode_scalar(*self as i32)
    }
}

pub struct DetailedEdrxParams {
    pub mode: DetailedEdrxMode,
    pub band: DetailedEdrxBand,
    pub interval: DetailedEdrxInterval,
    pub page_time_window: PageTimeWindow,
}

impl AtEncode for DetailedEdrxParams {
    fn encode<B: crate::SerialWrite>(
        &self,
        encoder: &mut super::Encoder<B>,
    ) -> Result<(), Error<B::SerialError>> {
        self.band.encode(encoder)?;
        encoder.encode_str(",")?;
        self.mode.encode(encoder)?;
        encoder.encode_str(",")?;
        self.page_time_window.encode(encoder)?;
        encoder.encode_str(",")?;
        self.interval.encode(encoder)
    }
}
