#[doc = "Reader of register PJ_OVTDIS"]
pub type R = crate::R<u32, super::PJ_OVTDIS>;
#[doc = "Writer for register PJ_OVTDIS"]
pub type W = crate::W<u32, super::PJ_OVTDIS>;
#[doc = "Register PJ_OVTDIS `reset()`'s with value 0"]
impl crate::ResetValue for super::PJ_OVTDIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OVTDIS`"]
pub type OVTDIS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OVTDIS`"]
pub struct OVTDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OVTDIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis(&self) -> OVTDIS_R {
        OVTDIS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Disable Over Voltage Capability"]
    #[inline(always)]
    pub fn ovtdis(&mut self) -> OVTDIS_W {
        OVTDIS_W { w: self }
    }
}
