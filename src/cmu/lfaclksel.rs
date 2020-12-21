#[doc = "Reader of register LFACLKSEL"]
pub type R = crate::R<u32, super::LFACLKSEL>;
#[doc = "Writer for register LFACLKSEL"]
pub type W = crate::W<u32, super::LFACLKSEL>;
#[doc = "Register LFACLKSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::LFACLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock Select for LFA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LFA_A {
    #[doc = "0: LFACLK is disabled"]
    DISABLED = 0,
    #[doc = "1: LFRCO selected as LFACLK"]
    LFRCO = 1,
    #[doc = "2: LFXO selected as LFACLK"]
    LFXO = 2,
    #[doc = "4: ULFRCO selected as LFACLK"]
    ULFRCO = 4,
}
impl From<LFA_A> for u8 {
    #[inline(always)]
    fn from(variant: LFA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LFA`"]
pub type LFA_R = crate::R<u8, LFA_A>;
impl LFA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LFA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LFA_A::DISABLED),
            1 => Val(LFA_A::LFRCO),
            2 => Val(LFA_A::LFXO),
            4 => Val(LFA_A::ULFRCO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == LFA_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == LFA_A::LFXO
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == LFA_A::ULFRCO
    }
}
#[doc = "Write proxy for field `LFA`"]
pub struct LFA_W<'a> {
    w: &'a mut W,
}
impl<'a> LFA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LFACLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFA_A::DISABLED)
    }
    #[doc = "LFRCO selected as LFACLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(LFA_A::LFRCO)
    }
    #[doc = "LFXO selected as LFACLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(LFA_A::LFXO)
    }
    #[doc = "ULFRCO selected as LFACLK"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(LFA_A::ULFRCO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select for LFA"]
    #[inline(always)]
    pub fn lfa(&self) -> LFA_R {
        LFA_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select for LFA"]
    #[inline(always)]
    pub fn lfa(&mut self) -> LFA_W {
        LFA_W { w: self }
    }
}
