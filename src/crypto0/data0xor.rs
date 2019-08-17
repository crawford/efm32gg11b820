#[doc = "Reader of register DATA0XOR"]
pub type R = crate::R<u32, super::DATA0XOR>;
#[doc = "Writer for register DATA0XOR"]
pub type W = crate::W<u32, super::DATA0XOR>;
#[doc = "Register DATA0XOR `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA0XOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA0XOR`"]
pub type DATA0XOR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DATA0XOR`"]
pub struct DATA0XOR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA0XOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - XOR Data 0 Access"]
    #[inline(always)]
    pub fn data0xor(&self) -> DATA0XOR_R {
        DATA0XOR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - XOR Data 0 Access"]
    #[inline(always)]
    pub fn data0xor(&mut self) -> DATA0XOR_W {
        DATA0XOR_W { w: self }
    }
}
