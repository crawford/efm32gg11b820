#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ITAPDLYEN`"]
pub type ITAPDLYEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ITAPDLYEN`"]
pub struct ITAPDLYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAPDLYEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ITAPDLYSEL`"]
pub type ITAPDLYSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ITAPDLYSEL`"]
pub struct ITAPDLYSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAPDLYSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | (((value as u32) & 0x1f) << 1);
        self.w
    }
}
#[doc = "Reader of field `ITAPCHGWIN`"]
pub type ITAPCHGWIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ITAPCHGWIN`"]
pub struct ITAPCHGWIN_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAPCHGWIN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `OTAPDLYEN`"]
pub type OTAPDLYEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTAPDLYEN`"]
pub struct OTAPDLYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OTAPDLYEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `OTAPDLYSEL`"]
pub type OTAPDLYSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OTAPDLYSEL`"]
pub struct OTAPDLYSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OTAPDLYSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TXDLYMUXSEL`"]
pub type TXDLYMUXSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXDLYMUXSEL`"]
pub struct TXDLYMUXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDLYMUXSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Selective Tap Delay Line Enable on Rxclk_in"]
    #[inline(always)]
    pub fn itapdlyen(&self) -> ITAPDLYEN_R {
        ITAPDLYEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:5 - Selects One of 32 Taps on the Rxclk_in Line"]
    #[inline(always)]
    pub fn itapdlysel(&self) -> ITAPDLYSEL_R {
        ITAPDLYSEL_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Gating Signal for Tap Delay Change"]
    #[inline(always)]
    pub fn itapchgwin(&self) -> ITAPCHGWIN_R {
        ITAPCHGWIN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Selective Tap Delay Line Enable on SDIO_CLK Pin"]
    #[inline(always)]
    pub fn otapdlyen(&self) -> OTAPDLYEN_R {
        OTAPDLYEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Selects One of 32 Taps on the SDIO_CLK Pin"]
    #[inline(always)]
    pub fn otapdlysel(&self) -> OTAPDLYSEL_R {
        OTAPDLYSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - TX Delay Mux Selection"]
    #[inline(always)]
    pub fn txdlymuxsel(&self) -> TXDLYMUXSEL_R {
        TXDLYMUXSEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Selective Tap Delay Line Enable on Rxclk_in"]
    #[inline(always)]
    pub fn itapdlyen(&mut self) -> ITAPDLYEN_W {
        ITAPDLYEN_W { w: self }
    }
    #[doc = "Bits 1:5 - Selects One of 32 Taps on the Rxclk_in Line"]
    #[inline(always)]
    pub fn itapdlysel(&mut self) -> ITAPDLYSEL_W {
        ITAPDLYSEL_W { w: self }
    }
    #[doc = "Bit 6 - Gating Signal for Tap Delay Change"]
    #[inline(always)]
    pub fn itapchgwin(&mut self) -> ITAPCHGWIN_W {
        ITAPCHGWIN_W { w: self }
    }
    #[doc = "Bit 7 - Selective Tap Delay Line Enable on SDIO_CLK Pin"]
    #[inline(always)]
    pub fn otapdlyen(&mut self) -> OTAPDLYEN_W {
        OTAPDLYEN_W { w: self }
    }
    #[doc = "Bits 8:11 - Selects One of 32 Taps on the SDIO_CLK Pin"]
    #[inline(always)]
    pub fn otapdlysel(&mut self) -> OTAPDLYSEL_W {
        OTAPDLYSEL_W { w: self }
    }
    #[doc = "Bits 16:17 - TX Delay Mux Selection"]
    #[inline(always)]
    pub fn txdlymuxsel(&mut self) -> TXDLYMUXSEL_W {
        TXDLYMUXSEL_W { w: self }
    }
}
