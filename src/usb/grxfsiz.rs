#[doc = "Reader of register GRXFSIZ"]
pub type R = crate::R<u32, super::GRXFSIZ>;
#[doc = "Writer for register GRXFSIZ"]
pub type W = crate::W<u32, super::GRXFSIZ>;
#[doc = "Register GRXFSIZ `reset()`'s with value 0x0200"]
impl crate::ResetValue for super::GRXFSIZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200
    }
}
#[doc = "Reader of field `RXFDEP`"]
pub type RXFDEP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RXFDEP`"]
pub struct RXFDEP_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFDEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - RxFIFO Depth"]
    #[inline(always)]
    pub fn rxfdep(&self) -> RXFDEP_R {
        RXFDEP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - RxFIFO Depth"]
    #[inline(always)]
    pub fn rxfdep(&mut self) -> RXFDEP_W {
        RXFDEP_W { w: self }
    }
}
