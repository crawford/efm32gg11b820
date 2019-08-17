#[doc = "Reader of register DATA0BYTE"]
pub type R = crate::R<u32, super::DATA0BYTE>;
#[doc = "Writer for register DATA0BYTE"]
pub type W = crate::W<u32, super::DATA0BYTE>;
#[doc = "Register DATA0BYTE `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA0BYTE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA0BYTE`"]
pub type DATA0BYTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA0BYTE`"]
pub struct DATA0BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA0BYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data 0 Byte Access"]
    #[inline(always)]
    pub fn data0byte(&self) -> DATA0BYTE_R {
        DATA0BYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 0 Byte Access"]
    #[inline(always)]
    pub fn data0byte(&mut self) -> DATA0BYTE_W {
        DATA0BYTE_W { w: self }
    }
}
