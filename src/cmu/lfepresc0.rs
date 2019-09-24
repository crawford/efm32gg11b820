#[doc = "Reader of register LFEPRESC0"]
pub type R = crate::R<u32, super::LFEPRESC0>;
#[doc = "Writer for register LFEPRESC0"]
pub type W = crate::W<u32, super::LFEPRESC0>;
#[doc = "Register LFEPRESC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LFEPRESC0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Real-Time Counter and Calendar Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCC_A {
    #[doc = "0: LFECLKRTCC = LFECLK"]
    DIV1,
    #[doc = "1: LFECLKRTCC = LFECLK/2"]
    DIV2,
    #[doc = "2: LFECLKRTCC = LFECLK/4"]
    DIV4,
}
impl From<RTCC_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCC_A) -> Self {
        match variant {
            RTCC_A::DIV1 => 0,
            RTCC_A::DIV2 => 1,
            RTCC_A::DIV4 => 2,
        }
    }
}
#[doc = "Reader of field `RTCC`"]
pub type RTCC_R = crate::R<u8, RTCC_A>;
impl RTCC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RTCC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RTCC_A::DIV1),
            1 => Val(RTCC_A::DIV2),
            2 => Val(RTCC_A::DIV4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == RTCC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == RTCC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == RTCC_A::DIV4
    }
}
#[doc = "Write proxy for field `RTCC`"]
pub struct RTCC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LFECLKRTCC = LFECLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(RTCC_A::DIV1)
    }
    #[doc = "LFECLKRTCC = LFECLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(RTCC_A::DIV2)
    }
    #[doc = "LFECLKRTCC = LFECLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(RTCC_A::DIV4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Real-Time Counter and Calendar Prescaler"]
    #[inline(always)]
    pub fn rtcc(&self) -> RTCC_R {
        RTCC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Real-Time Counter and Calendar Prescaler"]
    #[inline(always)]
    pub fn rtcc(&mut self) -> RTCC_W {
        RTCC_W { w: self }
    }
}
