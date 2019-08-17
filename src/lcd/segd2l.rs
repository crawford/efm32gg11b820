#[doc = "Reader of register SEGD2L"]
pub type R = crate::R<u32, super::SEGD2L>;
#[doc = "Writer for register SEGD2L"]
pub type W = crate::W<u32, super::SEGD2L>;
#[doc = "Register SEGD2L `reset()`'s with value 0"]
impl crate::ResetValue for super::SEGD2L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEGD2L`"]
pub type SEGD2L_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SEGD2L`"]
pub struct SEGD2L_W<'a> {
    w: &'a mut W,
}
impl<'a> SEGD2L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - COM2 Segment Data Low"]
    #[inline(always)]
    pub fn segd2l(&self) -> SEGD2L_R {
        SEGD2L_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM2 Segment Data Low"]
    #[inline(always)]
    pub fn segd2l(&mut self) -> SEGD2L_W {
        SEGD2L_W { w: self }
    }
}
