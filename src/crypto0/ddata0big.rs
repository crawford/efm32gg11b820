#[doc = "Reader of register DDATA0BIG"]
pub type R = crate::R<u32, super::DDATA0BIG>;
#[doc = "Writer for register DDATA0BIG"]
pub type W = crate::W<u32, super::DDATA0BIG>;
#[doc = "Register DDATA0BIG `reset()`'s with value 0"]
impl crate::ResetValue for super::DDATA0BIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DDATA0BIG`"]
pub type DDATA0BIG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DDATA0BIG`"]
pub struct DDATA0BIG_W<'a> {
    w: &'a mut W,
}
impl<'a> DDATA0BIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Double Data 0 Big Endian Access"]
    #[inline(always)]
    pub fn ddata0big(&self) -> DDATA0BIG_R {
        DDATA0BIG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Double Data 0 Big Endian Access"]
    #[inline(always)]
    pub fn ddata0big(&mut self) -> DDATA0BIG_W {
        DDATA0BIG_W { w: self }
    }
}
