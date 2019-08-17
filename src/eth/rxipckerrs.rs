#[doc = "Reader of register RXIPCKERRS"]
pub type R = crate::R<u32, super::RXIPCKERRS>;
#[doc = "Writer for register RXIPCKERRS"]
pub type W = crate::W<u32, super::RXIPCKERRS>;
#[doc = "Register RXIPCKERRS `reset()`'s with value 0"]
impl crate::ResetValue for super::RXIPCKERRS {
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
    #[doc = "Bits 0:7 - IP header checksum errors"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IP header checksum errors"]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W { w: self }
    }
}
