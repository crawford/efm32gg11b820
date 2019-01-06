#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LFCCLKSEL {
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
#[doc = "Possible values of the field `LFC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFCR {
    #[doc = "LFCCLK is disabled"]
    DISABLED,
    #[doc = "LFRCO selected as LFCCLK"]
    LFRCO,
    #[doc = "LFXO selected as LFCCLK"]
    LFXO,
    #[doc = "ULFRCO selected as LFCCLK"]
    ULFRCO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LFCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LFCR::DISABLED => 0,
            LFCR::LFRCO => 1,
            LFCR::LFXO => 2,
            LFCR::ULFRCO => 4,
            LFCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LFCR {
        match value {
            0 => LFCR::DISABLED,
            1 => LFCR::LFRCO,
            2 => LFCR::LFXO,
            4 => LFCR::ULFRCO,
            i => LFCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LFCR::DISABLED
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline]
    pub fn is_lfrco(&self) -> bool {
        *self == LFCR::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline]
    pub fn is_lfxo(&self) -> bool {
        *self == LFCR::LFXO
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline]
    pub fn is_ulfrco(&self) -> bool {
        *self == LFCR::ULFRCO
    }
}
#[doc = "Values that can be written to the field `LFC`"]
pub enum LFCW {
    #[doc = "LFCCLK is disabled"]
    DISABLED,
    #[doc = "LFRCO selected as LFCCLK"]
    LFRCO,
    #[doc = "LFXO selected as LFCCLK"]
    LFXO,
    #[doc = "ULFRCO selected as LFCCLK"]
    ULFRCO,
}
impl LFCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LFCW::DISABLED => 0,
            LFCW::LFRCO => 1,
            LFCW::LFXO => 2,
            LFCW::ULFRCO => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LFCW<'a> {
    w: &'a mut W,
}
impl<'a> _LFCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LFCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LFCCLK is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFCW::DISABLED)
    }
    #[doc = "LFRCO selected as LFCCLK"]
    #[inline]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(LFCW::LFRCO)
    }
    #[doc = "LFXO selected as LFCCLK"]
    #[inline]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(LFCW::LFXO)
    }
    #[doc = "ULFRCO selected as LFCCLK"]
    #[inline]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(LFCW::ULFRCO)
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
    #[doc = "Bits 0:2 - Clock Select for LFC"]
    #[inline]
    pub fn lfc(&self) -> LFCR {
        LFCR::_from({
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
    #[doc = "Bits 0:2 - Clock Select for LFC"]
    #[inline]
    pub fn lfc(&mut self) -> _LFCW {
        _LFCW { w: self }
    }
}
