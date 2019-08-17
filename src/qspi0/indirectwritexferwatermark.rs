#[doc = "Reader of register INDIRECTWRITEXFERWATERMARK"]
pub type R = crate::R<u32, super::INDIRECTWRITEXFERWATERMARK>;
#[doc = "Writer for register INDIRECTWRITEXFERWATERMARK"]
pub type W = crate::W<u32, super::INDIRECTWRITEXFERWATERMARK>;
#[doc = "Register INDIRECTWRITEXFERWATERMARK `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::INDIRECTWRITEXFERWATERMARK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `LEVEL`"]
pub type LEVEL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LEVEL`"]
pub struct LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Watermark Value"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Watermark Value"]
    #[inline(always)]
    pub fn level(&mut self) -> LEVEL_W {
        LEVEL_W { w: self }
    }
}
