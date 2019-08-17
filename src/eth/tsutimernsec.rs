#[doc = "Reader of register TSUTIMERNSEC"]
pub type R = crate::R<u32, super::TSUTIMERNSEC>;
#[doc = "Writer for register TSUTIMERNSEC"]
pub type W = crate::W<u32, super::TSUTIMERNSEC>;
#[doc = "Register TSUTIMERNSEC `reset()`'s with value 0"]
impl crate::ResetValue for super::TSUTIMERNSEC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMER`"]
pub type TIMER_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TIMER`"]
pub struct TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | ((value as u32) & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - Timer count in nanoseconds"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - Timer count in nanoseconds"]
    #[inline(always)]
    pub fn timer(&mut self) -> TIMER_W {
        TIMER_W { w: self }
    }
}
