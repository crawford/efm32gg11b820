#[doc = "Reader of register DTTIME"]
pub type R = crate::R<u32, super::DTTIME>;
#[doc = "Writer for register DTTIME"]
pub type W = crate::W<u32, super::DTTIME>;
#[doc = "Register DTTIME `reset()`'s with value 0"]
impl crate::ResetValue for super::DTTIME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DTI Prescaler Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTPRESC_A {
    #[doc = "0: The HFPERCLK is undivided"]
    DIV1 = 0,
    #[doc = "1: The HFPERCLK is divided by 2"]
    DIV2 = 1,
    #[doc = "2: The HFPERCLK is divided by 4"]
    DIV4 = 2,
    #[doc = "3: The HFPERCLK is divided by 8"]
    DIV8 = 3,
    #[doc = "4: The HFPERCLK is divided by 16"]
    DIV16 = 4,
    #[doc = "5: The HFPERCLK is divided by 32"]
    DIV32 = 5,
    #[doc = "6: The HFPERCLK is divided by 64"]
    DIV64 = 6,
    #[doc = "7: The HFPERCLK is divided by 128"]
    DIV128 = 7,
    #[doc = "8: The HFPERCLK is divided by 256"]
    DIV256 = 8,
    #[doc = "9: The HFPERCLK is divided by 512"]
    DIV512 = 9,
    #[doc = "10: The HFPERCLK is divided by 1024"]
    DIV1024 = 10,
}
impl From<DTPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: DTPRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DTPRESC`"]
pub type DTPRESC_R = crate::R<u8, DTPRESC_A>;
impl DTPRESC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DTPRESC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DTPRESC_A::DIV1),
            1 => Val(DTPRESC_A::DIV2),
            2 => Val(DTPRESC_A::DIV4),
            3 => Val(DTPRESC_A::DIV8),
            4 => Val(DTPRESC_A::DIV16),
            5 => Val(DTPRESC_A::DIV32),
            6 => Val(DTPRESC_A::DIV64),
            7 => Val(DTPRESC_A::DIV128),
            8 => Val(DTPRESC_A::DIV256),
            9 => Val(DTPRESC_A::DIV512),
            10 => Val(DTPRESC_A::DIV1024),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == DTPRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == DTPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == DTPRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == DTPRESC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == DTPRESC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == DTPRESC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == DTPRESC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == DTPRESC_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == DTPRESC_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == DTPRESC_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == DTPRESC_A::DIV1024
    }
}
#[doc = "Write proxy for field `DTPRESC`"]
pub struct DTPRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPRESC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTPRESC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The HFPERCLK is undivided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(DTPRESC_A::DIV1)
    }
    #[doc = "The HFPERCLK is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(DTPRESC_A::DIV2)
    }
    #[doc = "The HFPERCLK is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(DTPRESC_A::DIV4)
    }
    #[doc = "The HFPERCLK is divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(DTPRESC_A::DIV8)
    }
    #[doc = "The HFPERCLK is divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(DTPRESC_A::DIV16)
    }
    #[doc = "The HFPERCLK is divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(DTPRESC_A::DIV32)
    }
    #[doc = "The HFPERCLK is divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(DTPRESC_A::DIV64)
    }
    #[doc = "The HFPERCLK is divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(DTPRESC_A::DIV128)
    }
    #[doc = "The HFPERCLK is divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(DTPRESC_A::DIV256)
    }
    #[doc = "The HFPERCLK is divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(DTPRESC_A::DIV512)
    }
    #[doc = "The HFPERCLK is divided by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(DTPRESC_A::DIV1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DTRISET`"]
pub type DTRISET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTRISET`"]
pub struct DTRISET_W<'a> {
    w: &'a mut W,
}
impl<'a> DTRISET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `DTFALLT`"]
pub type DTFALLT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTFALLT`"]
pub struct DTFALLT_W<'a> {
    w: &'a mut W,
}
impl<'a> DTFALLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - DTI Prescaler Setting"]
    #[inline(always)]
    pub fn dtpresc(&self) -> DTPRESC_R {
        DTPRESC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - DTI Rise-time"]
    #[inline(always)]
    pub fn dtriset(&self) -> DTRISET_R {
        DTRISET_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - DTI Fall-time"]
    #[inline(always)]
    pub fn dtfallt(&self) -> DTFALLT_R {
        DTFALLT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DTI Prescaler Setting"]
    #[inline(always)]
    pub fn dtpresc(&mut self) -> DTPRESC_W {
        DTPRESC_W { w: self }
    }
    #[doc = "Bits 8:13 - DTI Rise-time"]
    #[inline(always)]
    pub fn dtriset(&mut self) -> DTRISET_W {
        DTRISET_W { w: self }
    }
    #[doc = "Bits 16:21 - DTI Fall-time"]
    #[inline(always)]
    pub fn dtfallt(&mut self) -> DTFALLT_W {
        DTFALLT_W { w: self }
    }
}
