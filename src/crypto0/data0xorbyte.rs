#[doc = "Reader of register DATA0XORBYTE"]
pub type R = crate::R<u32, super::DATA0XORBYTE>;
#[doc = "Writer for register DATA0XORBYTE"]
pub type W = crate::W<u32, super::DATA0XORBYTE>;
#[doc = "Register DATA0XORBYTE `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA0XORBYTE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA0XORBYTE`"]
pub type DATA0XORBYTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA0XORBYTE`"]
pub struct DATA0XORBYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA0XORBYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data 0 XOR Byte Access"]
    #[inline(always)]
    pub fn data0xorbyte(&self) -> DATA0XORBYTE_R {
        DATA0XORBYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 0 XOR Byte Access"]
    #[inline(always)]
    pub fn data0xorbyte(&mut self) -> DATA0XORBYTE_W {
        DATA0XORBYTE_W { w: self }
    }
}
