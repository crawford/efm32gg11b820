#[doc = "Reader of register DDATA0BYTE"]
pub type R = crate::R<u32, super::DDATA0BYTE>;
#[doc = "Writer for register DDATA0BYTE"]
pub type W = crate::W<u32, super::DDATA0BYTE>;
#[doc = "Register DDATA0BYTE `reset()`'s with value 0"]
impl crate::ResetValue for super::DDATA0BYTE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DDATA0BYTE`"]
pub type DDATA0BYTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DDATA0BYTE`"]
pub struct DDATA0BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> DDATA0BYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Ddata 0 Byte Access"]
    #[inline(always)]
    pub fn ddata0byte(&self) -> DDATA0BYTE_R {
        DDATA0BYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Ddata 0 Byte Access"]
    #[inline(always)]
    pub fn ddata0byte(&mut self) -> DDATA0BYTE_W {
        DDATA0BYTE_W { w: self }
    }
}
