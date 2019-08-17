#[doc = "Reader of register SYSWAKETIME"]
pub type R = crate::R<u32, super::SYSWAKETIME>;
#[doc = "Writer for register SYSWAKETIME"]
pub type W = crate::W<u32, super::SYSWAKETIME>;
#[doc = "Register SYSWAKETIME `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSWAKETIME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSWAKETIME`"]
pub type SYSWAKETIME_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SYSWAKETIME`"]
pub struct SYSWAKETIME_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSWAKETIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Count of 64ns, 320ns or 3200ns intervals before transmission starts after deassertion of tx_lpi_en"]
    #[inline(always)]
    pub fn syswaketime(&self) -> SYSWAKETIME_R {
        SYSWAKETIME_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count of 64ns, 320ns or 3200ns intervals before transmission starts after deassertion of tx_lpi_en"]
    #[inline(always)]
    pub fn syswaketime(&mut self) -> SYSWAKETIME_W {
        SYSWAKETIME_W { w: self }
    }
}
