#[doc = "Reader of register LFBCLKSEL"]
pub type R = crate::R<u32, super::LFBCLKSEL>;
#[doc = "Writer for register LFBCLKSEL"]
pub type W = crate::W<u32, super::LFBCLKSEL>;
#[doc = "Register LFBCLKSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::LFBCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock Select for LFB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LFB_A {
    #[doc = "0: LFBCLK is disabled"]
    DISABLED = 0,
    #[doc = "1: LFRCO selected as LFBCLK"]
    LFRCO = 1,
    #[doc = "2: LFXO selected as LFBCLK"]
    LFXO = 2,
    #[doc = "3: HFCLK divided by two/four is selected as LFBCLK"]
    HFCLKLE = 3,
    #[doc = "4: ULFRCO selected as LFBCLK"]
    ULFRCO = 4,
}
impl From<LFB_A> for u8 {
    #[inline(always)]
    fn from(variant: LFB_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LFB`"]
pub type LFB_R = crate::R<u8, LFB_A>;
impl LFB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LFB_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LFB_A::DISABLED),
            1 => Val(LFB_A::LFRCO),
            2 => Val(LFB_A::LFXO),
            3 => Val(LFB_A::HFCLKLE),
            4 => Val(LFB_A::ULFRCO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == LFB_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == LFB_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFCLKLE`"]
    #[inline(always)]
    pub fn is_hfclkle(&self) -> bool {
        *self == LFB_A::HFCLKLE
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == LFB_A::ULFRCO
    }
}
#[doc = "Write proxy for field `LFB`"]
pub struct LFB_W<'a> {
    w: &'a mut W,
}
impl<'a> LFB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFB_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LFBCLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFB_A::DISABLED)
    }
    #[doc = "LFRCO selected as LFBCLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(LFB_A::LFRCO)
    }
    #[doc = "LFXO selected as LFBCLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(LFB_A::LFXO)
    }
    #[doc = "HFCLK divided by two/four is selected as LFBCLK"]
    #[inline(always)]
    pub fn hfclkle(self) -> &'a mut W {
        self.variant(LFB_A::HFCLKLE)
    }
    #[doc = "ULFRCO selected as LFBCLK"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(LFB_A::ULFRCO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select for LFB"]
    #[inline(always)]
    pub fn lfb(&self) -> LFB_R {
        LFB_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select for LFB"]
    #[inline(always)]
    pub fn lfb(&mut self) -> LFB_W {
        LFB_W { w: self }
    }
}
