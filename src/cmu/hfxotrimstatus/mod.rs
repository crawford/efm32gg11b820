#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HFXOTRIMSTATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct IBTRIMXOCORER {
    bits: u16,
}
impl IBTRIMXOCORER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IBTRIMXOCOREMONR {
    bits: u16,
}
impl IBTRIMXOCOREMONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VALIDR {
    bits: bool,
}
impl VALIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct MONVALIDR {
    bits: bool,
}
impl MONVALIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:10 - Value of IBTRIMXOCORE Found By Automatic HFXO Peak Detection Algorithm"]
    #[inline]
    pub fn ibtrimxocore(&self) -> IBTRIMXOCORER {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        IBTRIMXOCORER { bits }
    }
    #[doc = "Bits 16:26 - Value of IBTRIMXOCORE Found By Automatic HFXO Peak Detection Algorithm or Peak Monitoring Algorithm (completion of Either Algorithm Will Cause an Update of IBTRIMXOCOREMON)"]
    #[inline]
    pub fn ibtrimxocoremon(&self) -> IBTRIMXOCOREMONR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        IBTRIMXOCOREMONR { bits }
    }
    #[doc = "Bit 30 - Peak Detection Algorithm Found a Value for IBTRIMXOCORE"]
    #[inline]
    pub fn valid(&self) -> VALIDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VALIDR { bits }
    }
    #[doc = "Bit 31 - Peak Detection Algorithm or Peak Monitoring Algorithm Found a Value for IBTRIMXOCOREMON"]
    #[inline]
    pub fn monvalid(&self) -> MONVALIDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MONVALIDR { bits }
    }
}
