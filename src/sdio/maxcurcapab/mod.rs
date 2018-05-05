#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MAXCURCAPAB {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct MAXCUR3P3VALR {
    bits: u8,
}
impl MAXCUR3P3VALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAXCUR3P0VALR {
    bits: u8,
}
impl MAXCUR3P0VALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAXCUR1P8VALR {
    bits: u8,
}
impl MAXCUR1P8VALR {
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
    #[doc = "Bits 0:7 - Maximum Current for 3.3V"]
    #[inline]
    pub fn maxcur3p3val(&self) -> MAXCUR3P3VALR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAXCUR3P3VALR { bits }
    }
    #[doc = "Bits 8:15 - Maximum Current for 3.0V"]
    #[inline]
    pub fn maxcur3p0val(&self) -> MAXCUR3P0VALR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAXCUR3P0VALR { bits }
    }
    #[doc = "Bits 16:23 - Maximum Current for 1.8V"]
    #[inline]
    pub fn maxcur1p8val(&self) -> MAXCUR1P8VALR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAXCUR1P8VALR { bits }
    }
}
