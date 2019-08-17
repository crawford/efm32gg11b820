#[doc = "Reader of register CDCONF"]
pub type R = crate::R<u32, super::CDCONF>;
#[doc = "Writer for register CDCONF"]
pub type W = crate::W<u32, super::CDCONF>;
#[doc = "Register CDCONF `reset()`'s with value 0"]
impl crate::ResetValue for super::CDCONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCDTOCONF`"]
pub type DCDTOCONF_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DCDTOCONF`"]
pub struct DCDTOCONF_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDTOCONF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - DCD Timeout (TDCD_TIMEOUT) Configuration"]
    #[inline(always)]
    pub fn dcdtoconf(&self) -> DCDTOCONF_R {
        DCDTOCONF_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DCD Timeout (TDCD_TIMEOUT) Configuration"]
    #[inline(always)]
    pub fn dcdtoconf(&mut self) -> DCDTOCONF_W {
        DCDTOCONF_W { w: self }
    }
}
