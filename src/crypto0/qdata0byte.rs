#[doc = "Reader of register QDATA0BYTE"]
pub type R = crate::R<u32, super::QDATA0BYTE>;
#[doc = "Writer for register QDATA0BYTE"]
pub type W = crate::W<u32, super::QDATA0BYTE>;
#[doc = "Register QDATA0BYTE `reset()`'s with value 0"]
impl crate::ResetValue for super::QDATA0BYTE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QDATA0BYTE`"]
pub type QDATA0BYTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `QDATA0BYTE`"]
pub struct QDATA0BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> QDATA0BYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Qdata 0 Byte Access"]
    #[inline(always)]
    pub fn qdata0byte(&self) -> QDATA0BYTE_R {
        QDATA0BYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Qdata 0 Byte Access"]
    #[inline(always)]
    pub fn qdata0byte(&mut self) -> QDATA0BYTE_W {
        QDATA0BYTE_W { w: self }
    }
}
