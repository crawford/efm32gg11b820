#[doc = "Reader of register SEGD3L"]
pub type R = crate::R<u32, super::SEGD3L>;
#[doc = "Writer for register SEGD3L"]
pub type W = crate::W<u32, super::SEGD3L>;
#[doc = "Register SEGD3L `reset()`'s with value 0"]
impl crate::ResetValue for super::SEGD3L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEGD3L`"]
pub type SEGD3L_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SEGD3L`"]
pub struct SEGD3L_W<'a> {
    w: &'a mut W,
}
impl<'a> SEGD3L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - COM3 Segment Data Low"]
    #[inline(always)]
    pub fn segd3l(&self) -> SEGD3L_R {
        SEGD3L_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM3 Segment Data Low"]
    #[inline(always)]
    pub fn segd3l(&mut self) -> SEGD3L_W {
        SEGD3L_W { w: self }
    }
}
