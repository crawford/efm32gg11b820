#[doc = "Reader of register SEGD3H"]
pub type R = crate::R<u32, super::SEGD3H>;
#[doc = "Writer for register SEGD3H"]
pub type W = crate::W<u32, super::SEGD3H>;
#[doc = "Register SEGD3H `reset()`'s with value 0"]
impl crate::ResetValue for super::SEGD3H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEGD3H`"]
pub type SEGD3H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEGD3H`"]
pub struct SEGD3H_W<'a> {
    w: &'a mut W,
}
impl<'a> SEGD3H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - COM3 Segment Data High"]
    #[inline(always)]
    pub fn segd3h(&self) -> SEGD3H_R {
        SEGD3H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM3 Segment Data High"]
    #[inline(always)]
    pub fn segd3h(&mut self) -> SEGD3H_W {
        SEGD3H_W { w: self }
    }
}
