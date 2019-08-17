#[doc = "Reader of register QDATA1BYTE"]
pub type R = crate::R<u32, super::QDATA1BYTE>;
#[doc = "Writer for register QDATA1BYTE"]
pub type W = crate::W<u32, super::QDATA1BYTE>;
#[doc = "Register QDATA1BYTE `reset()`'s with value 0"]
impl crate::ResetValue for super::QDATA1BYTE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QDATA1BYTE`"]
pub type QDATA1BYTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `QDATA1BYTE`"]
pub struct QDATA1BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> QDATA1BYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Qdata 1 Byte Access"]
    #[inline(always)]
    pub fn qdata1byte(&self) -> QDATA1BYTE_R {
        QDATA1BYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Qdata 1 Byte Access"]
    #[inline(always)]
    pub fn qdata1byte(&mut self) -> QDATA1BYTE_W {
        QDATA1BYTE_W { w: self }
    }
}
