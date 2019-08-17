#[doc = "Reader of register SEGD1H"]
pub type R = crate::R<u32, super::SEGD1H>;
#[doc = "Writer for register SEGD1H"]
pub type W = crate::W<u32, super::SEGD1H>;
#[doc = "Register SEGD1H `reset()`'s with value 0"]
impl crate::ResetValue for super::SEGD1H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEGD1H`"]
pub type SEGD1H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEGD1H`"]
pub struct SEGD1H_W<'a> {
    w: &'a mut W,
}
impl<'a> SEGD1H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - COM1 Segment Data High"]
    #[inline(always)]
    pub fn segd1h(&self) -> SEGD1H_R {
        SEGD1H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM1 Segment Data High"]
    #[inline(always)]
    pub fn segd1h(&mut self) -> SEGD1H_W {
        SEGD1H_W { w: self }
    }
}
