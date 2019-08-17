#[doc = "Reader of register EMA"]
pub type R = crate::R<u32, super::EMA>;
#[doc = "Writer for register EMA"]
pub type W = crate::W<u32, super::EMA>;
#[doc = "Register EMA `reset()`'s with value 0"]
impl crate::ResetValue for super::EMA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EMA`"]
pub type EMA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EMA`"]
pub struct EMA_W<'a> {
    w: &'a mut W,
}
impl<'a> EMA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u32) & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - Calculated Exponential Moving Average"]
    #[inline(always)]
    pub fn ema(&self) -> EMA_R {
        EMA_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - Calculated Exponential Moving Average"]
    #[inline(always)]
    pub fn ema(&mut self) -> EMA_W {
        EMA_W { w: self }
    }
}
