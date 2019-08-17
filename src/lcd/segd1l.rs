#[doc = "Reader of register SEGD1L"]
pub type R = crate::R<u32, super::SEGD1L>;
#[doc = "Writer for register SEGD1L"]
pub type W = crate::W<u32, super::SEGD1L>;
#[doc = "Register SEGD1L `reset()`'s with value 0"]
impl crate::ResetValue for super::SEGD1L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEGD1L`"]
pub type SEGD1L_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SEGD1L`"]
pub struct SEGD1L_W<'a> {
    w: &'a mut W,
}
impl<'a> SEGD1L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - COM1 Segment Data Low"]
    #[inline(always)]
    pub fn segd1l(&self) -> SEGD1L_R {
        SEGD1L_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM1 Segment Data Low"]
    #[inline(always)]
    pub fn segd1l(&mut self) -> SEGD1L_W {
        SEGD1L_W { w: self }
    }
}
