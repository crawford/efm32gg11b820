#[doc = "Reader of register QDATA0"]
pub type R = crate::R<u32, super::QDATA0>;
#[doc = "Writer for register QDATA0"]
pub type W = crate::W<u32, super::QDATA0>;
#[doc = "Register QDATA0 `reset()`'s with value 0"]
impl crate::ResetValue for super::QDATA0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QDATA0`"]
pub type QDATA0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `QDATA0`"]
pub struct QDATA0_W<'a> {
    w: &'a mut W,
}
impl<'a> QDATA0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Quad Data 0 Access"]
    #[inline(always)]
    pub fn qdata0(&self) -> QDATA0_R {
        QDATA0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Quad Data 0 Access"]
    #[inline(always)]
    pub fn qdata0(&mut self) -> QDATA0_W {
        QDATA0_W { w: self }
    }
}
