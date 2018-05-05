#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MODULEID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CONFR {
    bits: u8,
}
impl CONFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MODULEIDR {
    bits: u16,
}
impl MODULEIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FIXPATCHR {
    bits: u8,
}
impl FIXPATCHR {
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
    #[doc = "Bits 0:1 - Configuration ID Number"]
    #[inline]
    pub fn conf(&self) -> CONFR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CONFR { bits }
    }
    #[doc = "Bits 8:23 - Module/Revision ID Number"]
    #[inline]
    pub fn moduleid(&self) -> MODULEIDR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MODULEIDR { bits }
    }
    #[doc = "Bits 24:31 - Fix/patch Number"]
    #[inline]
    pub fn fixpatch(&self) -> FIXPATCHR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FIXPATCHR { bits }
    }
}
