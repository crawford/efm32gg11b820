#[doc = "Reader of register SEGD4L"]
pub type R = crate::R<u32, super::SEGD4L>;
#[doc = "Writer for register SEGD4L"]
pub type W = crate::W<u32, super::SEGD4L>;
#[doc = "Register SEGD4L `reset()`'s with value 0"]
impl crate::ResetValue for super::SEGD4L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEGD4L`"]
pub type SEGD4L_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SEGD4L`"]
pub struct SEGD4L_W<'a> {
    w: &'a mut W,
}
impl<'a> SEGD4L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - COM4 Segment Data"]
    #[inline(always)]
    pub fn segd4l(&self) -> SEGD4L_R {
        SEGD4L_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM4 Segment Data"]
    #[inline(always)]
    pub fn segd4l(&mut self) -> SEGD4L_W {
        SEGD4L_W { w: self }
    }
}
