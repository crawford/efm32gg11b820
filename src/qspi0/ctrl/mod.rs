#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `TXDLYMUXSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDLYMUXSELR {
    #[doc = "No delay"]
    NONE,
    #[doc = "Medium delay"]
    MEDIUM,
    #[doc = "Large delay"]
    LARGE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TXDLYMUXSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXDLYMUXSELR::NONE => 0,
            TXDLYMUXSELR::MEDIUM => 2,
            TXDLYMUXSELR::LARGE => 3,
            TXDLYMUXSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXDLYMUXSELR {
        match value {
            0 => TXDLYMUXSELR::NONE,
            2 => TXDLYMUXSELR::MEDIUM,
            3 => TXDLYMUXSELR::LARGE,
            i => TXDLYMUXSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == TXDLYMUXSELR::NONE
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline]
    pub fn is_medium(&self) -> bool {
        *self == TXDLYMUXSELR::MEDIUM
    }
    #[doc = "Checks if the value of the field is `LARGE`"]
    #[inline]
    pub fn is_large(&self) -> bool {
        *self == TXDLYMUXSELR::LARGE
    }
}
#[doc = "Values that can be written to the field `TXDLYMUXSEL`"]
pub enum TXDLYMUXSELW {
    #[doc = "No delay"]
    NONE,
    #[doc = "Medium delay"]
    MEDIUM,
    #[doc = "Large delay"]
    LARGE,
}
impl TXDLYMUXSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXDLYMUXSELW::NONE => 0,
            TXDLYMUXSELW::MEDIUM => 2,
            TXDLYMUXSELW::LARGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXDLYMUXSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDLYMUXSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXDLYMUXSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No delay"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(TXDLYMUXSELW::NONE)
    }
    #[doc = "Medium delay"]
    #[inline]
    pub fn medium(self) -> &'a mut W {
        self.variant(TXDLYMUXSELW::MEDIUM)
    }
    #[doc = "Large delay"]
    #[inline]
    pub fn large(self) -> &'a mut W {
        self.variant(TXDLYMUXSELW::LARGE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Transmit Delay Select"]
    #[inline]
    pub fn txdlymuxsel(&self) -> TXDLYMUXSELR {
        TXDLYMUXSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Transmit Delay Select"]
    #[inline]
    pub fn txdlymuxsel(&mut self) -> _TXDLYMUXSELW {
        _TXDLYMUXSELW { w: self }
    }
}
