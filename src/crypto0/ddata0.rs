#[doc = "Reader of register DDATA0"]
pub type R = crate::R<u32, super::DDATA0>;
#[doc = "Writer for register DDATA0"]
pub type W = crate::W<u32, super::DDATA0>;
#[doc = "Register DDATA0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDATA0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DDATA0`"]
pub type DDATA0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DDATA0`"]
pub struct DDATA0_W<'a> {
    w: &'a mut W,
}
impl<'a> DDATA0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    pub fn ddata0(&self) -> DDATA0_R {
        DDATA0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    pub fn ddata0(&mut self) -> DDATA0_W {
        DDATA0_W { w: self }
    }
}
