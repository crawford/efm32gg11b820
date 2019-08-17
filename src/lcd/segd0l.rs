#[doc = "Reader of register SEGD0L"]
pub type R = crate::R<u32, super::SEGD0L>;
#[doc = "Writer for register SEGD0L"]
pub type W = crate::W<u32, super::SEGD0L>;
#[doc = "Register SEGD0L `reset()`'s with value 0"]
impl crate::ResetValue for super::SEGD0L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEGD0L`"]
pub type SEGD0L_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SEGD0L`"]
pub struct SEGD0L_W<'a> {
    w: &'a mut W,
}
impl<'a> SEGD0L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - COM0 Segment Data Low"]
    #[inline(always)]
    pub fn segd0l(&self) -> SEGD0L_R {
        SEGD0L_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - COM0 Segment Data Low"]
    #[inline(always)]
    pub fn segd0l(&mut self) -> SEGD0L_W {
        SEGD0L_W { w: self }
    }
}
