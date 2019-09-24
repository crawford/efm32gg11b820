#[doc = "Reader of register LFECLKSEL"]
pub type R = crate::R<u32, super::LFECLKSEL>;
#[doc = "Writer for register LFECLKSEL"]
pub type W = crate::W<u32, super::LFECLKSEL>;
#[doc = "Register LFECLKSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::LFECLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock Select for LFE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFE_A {
    #[doc = "0: LFECLK is disabled"]
    DISABLED,
    #[doc = "1: LFRCO selected as LFECLK"]
    LFRCO,
    #[doc = "2: LFXO selected as LFECLK"]
    LFXO,
    #[doc = "4: ULFRCO selected as LFECLK"]
    ULFRCO,
}
impl From<LFE_A> for u8 {
    #[inline(always)]
    fn from(variant: LFE_A) -> Self {
        match variant {
            LFE_A::DISABLED => 0,
            LFE_A::LFRCO => 1,
            LFE_A::LFXO => 2,
            LFE_A::ULFRCO => 4,
        }
    }
}
#[doc = "Reader of field `LFE`"]
pub type LFE_R = crate::R<u8, LFE_A>;
impl LFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LFE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LFE_A::DISABLED),
            1 => Val(LFE_A::LFRCO),
            2 => Val(LFE_A::LFXO),
            4 => Val(LFE_A::ULFRCO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == LFE_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == LFE_A::LFXO
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == LFE_A::ULFRCO
    }
}
#[doc = "Write proxy for field `LFE`"]
pub struct LFE_W<'a> {
    w: &'a mut W,
}
impl<'a> LFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LFECLK is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFE_A::DISABLED)
    }
    #[doc = "LFRCO selected as LFECLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(LFE_A::LFRCO)
    }
    #[doc = "LFXO selected as LFECLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(LFE_A::LFXO)
    }
    #[doc = "ULFRCO selected as LFECLK"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(LFE_A::ULFRCO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Select for LFE"]
    #[inline(always)]
    pub fn lfe(&self) -> LFE_R {
        LFE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Select for LFE"]
    #[inline(always)]
    pub fn lfe(&mut self) -> LFE_W {
        LFE_W { w: self }
    }
}
