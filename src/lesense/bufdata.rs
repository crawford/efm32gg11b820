#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::BUFDATA {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct BUFDATAR {
    bits: u16,
}
impl BUFDATAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BUFDATASRCR {
    bits: u8,
}
impl BUFDATASRCR {
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
    #[doc = "Bits 0:15 - Result Data"]
    #[inline]
    pub fn bufdata(&self) -> BUFDATAR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        BUFDATAR { bits }
    }
    #[doc = "Bits 16:19 - Result Data Source"]
    #[inline]
    pub fn bufdatasrc(&self) -> BUFDATASRCR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BUFDATASRCR { bits }
    }
}
