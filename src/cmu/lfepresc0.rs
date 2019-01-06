#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LFEPRESC0 {
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
        R { bits: self.register.get() }
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
#[doc = "Possible values of the field `RTCC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCCR {
    #[doc = "LFECLKRTCC = LFECLK"]
    DIV1,
    #[doc = "LFECLKRTCC = LFECLK/2"]
    DIV2,
    #[doc = "LFECLKRTCC = LFECLK/4"]
    DIV4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RTCCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RTCCR::DIV1 => 0,
            RTCCR::DIV2 => 1,
            RTCCR::DIV4 => 2,
            RTCCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RTCCR {
        match value {
            0 => RTCCR::DIV1,
            1 => RTCCR::DIV2,
            2 => RTCCR::DIV4,
            i => RTCCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == RTCCR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == RTCCR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == RTCCR::DIV4
    }
}
#[doc = "Values that can be written to the field `RTCC`"]
pub enum RTCCW {
    #[doc = "LFECLKRTCC = LFECLK"]
    DIV1,
    #[doc = "LFECLKRTCC = LFECLK/2"]
    DIV2,
    #[doc = "LFECLKRTCC = LFECLK/4"]
    DIV4,
}
impl RTCCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RTCCW::DIV1 => 0,
            RTCCW::DIV2 => 1,
            RTCCW::DIV4 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCCW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LFECLKRTCC = LFECLK"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(RTCCW::DIV1)
    }
    #[doc = "LFECLKRTCC = LFECLK/2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(RTCCW::DIV2)
    }
    #[doc = "LFECLKRTCC = LFECLK/4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(RTCCW::DIV4)
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
    #[doc = "Bits 0:1 - Real-Time Counter and Calendar Prescaler"]
    #[inline]
    pub fn rtcc(&self) -> RTCCR {
        RTCCR::_from({
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
    #[doc = "Bits 0:1 - Real-Time Counter and Calendar Prescaler"]
    #[inline]
    pub fn rtcc(&mut self) -> _RTCCW {
        _RTCCW { w: self }
    }
}
