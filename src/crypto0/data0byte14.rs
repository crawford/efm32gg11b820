#[doc = "Reader of register DATA0BYTE14"]
pub type R = crate::R<u32, super::DATA0BYTE14>;
#[doc = "Writer for register DATA0BYTE14"]
pub type W = crate::W<u32, super::DATA0BYTE14>;
#[doc = "Register DATA0BYTE14 `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA0BYTE14 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA0BYTE14`"]
pub type DATA0BYTE14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA0BYTE14`"]
pub struct DATA0BYTE14_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA0BYTE14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data 0 Byte 14 Access"]
    #[inline(always)]
    pub fn data0byte14(&self) -> DATA0BYTE14_R {
        DATA0BYTE14_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 0 Byte 14 Access"]
    #[inline(always)]
    pub fn data0byte14(&mut self) -> DATA0BYTE14_W {
        DATA0BYTE14_W { w: self }
    }
}
