#[doc = "Reader of register EMACTRL"]
pub type R = crate::R<u32, super::EMACTRL>;
#[doc = "Writer for register EMACTRL"]
pub type W = crate::W<u32, super::EMACTRL>;
#[doc = "Register EMACTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::EMACTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "EMA Sample Weight\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMASAMPLE_A {
    #[doc = "0: EMA weight (N) is 1."]
    W1,
    #[doc = "1: EMA weight (N) is 2."]
    W2,
    #[doc = "2: EMA weight (N) is 4."]
    W4,
    #[doc = "3: EMA weight (N) is 8."]
    W8,
    #[doc = "4: EMA weight (N) is 16."]
    W16,
    #[doc = "5: EMA weight (N) is 32."]
    W32,
    #[doc = "6: EMA weight (N) is 64."]
    W64,
}
impl From<EMASAMPLE_A> for u8 {
    #[inline(always)]
    fn from(variant: EMASAMPLE_A) -> Self {
        match variant {
            EMASAMPLE_A::W1 => 0,
            EMASAMPLE_A::W2 => 1,
            EMASAMPLE_A::W4 => 2,
            EMASAMPLE_A::W8 => 3,
            EMASAMPLE_A::W16 => 4,
            EMASAMPLE_A::W32 => 5,
            EMASAMPLE_A::W64 => 6,
        }
    }
}
#[doc = "Reader of field `EMASAMPLE`"]
pub type EMASAMPLE_R = crate::R<u8, EMASAMPLE_A>;
impl EMASAMPLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EMASAMPLE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EMASAMPLE_A::W1),
            1 => Val(EMASAMPLE_A::W2),
            2 => Val(EMASAMPLE_A::W4),
            3 => Val(EMASAMPLE_A::W8),
            4 => Val(EMASAMPLE_A::W16),
            5 => Val(EMASAMPLE_A::W32),
            6 => Val(EMASAMPLE_A::W64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `W1`"]
    #[inline(always)]
    pub fn is_w1(&self) -> bool {
        *self == EMASAMPLE_A::W1
    }
    #[doc = "Checks if the value of the field is `W2`"]
    #[inline(always)]
    pub fn is_w2(&self) -> bool {
        *self == EMASAMPLE_A::W2
    }
    #[doc = "Checks if the value of the field is `W4`"]
    #[inline(always)]
    pub fn is_w4(&self) -> bool {
        *self == EMASAMPLE_A::W4
    }
    #[doc = "Checks if the value of the field is `W8`"]
    #[inline(always)]
    pub fn is_w8(&self) -> bool {
        *self == EMASAMPLE_A::W8
    }
    #[doc = "Checks if the value of the field is `W16`"]
    #[inline(always)]
    pub fn is_w16(&self) -> bool {
        *self == EMASAMPLE_A::W16
    }
    #[doc = "Checks if the value of the field is `W32`"]
    #[inline(always)]
    pub fn is_w32(&self) -> bool {
        *self == EMASAMPLE_A::W32
    }
    #[doc = "Checks if the value of the field is `W64`"]
    #[inline(always)]
    pub fn is_w64(&self) -> bool {
        *self == EMASAMPLE_A::W64
    }
}
#[doc = "Write proxy for field `EMASAMPLE`"]
pub struct EMASAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> EMASAMPLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMASAMPLE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "EMA weight (N) is 1."]
    #[inline(always)]
    pub fn w1(self) -> &'a mut W {
        self.variant(EMASAMPLE_A::W1)
    }
    #[doc = "EMA weight (N) is 2."]
    #[inline(always)]
    pub fn w2(self) -> &'a mut W {
        self.variant(EMASAMPLE_A::W2)
    }
    #[doc = "EMA weight (N) is 4."]
    #[inline(always)]
    pub fn w4(self) -> &'a mut W {
        self.variant(EMASAMPLE_A::W4)
    }
    #[doc = "EMA weight (N) is 8."]
    #[inline(always)]
    pub fn w8(self) -> &'a mut W {
        self.variant(EMASAMPLE_A::W8)
    }
    #[doc = "EMA weight (N) is 16."]
    #[inline(always)]
    pub fn w16(self) -> &'a mut W {
        self.variant(EMASAMPLE_A::W16)
    }
    #[doc = "EMA weight (N) is 32."]
    #[inline(always)]
    pub fn w32(self) -> &'a mut W {
        self.variant(EMASAMPLE_A::W32)
    }
    #[doc = "EMA weight (N) is 64."]
    #[inline(always)]
    pub fn w64(self) -> &'a mut W {
        self.variant(EMASAMPLE_A::W64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - EMA Sample Weight"]
    #[inline(always)]
    pub fn emasample(&self) -> EMASAMPLE_R {
        EMASAMPLE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - EMA Sample Weight"]
    #[inline(always)]
    pub fn emasample(&mut self) -> EMASAMPLE_W {
        EMASAMPLE_W { w: self }
    }
}
