#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DBGCLKSEL {
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
#[doc = "Possible values of the field `DBG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGR {
    #[doc = "AUXHFRCO is the debug trace clock"]
    AUXHFRCO,
    #[doc = "HFCLK is the debug trace clock"]
    HFCLK,
    #[doc = "HFRCO divided by 2 is the debug trace clock"]
    HFRCODIV2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DBGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DBGR::AUXHFRCO => 0,
            DBGR::HFCLK => 1,
            DBGR::HFRCODIV2 => 2,
            DBGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DBGR {
        match value {
            0 => DBGR::AUXHFRCO,
            1 => DBGR::HFCLK,
            2 => DBGR::HFRCODIV2,
            i => DBGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline]
    pub fn is_auxhfrco(&self) -> bool {
        *self == DBGR::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `HFCLK`"]
    #[inline]
    pub fn is_hfclk(&self) -> bool {
        *self == DBGR::HFCLK
    }
    #[doc = "Checks if the value of the field is `HFRCODIV2`"]
    #[inline]
    pub fn is_hfrcodiv2(&self) -> bool {
        *self == DBGR::HFRCODIV2
    }
}
#[doc = "Values that can be written to the field `DBG`"]
pub enum DBGW {
    #[doc = "AUXHFRCO is the debug trace clock"]
    AUXHFRCO,
    #[doc = "HFCLK is the debug trace clock"]
    HFCLK,
    #[doc = "HFRCO divided by 2 is the debug trace clock"]
    HFRCODIV2,
}
impl DBGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DBGW::AUXHFRCO => 0,
            DBGW::HFCLK => 1,
            DBGW::HFRCODIV2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBGW<'a> {
    w: &'a mut W,
}
impl<'a> _DBGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "AUXHFRCO is the debug trace clock"]
    #[inline]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(DBGW::AUXHFRCO)
    }
    #[doc = "HFCLK is the debug trace clock"]
    #[inline]
    pub fn hfclk(self) -> &'a mut W {
        self.variant(DBGW::HFCLK)
    }
    #[doc = "HFRCO divided by 2 is the debug trace clock"]
    #[inline]
    pub fn hfrcodiv2(self) -> &'a mut W {
        self.variant(DBGW::HFRCODIV2)
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
    #[doc = "Bits 0:1 - Debug Trace Clock"]
    #[inline]
    pub fn dbg(&self) -> DBGR {
        DBGR::_from({
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
    #[doc = "Bits 0:1 - Debug Trace Clock"]
    #[inline]
    pub fn dbg(&mut self) -> _DBGW {
        _DBGW { w: self }
    }
}
