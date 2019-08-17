#[doc = "Reader of register SCANMASK0"]
pub type R = crate::R<u32, super::SCANMASK0>;
#[doc = "Writer for register SCANMASK0"]
pub type W = crate::W<u32, super::SCANMASK0>;
#[doc = "Register SCANMASK0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCANMASK0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCANINPUTEN`"]
pub type SCANINPUTEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SCANINPUTEN`"]
pub struct SCANINPUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANINPUTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Scan Channel Mask"]
    #[inline(always)]
    pub fn scaninputen(&self) -> SCANINPUTEN_R {
        SCANINPUTEN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Scan Channel Mask"]
    #[inline(always)]
    pub fn scaninputen(&mut self) -> SCANINPUTEN_W {
        SCANINPUTEN_W { w: self }
    }
}
