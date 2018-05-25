#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIMCTRL {
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
#[doc = "Possible values of the field `PCPRESC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCPRESCR {
    #[doc = "The period counter clock frequency is LFBCLKCSEN/1"]
    DIV1,
    #[doc = "The period counter clock frequency is LFBCLKCSEN/2"]
    DIV2,
    #[doc = "The period counter clock frequency is LFBCLKCSEN/4"]
    DIV4,
    #[doc = "The period counter clock frequency is LFBCLKCSEN/8"]
    DIV8,
    #[doc = "The period counter clock frequency is LFBCLKCSEN/16"]
    DIV16,
    #[doc = "The period counter clock frequency is LFBCLKCSEN/32"]
    DIV32,
    #[doc = "The period counter clock frequency is LFBCLKCSEN/64"]
    DIV64,
    #[doc = "The period counter clock frequency is LFBCLKCSEN/128"]
    DIV128,
}
impl PCPRESCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCPRESCR::DIV1 => 0,
            PCPRESCR::DIV2 => 1,
            PCPRESCR::DIV4 => 2,
            PCPRESCR::DIV8 => 3,
            PCPRESCR::DIV16 => 4,
            PCPRESCR::DIV32 => 5,
            PCPRESCR::DIV64 => 6,
            PCPRESCR::DIV128 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCPRESCR {
        match value {
            0 => PCPRESCR::DIV1,
            1 => PCPRESCR::DIV2,
            2 => PCPRESCR::DIV4,
            3 => PCPRESCR::DIV8,
            4 => PCPRESCR::DIV16,
            5 => PCPRESCR::DIV32,
            6 => PCPRESCR::DIV64,
            7 => PCPRESCR::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == PCPRESCR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == PCPRESCR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == PCPRESCR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == PCPRESCR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == PCPRESCR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == PCPRESCR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == PCPRESCR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == PCPRESCR::DIV128
    }
}
#[doc = r" Value of the field"]
pub struct PCTOPR {
    bits: u8,
}
impl PCTOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WARMUPCNTR {
    bits: u8,
}
impl WARMUPCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `PCPRESC`"]
pub enum PCPRESCW {
    #[doc = "The period counter clock frequency is LFBCLKCSEN/1"]
    DIV1,
    #[doc = "The period counter clock frequency is LFBCLKCSEN/2"]
    DIV2,
    #[doc = "The period counter clock frequency is LFBCLKCSEN/4"]
    DIV4,
    #[doc = "The period counter clock frequency is LFBCLKCSEN/8"]
    DIV8,
    #[doc = "The period counter clock frequency is LFBCLKCSEN/16"]
    DIV16,
    #[doc = "The period counter clock frequency is LFBCLKCSEN/32"]
    DIV32,
    #[doc = "The period counter clock frequency is LFBCLKCSEN/64"]
    DIV64,
    #[doc = "The period counter clock frequency is LFBCLKCSEN/128"]
    DIV128,
}
impl PCPRESCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCPRESCW::DIV1 => 0,
            PCPRESCW::DIV2 => 1,
            PCPRESCW::DIV4 => 2,
            PCPRESCW::DIV8 => 3,
            PCPRESCW::DIV16 => 4,
            PCPRESCW::DIV32 => 5,
            PCPRESCW::DIV64 => 6,
            PCPRESCW::DIV128 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCPRESCW<'a> {
    w: &'a mut W,
}
impl<'a> _PCPRESCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCPRESCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/1"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(PCPRESCW::DIV1)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PCPRESCW::DIV2)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PCPRESCW::DIV4)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PCPRESCW::DIV8)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(PCPRESCW::DIV16)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(PCPRESCW::DIV32)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(PCPRESCW::DIV64)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(PCPRESCW::DIV128)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PCTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _PCTOPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WARMUPCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _WARMUPCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:2 - Period Counter Prescaler"]
    #[inline]
    pub fn pcpresc(&self) -> PCPRESCR {
        PCPRESCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Period Counter Top Value"]
    #[inline]
    pub fn pctop(&self) -> PCTOPR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PCTOPR { bits }
    }
    #[doc = "Bits 16:17 - Warmup Period Counter"]
    #[inline]
    pub fn warmupcnt(&self) -> WARMUPCNTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WARMUPCNTR { bits }
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
    #[doc = "Bits 0:2 - Period Counter Prescaler"]
    #[inline]
    pub fn pcpresc(&mut self) -> _PCPRESCW {
        _PCPRESCW { w: self }
    }
    #[doc = "Bits 8:15 - Period Counter Top Value"]
    #[inline]
    pub fn pctop(&mut self) -> _PCTOPW {
        _PCTOPW { w: self }
    }
    #[doc = "Bits 16:17 - Warmup Period Counter"]
    #[inline]
    pub fn warmupcnt(&mut self) -> _WARMUPCNTW {
        _WARMUPCNTW { w: self }
    }
}
