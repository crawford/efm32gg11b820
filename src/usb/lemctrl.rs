#[doc = "Reader of register LEMCTRL"]
pub type R = crate::R<u32, super::LEMCTRL>;
#[doc = "Writer for register LEMCTRL"]
pub type W = crate::W<u32, super::LEMCTRL>;
#[doc = "Register LEMCTRL `reset()`'s with value 0x67"]
impl crate::ResetValue for super::LEMCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x67
    }
}
#[doc = "Reader of field `TIMEBASE`"]
pub type TIMEBASE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TIMEBASE`"]
pub struct TIMEBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEBASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Set the Number of LFC Clk Counts to Form 3ms"]
    #[inline(always)]
    pub fn timebase(&self) -> TIMEBASE_R {
        TIMEBASE_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Set the Number of LFC Clk Counts to Form 3ms"]
    #[inline(always)]
    pub fn timebase(&mut self) -> TIMEBASE_W {
        TIMEBASE_W { w: self }
    }
}
