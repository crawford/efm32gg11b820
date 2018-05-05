#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DSTS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SUSPSTSR {
    bits: bool,
}
impl SUSPSTSR {
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
pub struct ENUMSPDR {
    bits: u8,
}
impl ENUMSPDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ERRTICERRR {
    bits: bool,
}
impl ERRTICERRR {
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
pub struct SOFFNR {
    bits: u16,
}
impl SOFFNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DEVLNSTSR {
    bits: u8,
}
impl DEVLNSTSR {
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
    #[doc = "Bit 0 - Suspend Status"]
    #[inline]
    pub fn suspsts(&self) -> SUSPSTSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SUSPSTSR { bits }
    }
    #[doc = "Bits 1:2 - Enumerated Speed"]
    #[inline]
    pub fn enumspd(&self) -> ENUMSPDR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ENUMSPDR { bits }
    }
    #[doc = "Bit 3 - Erratic Error"]
    #[inline]
    pub fn errticerr(&self) -> ERRTICERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ERRTICERRR { bits }
    }
    #[doc = "Bits 8:21 - Frame or Microframe Number of the Received SOF"]
    #[inline]
    pub fn soffn(&self) -> SOFFNR {
        let bits = {
            const MASK: u16 = 16383;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SOFFNR { bits }
    }
    #[doc = "Bits 22:23 - Device Line Status"]
    #[inline]
    pub fn devlnsts(&self) -> DEVLNSTSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DEVLNSTSR { bits }
    }
}
