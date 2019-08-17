#[doc = "Reader of register SEGD7L"]
pub type R = crate::R<u32, super::SEGD7L>;
#[doc = "Writer for register SEGD7L"]
pub type W = crate::W<u32, super::SEGD7L>;
#[doc = "Register SEGD7L `reset()`'s with value 0"]
impl crate::ResetValue for super::SEGD7L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEGD7L`"]
pub type SEGD7L_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SEGD7L`"]
pub struct SEGD7L_W<'a> {
    w: &'a mut W,
}
impl<'a> SEGD7L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - COM7 Segment Data"]
    #[inline(always)]
    pub fn segd7l(&self) -> SEGD7L_R {
        SEGD7L_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM7 Segment Data"]
    #[inline(always)]
    pub fn segd7l(&mut self) -> SEGD7L_W {
        SEGD7L_W { w: self }
    }
}
