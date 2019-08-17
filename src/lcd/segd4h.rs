#[doc = "Reader of register SEGD4H"]
pub type R = crate::R<u32, super::SEGD4H>;
#[doc = "Writer for register SEGD4H"]
pub type W = crate::W<u32, super::SEGD4H>;
#[doc = "Register SEGD4H `reset()`'s with value 0"]
impl crate::ResetValue for super::SEGD4H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEGD4H`"]
pub type SEGD4H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEGD4H`"]
pub struct SEGD4H_W<'a> {
    w: &'a mut W,
}
impl<'a> SEGD4H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - COM0 Segment Data High"]
    #[inline(always)]
    pub fn segd4h(&self) -> SEGD4H_R {
        SEGD4H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM0 Segment Data High"]
    #[inline(always)]
    pub fn segd4h(&mut self) -> SEGD4H_W {
        SEGD4H_W { w: self }
    }
}
