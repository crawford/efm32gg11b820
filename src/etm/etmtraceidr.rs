#[doc = "Reader of register ETMTRACEIDR"]
pub type R = crate::R<u32, super::ETMTRACEIDR>;
#[doc = "Writer for register ETMTRACEIDR"]
pub type W = crate::W<u32, super::ETMTRACEIDR>;
#[doc = "Register ETMTRACEIDR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETMTRACEIDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRACEID`"]
pub type TRACEID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRACEID`"]
pub struct TRACEID_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACEID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Trace ID"]
    #[inline(always)]
    pub fn traceid(&self) -> TRACEID_R {
        TRACEID_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Trace ID"]
    #[inline(always)]
    pub fn traceid(&mut self) -> TRACEID_W {
        TRACEID_W { w: self }
    }
}
