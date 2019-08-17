#[doc = "Reader of register SEGD5H"]
pub type R = crate::R<u32, super::SEGD5H>;
#[doc = "Writer for register SEGD5H"]
pub type W = crate::W<u32, super::SEGD5H>;
#[doc = "Register SEGD5H `reset()`'s with value 0"]
impl crate::ResetValue for super::SEGD5H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEGD5H`"]
pub type SEGD5H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEGD5H`"]
pub struct SEGD5H_W<'a> {
    w: &'a mut W,
}
impl<'a> SEGD5H_W<'a> {
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
    pub fn segd5h(&self) -> SEGD5H_R {
        SEGD5H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM1 Segment Data High"]
    #[inline(always)]
    pub fn segd5h(&mut self) -> SEGD5H_W {
        SEGD5H_W { w: self }
    }
}
