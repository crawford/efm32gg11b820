#[doc = "Reader of register TIMCTRL"]
pub type R = crate::R<u32, super::TIMCTRL>;
#[doc = "Writer for register TIMCTRL"]
pub type W = crate::W<u32, super::TIMCTRL>;
#[doc = "Register TIMCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Period Counter Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCPRESC_A {
    #[doc = "0: The period counter clock frequency is LFBCLKCSEN/1"]
    DIV1,
    #[doc = "1: The period counter clock frequency is LFBCLKCSEN/2"]
    DIV2,
    #[doc = "2: The period counter clock frequency is LFBCLKCSEN/4"]
    DIV4,
    #[doc = "3: The period counter clock frequency is LFBCLKCSEN/8"]
    DIV8,
    #[doc = "4: The period counter clock frequency is LFBCLKCSEN/16"]
    DIV16,
    #[doc = "5: The period counter clock frequency is LFBCLKCSEN/32"]
    DIV32,
    #[doc = "6: The period counter clock frequency is LFBCLKCSEN/64"]
    DIV64,
    #[doc = "7: The period counter clock frequency is LFBCLKCSEN/128"]
    DIV128,
}
impl From<PCPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PCPRESC_A) -> Self {
        match variant {
            PCPRESC_A::DIV1 => 0,
            PCPRESC_A::DIV2 => 1,
            PCPRESC_A::DIV4 => 2,
            PCPRESC_A::DIV8 => 3,
            PCPRESC_A::DIV16 => 4,
            PCPRESC_A::DIV32 => 5,
            PCPRESC_A::DIV64 => 6,
            PCPRESC_A::DIV128 => 7,
        }
    }
}
#[doc = "Reader of field `PCPRESC`"]
pub type PCPRESC_R = crate::R<u8, PCPRESC_A>;
impl PCPRESC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCPRESC_A {
        match self.bits {
            0 => PCPRESC_A::DIV1,
            1 => PCPRESC_A::DIV2,
            2 => PCPRESC_A::DIV4,
            3 => PCPRESC_A::DIV8,
            4 => PCPRESC_A::DIV16,
            5 => PCPRESC_A::DIV32,
            6 => PCPRESC_A::DIV64,
            7 => PCPRESC_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PCPRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PCPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PCPRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PCPRESC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PCPRESC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PCPRESC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PCPRESC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PCPRESC_A::DIV128
    }
}
#[doc = "Write proxy for field `PCPRESC`"]
pub struct PCPRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCPRESC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCPRESC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV1)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV2)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV4)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV8)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV16)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV32)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV64)
    }
    #[doc = "The period counter clock frequency is LFBCLKCSEN/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `PCTOP`"]
pub type PCTOP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCTOP`"]
pub struct PCTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> PCTOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `WARMUPCNT`"]
pub type WARMUPCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WARMUPCNT`"]
pub struct WARMUPCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> WARMUPCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Period Counter Prescaler"]
    #[inline(always)]
    pub fn pcpresc(&self) -> PCPRESC_R {
        PCPRESC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:15 - Period Counter Top Value"]
    #[inline(always)]
    pub fn pctop(&self) -> PCTOP_R {
        PCTOP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Warmup Period Counter"]
    #[inline(always)]
    pub fn warmupcnt(&self) -> WARMUPCNT_R {
        WARMUPCNT_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Period Counter Prescaler"]
    #[inline(always)]
    pub fn pcpresc(&mut self) -> PCPRESC_W {
        PCPRESC_W { w: self }
    }
    #[doc = "Bits 8:15 - Period Counter Top Value"]
    #[inline(always)]
    pub fn pctop(&mut self) -> PCTOP_W {
        PCTOP_W { w: self }
    }
    #[doc = "Bits 16:17 - Warmup Period Counter"]
    #[inline(always)]
    pub fn warmupcnt(&mut self) -> WARMUPCNT_W {
        WARMUPCNT_W { w: self }
    }
}
