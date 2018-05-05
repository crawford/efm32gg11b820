#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LFBPRESC0 {
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
#[doc = "Possible values of the field `LEUART0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEUART0R {
    #[doc = "LFBCLKLEUART0 = LFBCLK"]
    DIV1,
    #[doc = "LFBCLKLEUART0 = LFBCLK/2"]
    DIV2,
    #[doc = "LFBCLKLEUART0 = LFBCLK/4"]
    DIV4,
    #[doc = "LFBCLKLEUART0 = LFBCLK/8"]
    DIV8,
}
impl LEUART0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LEUART0R::DIV1 => 0,
            LEUART0R::DIV2 => 1,
            LEUART0R::DIV4 => 2,
            LEUART0R::DIV8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LEUART0R {
        match value {
            0 => LEUART0R::DIV1,
            1 => LEUART0R::DIV2,
            2 => LEUART0R::DIV4,
            3 => LEUART0R::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == LEUART0R::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == LEUART0R::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == LEUART0R::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == LEUART0R::DIV8
    }
}
#[doc = "Possible values of the field `LEUART1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEUART1R {
    #[doc = "LFBCLKLEUART1 = LFBCLK"]
    DIV1,
    #[doc = "LFBCLKLEUART1 = LFBCLK/2"]
    DIV2,
    #[doc = "LFBCLKLEUART1 = LFBCLK/4"]
    DIV4,
    #[doc = "LFBCLKLEUART1 = LFBCLK/8"]
    DIV8,
}
impl LEUART1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LEUART1R::DIV1 => 0,
            LEUART1R::DIV2 => 1,
            LEUART1R::DIV4 => 2,
            LEUART1R::DIV8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LEUART1R {
        match value {
            0 => LEUART1R::DIV1,
            1 => LEUART1R::DIV2,
            2 => LEUART1R::DIV4,
            3 => LEUART1R::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == LEUART1R::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == LEUART1R::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == LEUART1R::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == LEUART1R::DIV8
    }
}
#[doc = "Possible values of the field `SYSTICK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSTICKR {
    #[doc = "LFBCLKSYSTICK = LFBCLK"]
    DIV1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SYSTICKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSTICKR::DIV1 => 0,
            SYSTICKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYSTICKR {
        match value {
            0 => SYSTICKR::DIV1,
            i => SYSTICKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == SYSTICKR::DIV1
    }
}
#[doc = "Possible values of the field `CSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSENR {
    #[doc = "LFBCLKCSEN = LFBCLK/16"]
    DIV16,
    #[doc = "LFBCLKCSEN = LFBCLK/32"]
    DIV32,
    #[doc = "LFBCLKCSEN = LFBCLK/64"]
    DIV64,
    #[doc = "LFBCLKCSEN = LFBCLK/128"]
    DIV128,
}
impl CSENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSENR::DIV16 => 0,
            CSENR::DIV32 => 1,
            CSENR::DIV64 => 2,
            CSENR::DIV128 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSENR {
        match value {
            0 => CSENR::DIV16,
            1 => CSENR::DIV32,
            2 => CSENR::DIV64,
            3 => CSENR::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == CSENR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == CSENR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == CSENR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == CSENR::DIV128
    }
}
#[doc = "Values that can be written to the field `LEUART0`"]
pub enum LEUART0W {
    #[doc = "LFBCLKLEUART0 = LFBCLK"]
    DIV1,
    #[doc = "LFBCLKLEUART0 = LFBCLK/2"]
    DIV2,
    #[doc = "LFBCLKLEUART0 = LFBCLK/4"]
    DIV4,
    #[doc = "LFBCLKLEUART0 = LFBCLK/8"]
    DIV8,
}
impl LEUART0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LEUART0W::DIV1 => 0,
            LEUART0W::DIV2 => 1,
            LEUART0W::DIV4 => 2,
            LEUART0W::DIV8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LEUART0W<'a> {
    w: &'a mut W,
}
impl<'a> _LEUART0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LEUART0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(LEUART0W::DIV1)
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(LEUART0W::DIV2)
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(LEUART0W::DIV4)
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(LEUART0W::DIV8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LEUART1`"]
pub enum LEUART1W {
    #[doc = "LFBCLKLEUART1 = LFBCLK"]
    DIV1,
    #[doc = "LFBCLKLEUART1 = LFBCLK/2"]
    DIV2,
    #[doc = "LFBCLKLEUART1 = LFBCLK/4"]
    DIV4,
    #[doc = "LFBCLKLEUART1 = LFBCLK/8"]
    DIV8,
}
impl LEUART1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LEUART1W::DIV1 => 0,
            LEUART1W::DIV2 => 1,
            LEUART1W::DIV4 => 2,
            LEUART1W::DIV8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LEUART1W<'a> {
    w: &'a mut W,
}
impl<'a> _LEUART1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LEUART1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LFBCLKLEUART1 = LFBCLK"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(LEUART1W::DIV1)
    }
    #[doc = "LFBCLKLEUART1 = LFBCLK/2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(LEUART1W::DIV2)
    }
    #[doc = "LFBCLKLEUART1 = LFBCLK/4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(LEUART1W::DIV4)
    }
    #[doc = "LFBCLKLEUART1 = LFBCLK/8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(LEUART1W::DIV8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CSEN`"]
pub enum CSENW {
    #[doc = "LFBCLKCSEN = LFBCLK/16"]
    DIV16,
    #[doc = "LFBCLKCSEN = LFBCLK/32"]
    DIV32,
    #[doc = "LFBCLKCSEN = LFBCLK/64"]
    DIV64,
    #[doc = "LFBCLKCSEN = LFBCLK/128"]
    DIV128,
}
impl CSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSENW::DIV16 => 0,
            CSENW::DIV32 => 1,
            CSENW::DIV64 => 2,
            CSENW::DIV128 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSENW<'a> {
    w: &'a mut W,
}
impl<'a> _CSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LFBCLKCSEN = LFBCLK/16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(CSENW::DIV16)
    }
    #[doc = "LFBCLKCSEN = LFBCLK/32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(CSENW::DIV32)
    }
    #[doc = "LFBCLKCSEN = LFBCLK/64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(CSENW::DIV64)
    }
    #[doc = "LFBCLKCSEN = LFBCLK/128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(CSENW::DIV128)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:1 - Low Energy UART 0 Prescaler"]
    #[inline]
    pub fn leuart0(&self) -> LEUART0R {
        LEUART0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Low Energy UART 1 Prescaler"]
    #[inline]
    pub fn leuart1(&self) -> LEUART1R {
        LEUART1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Prescaler"]
    #[inline]
    pub fn systick(&self) -> SYSTICKR {
        SYSTICKR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Capacitive touch sense module Prescaler"]
    #[inline]
    pub fn csen(&self) -> CSENR {
        CSENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:1 - Low Energy UART 0 Prescaler"]
    #[inline]
    pub fn leuart0(&mut self) -> _LEUART0W {
        _LEUART0W { w: self }
    }
    #[doc = "Bits 4:5 - Low Energy UART 1 Prescaler"]
    #[inline]
    pub fn leuart1(&mut self) -> _LEUART1W {
        _LEUART1W { w: self }
    }
    #[doc = "Bits 12:13 - Capacitive touch sense module Prescaler"]
    #[inline]
    pub fn csen(&mut self) -> _CSENW {
        _CSENW { w: self }
    }
}
