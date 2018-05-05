#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TSUPTPTXNSEC {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TIMERR {
    bits: u32,
}
impl TIMERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:29 - PTP Event Frame Transmitted Nanoseconds"]
    #[inline]
    pub fn timer(&self) -> TIMERR {
        let bits = {
            const MASK: u32 = 1073741823;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        TIMERR { bits }
    }
}
