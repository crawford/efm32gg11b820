#[doc = "Reader of register ETMSYNCFR"]
pub type R = crate::R<u32, super::ETMSYNCFR>;
#[doc = "Writer for register ETMSYNCFR"]
pub type W = crate::W<u32, super::ETMSYNCFR>;
#[doc = "Register ETMSYNCFR `reset()`'s with value 0x0400"]
impl crate::ResetValue for super::ETMSYNCFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0400
    }
}
#[doc = "Reader of field `FREQ`"]
pub type FREQ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FREQ`"]
pub struct FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Synchronisation Frequency Value"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Synchronisation Frequency Value"]
    #[inline(always)]
    pub fn freq(&mut self) -> FREQ_W {
        FREQ_W { w: self }
    }
}
