#[doc = "Reader of register LFBPRESC0"]
pub type R = crate::R<u32, super::LFBPRESC0>;
#[doc = "Writer for register LFBPRESC0"]
pub type W = crate::W<u32, super::LFBPRESC0>;
#[doc = "Register LFBPRESC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LFBPRESC0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `LEUART0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEUART0_A {
    #[doc = "LFBCLKLEUART0 = LFBCLK"]
    DIV1,
    #[doc = "LFBCLKLEUART0 = LFBCLK/2"]
    DIV2,
    #[doc = "LFBCLKLEUART0 = LFBCLK/4"]
    DIV4,
    #[doc = "LFBCLKLEUART0 = LFBCLK/8"]
    DIV8,
}
impl crate::ToBits<u8> for LEUART0_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            LEUART0_A::DIV1 => 0,
            LEUART0_A::DIV2 => 1,
            LEUART0_A::DIV4 => 2,
            LEUART0_A::DIV8 => 3,
        }
    }
}
#[doc = "Reader of field `LEUART0`"]
pub type LEUART0_R = crate::R<u8, LEUART0_A>;
impl LEUART0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEUART0_A {
        match self.bits {
            0 => LEUART0_A::DIV1,
            1 => LEUART0_A::DIV2,
            2 => LEUART0_A::DIV4,
            3 => LEUART0_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == LEUART0_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == LEUART0_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == LEUART0_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == LEUART0_A::DIV8
    }
}
#[doc = "Write proxy for field `LEUART0`"]
pub struct LEUART0_W<'a> {
    w: &'a mut W,
}
impl<'a> LEUART0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEUART0_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(LEUART0_A::DIV1)
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(LEUART0_A::DIV2)
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(LEUART0_A::DIV4)
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(LEUART0_A::DIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `LEUART1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEUART1_A {
    #[doc = "LFBCLKLEUART1 = LFBCLK"]
    DIV1,
    #[doc = "LFBCLKLEUART1 = LFBCLK/2"]
    DIV2,
    #[doc = "LFBCLKLEUART1 = LFBCLK/4"]
    DIV4,
    #[doc = "LFBCLKLEUART1 = LFBCLK/8"]
    DIV8,
}
impl crate::ToBits<u8> for LEUART1_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            LEUART1_A::DIV1 => 0,
            LEUART1_A::DIV2 => 1,
            LEUART1_A::DIV4 => 2,
            LEUART1_A::DIV8 => 3,
        }
    }
}
#[doc = "Reader of field `LEUART1`"]
pub type LEUART1_R = crate::R<u8, LEUART1_A>;
impl LEUART1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEUART1_A {
        match self.bits {
            0 => LEUART1_A::DIV1,
            1 => LEUART1_A::DIV2,
            2 => LEUART1_A::DIV4,
            3 => LEUART1_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == LEUART1_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == LEUART1_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == LEUART1_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == LEUART1_A::DIV8
    }
}
#[doc = "Write proxy for field `LEUART1`"]
pub struct LEUART1_W<'a> {
    w: &'a mut W,
}
impl<'a> LEUART1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEUART1_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LFBCLKLEUART1 = LFBCLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(LEUART1_A::DIV1)
    }
    #[doc = "LFBCLKLEUART1 = LFBCLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(LEUART1_A::DIV2)
    }
    #[doc = "LFBCLKLEUART1 = LFBCLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(LEUART1_A::DIV4)
    }
    #[doc = "LFBCLKLEUART1 = LFBCLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(LEUART1_A::DIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `SYSTICK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSTICK_A {
    #[doc = "LFBCLKSYSTICK = LFBCLK"]
    DIV1,
}
impl crate::ToBits<u8> for SYSTICK_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SYSTICK_A::DIV1 => 0,
        }
    }
}
#[doc = "Reader of field `SYSTICK`"]
pub type SYSTICK_R = crate::R<u8, SYSTICK_A>;
impl SYSTICK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYSTICK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYSTICK_A::DIV1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == SYSTICK_A::DIV1
    }
}
#[doc = "Possible values of the field `CSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSEN_A {
    #[doc = "LFBCLKCSEN = LFBCLK/16"]
    DIV16,
    #[doc = "LFBCLKCSEN = LFBCLK/32"]
    DIV32,
    #[doc = "LFBCLKCSEN = LFBCLK/64"]
    DIV64,
    #[doc = "LFBCLKCSEN = LFBCLK/128"]
    DIV128,
}
impl crate::ToBits<u8> for CSEN_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CSEN_A::DIV16 => 0,
            CSEN_A::DIV32 => 1,
            CSEN_A::DIV64 => 2,
            CSEN_A::DIV128 => 3,
        }
    }
}
#[doc = "Reader of field `CSEN`"]
pub type CSEN_R = crate::R<u8, CSEN_A>;
impl CSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSEN_A {
        match self.bits {
            0 => CSEN_A::DIV16,
            1 => CSEN_A::DIV32,
            2 => CSEN_A::DIV64,
            3 => CSEN_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CSEN_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == CSEN_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == CSEN_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == CSEN_A::DIV128
    }
}
#[doc = "Write proxy for field `CSEN`"]
pub struct CSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSEN_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "LFBCLKCSEN = LFBCLK/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(CSEN_A::DIV16)
    }
    #[doc = "LFBCLKCSEN = LFBCLK/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(CSEN_A::DIV32)
    }
    #[doc = "LFBCLKCSEN = LFBCLK/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(CSEN_A::DIV64)
    }
    #[doc = "LFBCLKCSEN = LFBCLK/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(CSEN_A::DIV128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Low Energy UART 0 Prescaler"]
    #[inline(always)]
    pub fn leuart0(&self) -> LEUART0_R {
        LEUART0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Low Energy UART 1 Prescaler"]
    #[inline(always)]
    pub fn leuart1(&self) -> LEUART1_R {
        LEUART1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Prescaler"]
    #[inline(always)]
    pub fn systick(&self) -> SYSTICK_R {
        SYSTICK_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Capacitive touch sense module Prescaler"]
    #[inline(always)]
    pub fn csen(&self) -> CSEN_R {
        CSEN_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low Energy UART 0 Prescaler"]
    #[inline(always)]
    pub fn leuart0(&mut self) -> LEUART0_W {
        LEUART0_W { w: self }
    }
    #[doc = "Bits 4:5 - Low Energy UART 1 Prescaler"]
    #[inline(always)]
    pub fn leuart1(&mut self) -> LEUART1_W {
        LEUART1_W { w: self }
    }
    #[doc = "Bits 12:13 - Capacitive touch sense module Prescaler"]
    #[inline(always)]
    pub fn csen(&mut self) -> CSEN_W {
        CSEN_W { w: self }
    }
}
