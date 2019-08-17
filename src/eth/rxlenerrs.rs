#[doc = "Reader of register RXLENERRS"]
pub type R = crate::R<u32, super::RXLENERRS>;
#[doc = "Writer for register RXLENERRS"]
pub type W = crate::W<u32, super::RXLENERRS>;
#[doc = "Register RXLENERRS `reset()`'s with value 0"]
impl crate::ResetValue for super::RXLENERRS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COUNT`"]
pub struct COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Length field frame errors"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Length field frame errors"]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W { w: self }
    }
}
