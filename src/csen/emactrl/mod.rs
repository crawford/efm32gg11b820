#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EMACTRL {
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
#[doc = "Possible values of the field `EMASAMPLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMASAMPLER {
    #[doc = "EMA weight (N) is 1."]
    W1,
    #[doc = "EMA weight (N) is 2."]
    W2,
    #[doc = "EMA weight (N) is 4."]
    W4,
    #[doc = "EMA weight (N) is 8."]
    W8,
    #[doc = "EMA weight (N) is 16."]
    W16,
    #[doc = "EMA weight (N) is 32."]
    W32,
    #[doc = "EMA weight (N) is 64."]
    W64,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EMASAMPLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EMASAMPLER::W1 => 0,
            EMASAMPLER::W2 => 1,
            EMASAMPLER::W4 => 2,
            EMASAMPLER::W8 => 3,
            EMASAMPLER::W16 => 4,
            EMASAMPLER::W32 => 5,
            EMASAMPLER::W64 => 6,
            EMASAMPLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EMASAMPLER {
        match value {
            0 => EMASAMPLER::W1,
            1 => EMASAMPLER::W2,
            2 => EMASAMPLER::W4,
            3 => EMASAMPLER::W8,
            4 => EMASAMPLER::W16,
            5 => EMASAMPLER::W32,
            6 => EMASAMPLER::W64,
            i => EMASAMPLER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `W1`"]
    #[inline]
    pub fn is_w1(&self) -> bool {
        *self == EMASAMPLER::W1
    }
    #[doc = "Checks if the value of the field is `W2`"]
    #[inline]
    pub fn is_w2(&self) -> bool {
        *self == EMASAMPLER::W2
    }
    #[doc = "Checks if the value of the field is `W4`"]
    #[inline]
    pub fn is_w4(&self) -> bool {
        *self == EMASAMPLER::W4
    }
    #[doc = "Checks if the value of the field is `W8`"]
    #[inline]
    pub fn is_w8(&self) -> bool {
        *self == EMASAMPLER::W8
    }
    #[doc = "Checks if the value of the field is `W16`"]
    #[inline]
    pub fn is_w16(&self) -> bool {
        *self == EMASAMPLER::W16
    }
    #[doc = "Checks if the value of the field is `W32`"]
    #[inline]
    pub fn is_w32(&self) -> bool {
        *self == EMASAMPLER::W32
    }
    #[doc = "Checks if the value of the field is `W64`"]
    #[inline]
    pub fn is_w64(&self) -> bool {
        *self == EMASAMPLER::W64
    }
}
#[doc = "Values that can be written to the field `EMASAMPLE`"]
pub enum EMASAMPLEW {
    #[doc = "EMA weight (N) is 1."]
    W1,
    #[doc = "EMA weight (N) is 2."]
    W2,
    #[doc = "EMA weight (N) is 4."]
    W4,
    #[doc = "EMA weight (N) is 8."]
    W8,
    #[doc = "EMA weight (N) is 16."]
    W16,
    #[doc = "EMA weight (N) is 32."]
    W32,
    #[doc = "EMA weight (N) is 64."]
    W64,
}
impl EMASAMPLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMASAMPLEW::W1 => 0,
            EMASAMPLEW::W2 => 1,
            EMASAMPLEW::W4 => 2,
            EMASAMPLEW::W8 => 3,
            EMASAMPLEW::W16 => 4,
            EMASAMPLEW::W32 => 5,
            EMASAMPLEW::W64 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMASAMPLEW<'a> {
    w: &'a mut W,
}
impl<'a> _EMASAMPLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMASAMPLEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "EMA weight (N) is 1."]
    #[inline]
    pub fn w1(self) -> &'a mut W {
        self.variant(EMASAMPLEW::W1)
    }
    #[doc = "EMA weight (N) is 2."]
    #[inline]
    pub fn w2(self) -> &'a mut W {
        self.variant(EMASAMPLEW::W2)
    }
    #[doc = "EMA weight (N) is 4."]
    #[inline]
    pub fn w4(self) -> &'a mut W {
        self.variant(EMASAMPLEW::W4)
    }
    #[doc = "EMA weight (N) is 8."]
    #[inline]
    pub fn w8(self) -> &'a mut W {
        self.variant(EMASAMPLEW::W8)
    }
    #[doc = "EMA weight (N) is 16."]
    #[inline]
    pub fn w16(self) -> &'a mut W {
        self.variant(EMASAMPLEW::W16)
    }
    #[doc = "EMA weight (N) is 32."]
    #[inline]
    pub fn w32(self) -> &'a mut W {
        self.variant(EMASAMPLEW::W32)
    }
    #[doc = "EMA weight (N) is 64."]
    #[inline]
    pub fn w64(self) -> &'a mut W {
        self.variant(EMASAMPLEW::W64)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 0:2 - EMA Sample Weight"]
    #[inline]
    pub fn emasample(&self) -> EMASAMPLER {
        EMASAMPLER::_from({
            const MASK: u8 = 7;
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
    #[doc = "Bits 0:2 - EMA Sample Weight"]
    #[inline]
    pub fn emasample(&mut self) -> _EMASAMPLEW {
        _EMASAMPLEW { w: self }
    }
}
