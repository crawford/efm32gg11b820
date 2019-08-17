#[doc = "Reader of register DDATA1BYTE"]
pub type R = crate::R<u32, super::DDATA1BYTE>;
#[doc = "Writer for register DDATA1BYTE"]
pub type W = crate::W<u32, super::DDATA1BYTE>;
#[doc = "Register DDATA1BYTE `reset()`'s with value 0"]
impl crate::ResetValue for super::DDATA1BYTE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DDATA1BYTE`"]
pub type DDATA1BYTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DDATA1BYTE`"]
pub struct DDATA1BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> DDATA1BYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Ddata 1 Byte Access"]
    #[inline(always)]
    pub fn ddata1byte(&self) -> DDATA1BYTE_R {
        DDATA1BYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Ddata 1 Byte Access"]
    #[inline(always)]
    pub fn ddata1byte(&mut self) -> DDATA1BYTE_W {
        DDATA1BYTE_W { w: self }
    }
}
