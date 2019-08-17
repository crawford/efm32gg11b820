#[doc = "Reader of register TFTTIMING"]
pub type R = crate::R<u32, super::TFTTIMING>;
#[doc = "Writer for register TFTTIMING"]
pub type W = crate::W<u32, super::TFTTIMING>;
#[doc = "Register TFTTIMING `reset()`'s with value 0"]
impl crate::ResetValue for super::TFTTIMING {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCLKPERIOD`"]
pub type DCLKPERIOD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DCLKPERIOD`"]
pub struct DCLKPERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DCLKPERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `TFTSTART`"]
pub type TFTSTART_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TFTSTART`"]
pub struct TFTSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> TFTSTART_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 12)) | (((value as u32) & 0x0fff) << 12);
        self.w
    }
}
#[doc = "Reader of field `TFTSETUP`"]
pub type TFTSETUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TFTSETUP`"]
pub struct TFTSETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> TFTSETUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `TFTHOLD`"]
pub type TFTHOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TFTHOLD`"]
pub struct TFTHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TFTHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - TFT Direct Drive Transaction (EBI_DCLK) Period"]
    #[inline(always)]
    pub fn dclkperiod(&self) -> DCLKPERIOD_R {
        DCLKPERIOD_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - TFT Direct Drive Transaction Start"]
    #[inline(always)]
    pub fn tftstart(&self) -> TFTSTART_R {
        TFTSTART_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bits 24:26 - TFT Setup Time"]
    #[inline(always)]
    pub fn tftsetup(&self) -> TFTSETUP_R {
        TFTSETUP_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - TFT Hold Time"]
    #[inline(always)]
    pub fn tfthold(&self) -> TFTHOLD_R {
        TFTHOLD_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - TFT Direct Drive Transaction (EBI_DCLK) Period"]
    #[inline(always)]
    pub fn dclkperiod(&mut self) -> DCLKPERIOD_W {
        DCLKPERIOD_W { w: self }
    }
    #[doc = "Bits 12:23 - TFT Direct Drive Transaction Start"]
    #[inline(always)]
    pub fn tftstart(&mut self) -> TFTSTART_W {
        TFTSTART_W { w: self }
    }
    #[doc = "Bits 24:26 - TFT Setup Time"]
    #[inline(always)]
    pub fn tftsetup(&mut self) -> TFTSETUP_W {
        TFTSETUP_W { w: self }
    }
    #[doc = "Bits 28:30 - TFT Hold Time"]
    #[inline(always)]
    pub fn tfthold(&mut self) -> TFTHOLD_W {
        TFTHOLD_W { w: self }
    }
}
