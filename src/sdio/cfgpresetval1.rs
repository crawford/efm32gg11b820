#[doc = "Reader of register CFGPRESETVAL1"]
pub type R = crate::R<u32, super::CFGPRESETVAL1>;
#[doc = "Writer for register CFGPRESETVAL1"]
pub type W = crate::W<u32, super::CFGPRESETVAL1>;
#[doc = "Register CFGPRESETVAL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGPRESETVAL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HSPSDCLKFREQ`"]
pub type HSPSDCLKFREQ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HSPSDCLKFREQ`"]
pub struct HSPSDCLKFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HSPSDCLKFREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `HSPCLKGENEN`"]
pub type HSPCLKGENEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSPCLKGENEN`"]
pub struct HSPCLKGENEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSPCLKGENEN_W<'a> {
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
#[doc = "Reader of field `HSPDRVST`"]
pub type HSPDRVST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSPDRVST`"]
pub struct HSPDRVST_W<'a> {
    w: &'a mut W,
}
impl<'a> HSPDRVST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `SDR12SDCLKFREQ`"]
pub type SDR12SDCLKFREQ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SDR12SDCLKFREQ`"]
pub struct SDR12SDCLKFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SDR12SDCLKFREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SDR12CLKGENEN`"]
pub type SDR12CLKGENEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDR12CLKGENEN`"]
pub struct SDR12CLKGENEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDR12CLKGENEN_W<'a> {
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
#[doc = "Reader of field `SDR12DRVST`"]
pub type SDR12DRVST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDR12DRVST`"]
pub struct SDR12DRVST_W<'a> {
    w: &'a mut W,
}
impl<'a> SDR12DRVST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - High Speed SD_CLK Frequency"]
    #[inline(always)]
    pub fn hspsdclkfreq(&self) -> HSPSDCLKFREQ_R {
        HSPSDCLKFREQ_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - High Speed SD_CLK Gen Enable"]
    #[inline(always)]
    pub fn hspclkgenen(&self) -> HSPCLKGENEN_R {
        HSPCLKGENEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - High Speed SD Drive Strength"]
    #[inline(always)]
    pub fn hspdrvst(&self) -> HSPDRVST_R {
        HSPDRVST_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 16:25 - Preset Value for SDR12 Speed of SD_CLK"]
    #[inline(always)]
    pub fn sdr12sdclkfreq(&self) -> SDR12SDCLKFREQ_R {
        SDR12SDCLKFREQ_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - SDR12 Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn sdr12clkgenen(&self) -> SDR12CLKGENEN_R {
        SDR12CLKGENEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:28 - SDR12 Speed Drive Strength"]
    #[inline(always)]
    pub fn sdr12drvst(&self) -> SDR12DRVST_R {
        SDR12DRVST_R::new(((self.bits >> 27) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - High Speed SD_CLK Frequency"]
    #[inline(always)]
    pub fn hspsdclkfreq(&mut self) -> HSPSDCLKFREQ_W {
        HSPSDCLKFREQ_W { w: self }
    }
    #[doc = "Bit 10 - High Speed SD_CLK Gen Enable"]
    #[inline(always)]
    pub fn hspclkgenen(&mut self) -> HSPCLKGENEN_W {
        HSPCLKGENEN_W { w: self }
    }
    #[doc = "Bits 11:12 - High Speed SD Drive Strength"]
    #[inline(always)]
    pub fn hspdrvst(&mut self) -> HSPDRVST_W {
        HSPDRVST_W { w: self }
    }
    #[doc = "Bits 16:25 - Preset Value for SDR12 Speed of SD_CLK"]
    #[inline(always)]
    pub fn sdr12sdclkfreq(&mut self) -> SDR12SDCLKFREQ_W {
        SDR12SDCLKFREQ_W { w: self }
    }
    #[doc = "Bit 26 - SDR12 Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn sdr12clkgenen(&mut self) -> SDR12CLKGENEN_W {
        SDR12CLKGENEN_W { w: self }
    }
    #[doc = "Bits 27:28 - SDR12 Speed Drive Strength"]
    #[inline(always)]
    pub fn sdr12drvst(&mut self) -> SDR12DRVST_W {
        SDR12DRVST_W { w: self }
    }
}
