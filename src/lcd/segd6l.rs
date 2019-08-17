#[doc = "Reader of register SEGD6L"]
pub type R = crate::R<u32, super::SEGD6L>;
#[doc = "Writer for register SEGD6L"]
pub type W = crate::W<u32, super::SEGD6L>;
#[doc = "Register SEGD6L `reset()`'s with value 0"]
impl crate::ResetValue for super::SEGD6L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEGD6L`"]
pub type SEGD6L_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SEGD6L`"]
pub struct SEGD6L_W<'a> {
    w: &'a mut W,
}
impl<'a> SEGD6L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - COM6 Segment Data"]
    #[inline(always)]
    pub fn segd6l(&self) -> SEGD6L_R {
        SEGD6L_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM6 Segment Data"]
    #[inline(always)]
    pub fn segd6l(&mut self) -> SEGD6L_W {
        SEGD6L_W { w: self }
    }
}
