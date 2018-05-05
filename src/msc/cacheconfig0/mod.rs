#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CACHECONFIG0 {
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
#[doc = "Possible values of the field `CACHELPLEVEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHELPLEVELR {
    #[doc = "Base instruction cache functionality."]
    BASE,
    #[doc = "Advanced buffering mode, where the cache uses the fetch pattern to predict highly accessed data and store it in low-energy memory."]
    ADVANCED,
    #[doc = "Minimum activity mode, which allows the cache to minimize activity in logic that it predicts has a low probability being used. This mode can introduce wait-states into the instruction fetch stream when the cache exits one of its low-activity states. The number of wait-states introduced is small, but users running with 0-wait-state memory and wishing to reduce the variability that the cache might introduce with additional wait-states may wish to lower the cache low-power level. Note, this mode includes the advanced buffering mode functionality."]
    MINACTIVITY,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CACHELPLEVELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CACHELPLEVELR::BASE => 0,
            CACHELPLEVELR::ADVANCED => 1,
            CACHELPLEVELR::MINACTIVITY => 3,
            CACHELPLEVELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CACHELPLEVELR {
        match value {
            0 => CACHELPLEVELR::BASE,
            1 => CACHELPLEVELR::ADVANCED,
            3 => CACHELPLEVELR::MINACTIVITY,
            i => CACHELPLEVELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BASE`"]
    #[inline]
    pub fn is_base(&self) -> bool {
        *self == CACHELPLEVELR::BASE
    }
    #[doc = "Checks if the value of the field is `ADVANCED`"]
    #[inline]
    pub fn is_advanced(&self) -> bool {
        *self == CACHELPLEVELR::ADVANCED
    }
    #[doc = "Checks if the value of the field is `MINACTIVITY`"]
    #[inline]
    pub fn is_minactivity(&self) -> bool {
        *self == CACHELPLEVELR::MINACTIVITY
    }
}
#[doc = "Values that can be written to the field `CACHELPLEVEL`"]
pub enum CACHELPLEVELW {
    #[doc = "Base instruction cache functionality."]
    BASE,
    #[doc = "Advanced buffering mode, where the cache uses the fetch pattern to predict highly accessed data and store it in low-energy memory."]
    ADVANCED,
    #[doc = "Minimum activity mode, which allows the cache to minimize activity in logic that it predicts has a low probability being used. This mode can introduce wait-states into the instruction fetch stream when the cache exits one of its low-activity states. The number of wait-states introduced is small, but users running with 0-wait-state memory and wishing to reduce the variability that the cache might introduce with additional wait-states may wish to lower the cache low-power level. Note, this mode includes the advanced buffering mode functionality."]
    MINACTIVITY,
}
impl CACHELPLEVELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CACHELPLEVELW::BASE => 0,
            CACHELPLEVELW::ADVANCED => 1,
            CACHELPLEVELW::MINACTIVITY => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CACHELPLEVELW<'a> {
    w: &'a mut W,
}
impl<'a> _CACHELPLEVELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CACHELPLEVELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Base instruction cache functionality."]
    #[inline]
    pub fn base(self) -> &'a mut W {
        self.variant(CACHELPLEVELW::BASE)
    }
    #[doc = "Advanced buffering mode, where the cache uses the fetch pattern to predict highly accessed data and store it in low-energy memory."]
    #[inline]
    pub fn advanced(self) -> &'a mut W {
        self.variant(CACHELPLEVELW::ADVANCED)
    }
    #[doc = "Minimum activity mode, which allows the cache to minimize activity in logic that it predicts has a low probability being used. This mode can introduce wait-states into the instruction fetch stream when the cache exits one of its low-activity states. The number of wait-states introduced is small, but users running with 0-wait-state memory and wishing to reduce the variability that the cache might introduce with additional wait-states may wish to lower the cache low-power level. Note, this mode includes the advanced buffering mode functionality."]
    #[inline]
    pub fn minactivity(self) -> &'a mut W {
        self.variant(CACHELPLEVELW::MINACTIVITY)
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
    #[doc = "Bits 0:1 - Instruction Cache Low-Power Level"]
    #[inline]
    pub fn cachelplevel(&self) -> CACHELPLEVELR {
        CACHELPLEVELR::_from({
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
        W { bits: 3 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Instruction Cache Low-Power Level"]
    #[inline]
    pub fn cachelplevel(&mut self) -> _CACHELPLEVELW {
        _CACHELPLEVELW { w: self }
    }
}
