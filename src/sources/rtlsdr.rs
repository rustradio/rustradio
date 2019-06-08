extern crate rtlsdr_mt;

use rtlsdr_mt::{Controller, Reader};

pub struct RtlSdrSink {
    pub ctl: Controller,
    pub reader: Reader
}

impl RtlSdrSink {
    pub fn new(freq: u32) -> Self {
        let (mut ctl, reader) = rtlsdr_mt::open(0).unwrap();

        ctl.enable_agc().unwrap();
        ctl.set_center_freq(freq);

        RtlSdrSink{
            ctl,
            reader
        }
    }
}
