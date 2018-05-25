#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::GRXSTSP {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct CHNUMR {
    bits: u8,
}
impl CHNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BCNTR {
    bits: u16,
}
impl BCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DPIDR {
    bits: u8,
}
impl DPIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PKTSTSR {
    bits: u8,
}
impl PKTSTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FNR {
    bits: u8,
}
impl FNR {
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
    #[doc = "Bits 0:3 - Channel Number"]
    #[inline]
    pub fn chnum(&self) -> CHNUMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CHNUMR { bits }
    }
    #[doc = "Bits 4:14 - Byte Count"]
    #[inline]
    pub fn bcnt(&self) -> BCNTR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        BCNTR { bits }
    }
    #[doc = "Bits 15:16 - Data PID"]
    #[inline]
    pub fn dpid(&self) -> DPIDR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DPIDR { bits }
    }
    #[doc = "Bits 17:20 - Packet Status"]
    #[inline]
    pub fn pktsts(&self) -> PKTSTSR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PKTSTSR { bits }
    }
    #[doc = "Bits 21:24 - Frame Number"]
    #[inline]
    pub fn fn_(&self) -> FNR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FNR { bits }
    }
}
