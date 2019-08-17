#[doc = "Reader of register SEGD2H"]
pub type R = crate::R<u32, super::SEGD2H>;
#[doc = "Writer for register SEGD2H"]
pub type W = crate::W<u32, super::SEGD2H>;
#[doc = "Register SEGD2H `reset()`'s with value 0"]
impl crate::ResetValue for super::SEGD2H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEGD2H`"]
pub type SEGD2H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEGD2H`"]
pub struct SEGD2H_W<'a> {
    w: &'a mut W,
}
impl<'a> SEGD2H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - COM2 Segment Data High"]
    #[inline(always)]
    pub fn segd2h(&self) -> SEGD2H_R {
        SEGD2H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM2 Segment Data High"]
    #[inline(always)]
    pub fn segd2h(&mut self) -> SEGD2H_W {
        SEGD2H_W { w: self }
    }
}
