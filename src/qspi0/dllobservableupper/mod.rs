#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DLLOBSERVABLEUPPER {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct DLLOBSERVABLEUPPERRXDECODEROUTPUTR {
    bits: u8,
}
impl DLLOBSERVABLEUPPERRXDECODEROUTPUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLLOBSERVABLEUPPERTXDECODEROUTPUTR {
    bits: u8,
}
impl DLLOBSERVABLEUPPERTXDECODEROUTPUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - DLL Observable Upper RX Decoder"]
    #[inline]
    pub fn dllobservableupperrxdecoderoutput(&self) -> DLLOBSERVABLEUPPERRXDECODEROUTPUTR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLLOBSERVABLEUPPERRXDECODEROUTPUTR { bits }
    }
    #[doc = "Bits 16:22 - Dll Observable Upper TX Decoder"]
    #[inline]
    pub fn dllobservableuppertxdecoderoutput(&self) -> DLLOBSERVABLEUPPERTXDECODEROUTPUTR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLLOBSERVABLEUPPERTXDECODEROUTPUTR { bits }
    }
}
