#[doc = "Reader of register DDATA0BYTE32"]
pub type R = crate::R<u32, super::DDATA0BYTE32>;
#[doc = "Writer for register DDATA0BYTE32"]
pub type W = crate::W<u32, super::DDATA0BYTE32>;
#[doc = "Register DDATA0BYTE32 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDATA0BYTE32 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DDATA0BYTE32`"]
pub type DDATA0BYTE32_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DDATA0BYTE32`"]
pub struct DDATA0BYTE32_W<'a> {
    w: &'a mut W,
}
impl<'a> DDATA0BYTE32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Ddata 0 Byte 32 Access"]
    #[inline(always)]
    pub fn ddata0byte32(&self) -> DDATA0BYTE32_R {
        DDATA0BYTE32_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Ddata 0 Byte 32 Access"]
    #[inline(always)]
    pub fn ddata0byte32(&mut self) -> DDATA0BYTE32_W {
        DDATA0BYTE32_W { w: self }
    }
}
