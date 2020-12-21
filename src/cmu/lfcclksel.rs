#[doc = "Reader of register LFCCLKSEL"]
pub type R = crate::R<u32, super::LFCCLKSEL>;
#[doc = "Writer for register LFCCLKSEL"]
pub type W = crate::W<u32, super::LFCCLKSEL>;
#[doc = "Register LFCCLKSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::LFCCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock Select for LFC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LFC_A {
    #[doc = "0: LFCCLK is disabled"]
    DISABLED = 0,
    #[doc = "1: LFRCO selected as LFCCLK"]
    LFRCO = 1,
    #[doc = "2: LFXO selected as LFCCLK"]
    LFXO = 2,
    #[doc = "4: ULFRCO selected as LFCCLK"]
    ULFRCO = 4,
}
impl From<LFC_A> for u8 {
    #[inline(always)]
    fn from(variant: LFC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LFC`"]
pub type LFC_R = crate::R<u8, LFC_A>;
impl LFC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LFC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LFC_A::DISABLED),
            1 => Val(LFC_A::LFRCO),
            2 => Val(LFC_A::LFXO),
            4 => Val(LFC_A::ULFRCO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == LFC_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == LFC_A::LFXO
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == LFC_A::ULFRCO
    }
}
#[doc = "Write proxy for field `LFC`"]
pub struct LFC_W<'a> {
    w: &'a mut W,
}
impl<'a> LFC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LFCCLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFC_A::DISABLED)
    }
    #[doc = "LFRCO selected as LFCCLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(LFC_A::LFRCO)
    }
    #[doc = "LFXO selected as LFCCLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(LFC_A::LFXO)
    }
    #[doc = "ULFRCO selected as LFCCLK"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(LFC_A::ULFRCO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select for LFC"]
    #[inline(always)]
    pub fn lfc(&self) -> LFC_R {
        LFC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select for LFC"]
    #[inline(always)]
    pub fn lfc(&mut self) -> LFC_W {
        LFC_W { w: self }
    }
}
