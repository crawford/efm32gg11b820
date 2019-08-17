#[doc = "Writer for register SWREQ"]
pub type W = crate::W<u32, super::SWREQ>;
#[doc = "Register SWREQ `reset()`'s with value 0"]
impl crate::ResetValue for super::SWREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SWREQ`"]
pub struct SWREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:23 - Software Transfer Requests"]
    #[inline(always)]
    pub fn swreq(&mut self) -> SWREQ_W {
        SWREQ_W { w: self }
    }
}
