#[doc = "Reader of register CFGPRESETVAL2"]
pub type R = crate::R<u32, super::CFGPRESETVAL2>;
#[doc = "Writer for register CFGPRESETVAL2"]
pub type W = crate::W<u32, super::CFGPRESETVAL2>;
#[doc = "Register CFGPRESETVAL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGPRESETVAL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDR25SDCLKFREQ`"]
pub type SDR25SDCLKFREQ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SDR25SDCLKFREQ`"]
pub struct SDR25SDCLKFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SDR25SDCLKFREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `SDR25CLKGENEN`"]
pub type SDR25CLKGENEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDR25CLKGENEN`"]
pub struct SDR25CLKGENEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDR25CLKGENEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SDR25DRVST`"]
pub type SDR25DRVST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDR25DRVST`"]
pub struct SDR25DRVST_W<'a> {
    w: &'a mut W,
}
impl<'a> SDR25DRVST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `SDR50SDCLKFREQ`"]
pub type SDR50SDCLKFREQ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SDR50SDCLKFREQ`"]
pub struct SDR50SDCLKFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SDR50SDCLKFREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SDR50CLKGENEN`"]
pub type SDR50CLKGENEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDR50CLKGENEN`"]
pub struct SDR50CLKGENEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDR50CLKGENEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `SDR50DRVST`"]
pub type SDR50DRVST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDR50DRVST`"]
pub struct SDR50DRVST_W<'a> {
    w: &'a mut W,
}
impl<'a> SDR50DRVST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - SDR25 SD_CLK Frequency"]
    #[inline(always)]
    pub fn sdr25sdclkfreq(&self) -> SDR25SDCLKFREQ_R {
        SDR25SDCLKFREQ_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - SDR25 SD_CLK Gen Enable"]
    #[inline(always)]
    pub fn sdr25clkgenen(&self) -> SDR25CLKGENEN_R {
        SDR25CLKGENEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - SDR25 SD Drive Strength"]
    #[inline(always)]
    pub fn sdr25drvst(&self) -> SDR25DRVST_R {
        SDR25DRVST_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 16:25 - Preset Value for SDR50 Speed of SD_CLK"]
    #[inline(always)]
    pub fn sdr50sdclkfreq(&self) -> SDR50SDCLKFREQ_R {
        SDR50SDCLKFREQ_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - SDR50 Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn sdr50clkgenen(&self) -> SDR50CLKGENEN_R {
        SDR50CLKGENEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:28 - SDR50 Speed Drive Strength"]
    #[inline(always)]
    pub fn sdr50drvst(&self) -> SDR50DRVST_R {
        SDR50DRVST_R::new(((self.bits >> 27) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - SDR25 SD_CLK Frequency"]
    #[inline(always)]
    pub fn sdr25sdclkfreq(&mut self) -> SDR25SDCLKFREQ_W {
        SDR25SDCLKFREQ_W { w: self }
    }
    #[doc = "Bit 10 - SDR25 SD_CLK Gen Enable"]
    #[inline(always)]
    pub fn sdr25clkgenen(&mut self) -> SDR25CLKGENEN_W {
        SDR25CLKGENEN_W { w: self }
    }
    #[doc = "Bits 11:12 - SDR25 SD Drive Strength"]
    #[inline(always)]
    pub fn sdr25drvst(&mut self) -> SDR25DRVST_W {
        SDR25DRVST_W { w: self }
    }
    #[doc = "Bits 16:25 - Preset Value for SDR50 Speed of SD_CLK"]
    #[inline(always)]
    pub fn sdr50sdclkfreq(&mut self) -> SDR50SDCLKFREQ_W {
        SDR50SDCLKFREQ_W { w: self }
    }
    #[doc = "Bit 26 - SDR50 Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn sdr50clkgenen(&mut self) -> SDR50CLKGENEN_W {
        SDR50CLKGENEN_W { w: self }
    }
    #[doc = "Bits 27:28 - SDR50 Speed Drive Strength"]
    #[inline(always)]
    pub fn sdr50drvst(&mut self) -> SDR50DRVST_W {
        SDR50DRVST_W { w: self }
    }
}
