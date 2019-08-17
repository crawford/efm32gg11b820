#[doc = "Reader of register RXLPITIME"]
pub type R = crate::R<u32, super::RXLPITIME>;
#[doc = "Writer for register RXLPITIME"]
pub type W = crate::W<u32, super::RXLPITIME>;
#[doc = "Register RXLPITIME `reset()`'s with value 0"]
impl crate::ResetValue for super::RXLPITIME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LPITIME`"]
pub type LPITIME_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LPITIME`"]
pub struct LPITIME_W<'a> {
    w: &'a mut W,
}
impl<'a> LPITIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Time in LPI"]
    #[inline(always)]
    pub fn lpitime(&self) -> LPITIME_R {
        LPITIME_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Time in LPI"]
    #[inline(always)]
    pub fn lpitime(&mut self) -> LPITIME_W {
        LPITIME_W { w: self }
    }
}
