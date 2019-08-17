#[doc = "Reader of register SEGD5L"]
pub type R = crate::R<u32, super::SEGD5L>;
#[doc = "Writer for register SEGD5L"]
pub type W = crate::W<u32, super::SEGD5L>;
#[doc = "Register SEGD5L `reset()`'s with value 0"]
impl crate::ResetValue for super::SEGD5L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEGD5L`"]
pub type SEGD5L_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SEGD5L`"]
pub struct SEGD5L_W<'a> {
    w: &'a mut W,
}
impl<'a> SEGD5L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - COM5 Segment Data"]
    #[inline(always)]
    pub fn segd5l(&self) -> SEGD5L_R {
        SEGD5L_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM5 Segment Data"]
    #[inline(always)]
    pub fn segd5l(&mut self) -> SEGD5L_W {
        SEGD5L_W { w: self }
    }
}
