#[doc = "Reader of register TFTALPHA"]
pub type R = crate::R<u32, super::TFTALPHA>;
#[doc = "Writer for register TFTALPHA"]
pub type W = crate::W<u32, super::TFTALPHA>;
#[doc = "Register TFTALPHA `reset()`'s with value 0"]
impl crate::ResetValue for super::TFTALPHA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ALPHA`"]
pub type ALPHA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ALPHA`"]
pub struct ALPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> ALPHA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - TFT Alpha Blending Factor"]
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - TFT Alpha Blending Factor"]
    #[inline(always)]
    pub fn alpha(&mut self) -> ALPHA_W {
        ALPHA_W { w: self }
    }
}
