#[doc = "Reader of register DDATA1"]
pub type R = crate::R<u32, super::DDATA1>;
#[doc = "Writer for register DDATA1"]
pub type W = crate::W<u32, super::DDATA1>;
#[doc = "Register DDATA1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDATA1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DDATA1`"]
pub type DDATA1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DDATA1`"]
pub struct DDATA1_W<'a> {
    w: &'a mut W,
}
impl<'a> DDATA1_W<'a> {
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
    pub fn ddata1(&self) -> DDATA1_R {
        DDATA1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    pub fn ddata1(&mut self) -> DDATA1_W {
        DDATA1_W { w: self }
    }
}
