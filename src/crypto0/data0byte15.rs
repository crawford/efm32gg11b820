#[doc = "Reader of register DATA0BYTE15"]
pub type R = crate::R<u32, super::DATA0BYTE15>;
#[doc = "Writer for register DATA0BYTE15"]
pub type W = crate::W<u32, super::DATA0BYTE15>;
#[doc = "Register DATA0BYTE15 `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA0BYTE15 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA0BYTE15`"]
pub type DATA0BYTE15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA0BYTE15`"]
pub struct DATA0BYTE15_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA0BYTE15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data 0 Byte 15 Access"]
    #[inline(always)]
    pub fn data0byte15(&self) -> DATA0BYTE15_R {
        DATA0BYTE15_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 0 Byte 15 Access"]
    #[inline(always)]
    pub fn data0byte15(&mut self) -> DATA0BYTE15_W {
        DATA0BYTE15_W { w: self }
    }
}
