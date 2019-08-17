#[doc = "Reader of register SEGD0H"]
pub type R = crate::R<u32, super::SEGD0H>;
#[doc = "Writer for register SEGD0H"]
pub type W = crate::W<u32, super::SEGD0H>;
#[doc = "Register SEGD0H `reset()`'s with value 0"]
impl crate::ResetValue for super::SEGD0H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEGD0H`"]
pub type SEGD0H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEGD0H`"]
pub struct SEGD0H_W<'a> {
    w: &'a mut W,
}
impl<'a> SEGD0H_W<'a> {
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
    pub fn segd0h(&self) -> SEGD0H_R {
        SEGD0H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM0 Segment Data High"]
    #[inline(always)]
    pub fn segd0h(&mut self) -> SEGD0H_W {
        SEGD0H_W { w: self }
    }
}
