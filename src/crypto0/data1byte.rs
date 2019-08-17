#[doc = "Reader of register DATA1BYTE"]
pub type R = crate::R<u32, super::DATA1BYTE>;
#[doc = "Writer for register DATA1BYTE"]
pub type W = crate::W<u32, super::DATA1BYTE>;
#[doc = "Register DATA1BYTE `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA1BYTE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA1BYTE`"]
pub type DATA1BYTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA1BYTE`"]
pub struct DATA1BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA1BYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data 1 Byte Access"]
    #[inline(always)]
    pub fn data1byte(&self) -> DATA1BYTE_R {
        DATA1BYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 1 Byte Access"]
    #[inline(always)]
    pub fn data1byte(&mut self) -> DATA1BYTE_W {
        DATA1BYTE_W { w: self }
    }
}
