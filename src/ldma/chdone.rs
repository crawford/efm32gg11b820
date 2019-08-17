#[doc = "Reader of register CHDONE"]
pub type R = crate::R<u32, super::CHDONE>;
#[doc = "Writer for register CHDONE"]
pub type W = crate::W<u32, super::CHDONE>;
#[doc = "Register CHDONE `reset()`'s with value 0"]
impl crate::ResetValue for super::CHDONE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHDONE`"]
pub type CHDONE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CHDONE`"]
pub struct CHDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHDONE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - DMA Channel Linking or Done"]
    #[inline(always)]
    pub fn chdone(&self) -> CHDONE_R {
        CHDONE_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - DMA Channel Linking or Done"]
    #[inline(always)]
    pub fn chdone(&mut self) -> CHDONE_W {
        CHDONE_W { w: self }
    }
}
