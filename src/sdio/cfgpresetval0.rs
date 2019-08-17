#[doc = "Reader of register CFGPRESETVAL0"]
pub type R = crate::R<u32, super::CFGPRESETVAL0>;
#[doc = "Writer for register CFGPRESETVAL0"]
pub type W = crate::W<u32, super::CFGPRESETVAL0>;
#[doc = "Register CFGPRESETVAL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGPRESETVAL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INITSDCLKFREQ`"]
pub type INITSDCLKFREQ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INITSDCLKFREQ`"]
pub struct INITSDCLKFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> INITSDCLKFREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `INITCLKGENEN`"]
pub type INITCLKGENEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INITCLKGENEN`"]
pub struct INITCLKGENEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INITCLKGENEN_W<'a> {
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
#[doc = "Reader of field `INITDRVST`"]
pub type INITDRVST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INITDRVST`"]
pub struct INITDRVST_W<'a> {
    w: &'a mut W,
}
impl<'a> INITDRVST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `DSPSDCLKFREQ`"]
pub type DSPSDCLKFREQ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DSPSDCLKFREQ`"]
pub struct DSPSDCLKFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DSPSDCLKFREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DSPCLKGENEN`"]
pub type DSPCLKGENEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSPCLKGENEN`"]
pub struct DSPCLKGENEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSPCLKGENEN_W<'a> {
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
#[doc = "Reader of field `DSPDRVST`"]
pub type DSPDRVST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DSPDRVST`"]
pub struct DSPDRVST_W<'a> {
    w: &'a mut W,
}
impl<'a> DSPDRVST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Initial SD_CLK Frequency"]
    #[inline(always)]
    pub fn initsdclkfreq(&self) -> INITSDCLKFREQ_R {
        INITSDCLKFREQ_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Initial Clock Gen Enable"]
    #[inline(always)]
    pub fn initclkgenen(&self) -> INITCLKGENEN_R {
        INITCLKGENEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - Initial Drive Strength"]
    #[inline(always)]
    pub fn initdrvst(&self) -> INITDRVST_R {
        INITDRVST_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 16:25 - Preset Value for Default Speed of SD_CLK"]
    #[inline(always)]
    pub fn dspsdclkfreq(&self) -> DSPSDCLKFREQ_R {
        DSPSDCLKFREQ_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Default Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn dspclkgenen(&self) -> DSPCLKGENEN_R {
        DSPCLKGENEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:28 - Default Speed Drive Strength"]
    #[inline(always)]
    pub fn dspdrvst(&self) -> DSPDRVST_R {
        DSPDRVST_R::new(((self.bits >> 27) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Initial SD_CLK Frequency"]
    #[inline(always)]
    pub fn initsdclkfreq(&mut self) -> INITSDCLKFREQ_W {
        INITSDCLKFREQ_W { w: self }
    }
    #[doc = "Bit 10 - Initial Clock Gen Enable"]
    #[inline(always)]
    pub fn initclkgenen(&mut self) -> INITCLKGENEN_W {
        INITCLKGENEN_W { w: self }
    }
    #[doc = "Bits 11:12 - Initial Drive Strength"]
    #[inline(always)]
    pub fn initdrvst(&mut self) -> INITDRVST_W {
        INITDRVST_W { w: self }
    }
    #[doc = "Bits 16:25 - Preset Value for Default Speed of SD_CLK"]
    #[inline(always)]
    pub fn dspsdclkfreq(&mut self) -> DSPSDCLKFREQ_W {
        DSPSDCLKFREQ_W { w: self }
    }
    #[doc = "Bit 26 - Default Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn dspclkgenen(&mut self) -> DSPCLKGENEN_W {
        DSPCLKGENEN_W { w: self }
    }
    #[doc = "Bits 27:28 - Default Speed Drive Strength"]
    #[inline(always)]
    pub fn dspdrvst(&mut self) -> DSPDRVST_W {
        DSPDRVST_W { w: self }
    }
}
