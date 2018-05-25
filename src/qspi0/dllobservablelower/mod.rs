#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DLLOBSERVABLELOWER {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct DLLOBSERVABLELOWERDLLLOCKR {
    bits: bool,
}
impl DLLOBSERVABLELOWERDLLLOCKR {
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
pub struct DLLOBSERVABLELOWERLOCKMODER {
    bits: u8,
}
impl DLLOBSERVABLELOWERLOCKMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLLOBSERVABLELOWERUNLOCKCOUNTERR {
    bits: u8,
}
impl DLLOBSERVABLELOWERUNLOCKCOUNTERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLLOBSERVABLELOWERLOCKVALUER {
    bits: u8,
}
impl DLLOBSERVABLELOWERLOCKVALUER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLLOBSERVABLELOWERLOOPBACKLOCKR {
    bits: bool,
}
impl DLLOBSERVABLELOWERLOOPBACKLOCKR {
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
pub struct DLLOBSERVABLELOWERDLLLOCKDECR {
    bits: u8,
}
impl DLLOBSERVABLELOWERDLLLOCKDECR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLLOBSERVABLELOWERDLLLOCKINCR {
    bits: u8,
}
impl DLLOBSERVABLELOWERDLLLOCKINCR {
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
    #[doc = "Bit 0 - Indicates Status of DLL"]
    #[inline]
    pub fn dllobservablelowerdlllock(&self) -> DLLOBSERVABLELOWERDLLLOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DLLOBSERVABLELOWERDLLLOCKR { bits }
    }
    #[doc = "Bits 1:2 - DLL Oberservable Lock Mode"]
    #[inline]
    pub fn dllobservablelowerlockmode(&self) -> DLLOBSERVABLELOWERLOCKMODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLLOBSERVABLELOWERLOCKMODER { bits }
    }
    #[doc = "Bits 3:7 - DLL Observable Lower Unlock Counter"]
    #[inline]
    pub fn dllobservablelowerunlockcounter(&self) -> DLLOBSERVABLELOWERUNLOCKCOUNTERR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLLOBSERVABLELOWERUNLOCKCOUNTERR { bits }
    }
    #[doc = "Bits 8:14 - DLL Observable Lower Lock Value"]
    #[inline]
    pub fn dllobservablelowerlockvalue(&self) -> DLLOBSERVABLELOWERLOCKVALUER {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLLOBSERVABLELOWERLOCKVALUER { bits }
    }
    #[doc = "Bit 15 - DLL Observable Lower Loopback Lock"]
    #[inline]
    pub fn dllobservablelowerloopbacklock(&self) -> DLLOBSERVABLELOWERLOOPBACKLOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DLLOBSERVABLELOWERLOOPBACKLOCKR { bits }
    }
    #[doc = "Bits 16:23 - DLL Observable Lower Lock Decrement"]
    #[inline]
    pub fn dllobservablelowerdlllockdec(&self) -> DLLOBSERVABLELOWERDLLLOCKDECR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLLOBSERVABLELOWERDLLLOCKDECR { bits }
    }
    #[doc = "Bits 24:31 - DLL Observable Lower Lock Increment"]
    #[inline]
    pub fn dllobservablelowerdlllockinc(&self) -> DLLOBSERVABLELOWERDLLLOCKINCR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLLOBSERVABLELOWERDLLLOCKINCR { bits }
    }
}
