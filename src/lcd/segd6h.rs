#[doc = "Reader of register SEGD6H"]
pub type R = crate::R<u32, super::SEGD6H>;
#[doc = "Writer for register SEGD6H"]
pub type W = crate::W<u32, super::SEGD6H>;
#[doc = "Register SEGD6H `reset()`'s with value 0"]
impl crate::ResetValue for super::SEGD6H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEGD6H`"]
pub type SEGD6H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEGD6H`"]
pub struct SEGD6H_W<'a> {
    w: &'a mut W,
}
impl<'a> SEGD6H_W<'a> {
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
    pub fn segd6h(&self) -> SEGD6H_R {
        SEGD6H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM2 Segment Data High"]
    #[inline(always)]
    pub fn segd6h(&mut self) -> SEGD6H_W {
        SEGD6H_W { w: self }
    }
}
