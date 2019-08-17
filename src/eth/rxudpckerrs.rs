#[doc = "Reader of register RXUDPCKERRS"]
pub type R = crate::R<u32, super::RXUDPCKERRS>;
#[doc = "Writer for register RXUDPCKERRS"]
pub type W = crate::W<u32, super::RXUDPCKERRS>;
#[doc = "Register RXUDPCKERRS `reset()`'s with value 0"]
impl crate::ResetValue for super::RXUDPCKERRS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COUNT`"]
pub struct COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - UDP checksum errors"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - UDP checksum errors"]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W { w: self }
    }
}
