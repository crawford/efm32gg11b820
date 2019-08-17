#[doc = "Reader of register CFGPRESETVAL3"]
pub type R = crate::R<u32, super::CFGPRESETVAL3>;
#[doc = "Writer for register CFGPRESETVAL3"]
pub type W = crate::W<u32, super::CFGPRESETVAL3>;
#[doc = "Register CFGPRESETVAL3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGPRESETVAL3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDR104SDCLKFREQ`"]
pub type SDR104SDCLKFREQ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SDR104SDCLKFREQ`"]
pub struct SDR104SDCLKFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SDR104SDCLKFREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `SDR104CLKGENEN`"]
pub type SDR104CLKGENEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDR104CLKGENEN`"]
pub struct SDR104CLKGENEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDR104CLKGENEN_W<'a> {
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
#[doc = "Reader of field `SDR104DRVST`"]
pub type SDR104DRVST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDR104DRVST`"]
pub struct SDR104DRVST_W<'a> {
    w: &'a mut W,
}
impl<'a> SDR104DRVST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `DDR50SDCLKFREQ`"]
pub type DDR50SDCLKFREQ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DDR50SDCLKFREQ`"]
pub struct DDR50SDCLKFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DDR50SDCLKFREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DDR50CLKGENEN`"]
pub type DDR50CLKGENEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDR50CLKGENEN`"]
pub struct DDR50CLKGENEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDR50CLKGENEN_W<'a> {
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
#[doc = "Reader of field `DDR50DRVST`"]
pub type DDR50DRVST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DDR50DRVST`"]
pub struct DDR50DRVST_W<'a> {
    w: &'a mut W,
}
impl<'a> DDR50DRVST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - SDR104 SD_CLK Frequency"]
    #[inline(always)]
    pub fn sdr104sdclkfreq(&self) -> SDR104SDCLKFREQ_R {
        SDR104SDCLKFREQ_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - SDR104 SD_CLK Gen Enable"]
    #[inline(always)]
    pub fn sdr104clkgenen(&self) -> SDR104CLKGENEN_R {
        SDR104CLKGENEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - SDR104 SD Drive Strength"]
    #[inline(always)]
    pub fn sdr104drvst(&self) -> SDR104DRVST_R {
        SDR104DRVST_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 16:25 - Preset Value for DDR50 Speed of SD_CLK"]
    #[inline(always)]
    pub fn ddr50sdclkfreq(&self) -> DDR50SDCLKFREQ_R {
        DDR50SDCLKFREQ_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - DDR50 Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn ddr50clkgenen(&self) -> DDR50CLKGENEN_R {
        DDR50CLKGENEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:28 - DDR50 Speed Drive Strength"]
    #[inline(always)]
    pub fn ddr50drvst(&self) -> DDR50DRVST_R {
        DDR50DRVST_R::new(((self.bits >> 27) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - SDR104 SD_CLK Frequency"]
    #[inline(always)]
    pub fn sdr104sdclkfreq(&mut self) -> SDR104SDCLKFREQ_W {
        SDR104SDCLKFREQ_W { w: self }
    }
    #[doc = "Bit 10 - SDR104 SD_CLK Gen Enable"]
    #[inline(always)]
    pub fn sdr104clkgenen(&mut self) -> SDR104CLKGENEN_W {
        SDR104CLKGENEN_W { w: self }
    }
    #[doc = "Bits 11:12 - SDR104 SD Drive Strength"]
    #[inline(always)]
    pub fn sdr104drvst(&mut self) -> SDR104DRVST_W {
        SDR104DRVST_W { w: self }
    }
    #[doc = "Bits 16:25 - Preset Value for DDR50 Speed of SD_CLK"]
    #[inline(always)]
    pub fn ddr50sdclkfreq(&mut self) -> DDR50SDCLKFREQ_W {
        DDR50SDCLKFREQ_W { w: self }
    }
    #[doc = "Bit 26 - DDR50 Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn ddr50clkgenen(&mut self) -> DDR50CLKGENEN_W {
        DDR50CLKGENEN_W { w: self }
    }
    #[doc = "Bits 27:28 - DDR50 Speed Drive Strength"]
    #[inline(always)]
    pub fn ddr50drvst(&mut self) -> DDR50DRVST_W {
        DDR50DRVST_W { w: self }
    }
}
