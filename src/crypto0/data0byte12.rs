#[doc = "Reader of register DATA0BYTE12"]
pub type R = crate::R<u32, super::DATA0BYTE12>;
#[doc = "Writer for register DATA0BYTE12"]
pub type W = crate::W<u32, super::DATA0BYTE12>;
#[doc = "Register DATA0BYTE12 `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA0BYTE12 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA0BYTE12`"]
pub type DATA0BYTE12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA0BYTE12`"]
pub struct DATA0BYTE12_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA0BYTE12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data 0 Byte 12 Access"]
    #[inline(always)]
    pub fn data0byte12(&self) -> DATA0BYTE12_R {
        DATA0BYTE12_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 0 Byte 12 Access"]
    #[inline(always)]
    pub fn data0byte12(&mut self) -> DATA0BYTE12_W {
        DATA0BYTE12_W { w: self }
    }
}
