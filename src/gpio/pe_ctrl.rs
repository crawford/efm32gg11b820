#[doc = "Reader of register PE_CTRL"]
pub type R = crate::R<u32, super::PE_CTRL>;
#[doc = "Writer for register PE_CTRL"]
pub type W = crate::W<u32, super::PE_CTRL>;
#[doc = "Register PE_CTRL `reset()`'s with value 0x0050_0050"]
impl crate::ResetValue for super::PE_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0050_0050
    }
}
#[doc = "Reader of field `DRIVESTRENGTH`"]
pub type DRIVESTRENGTH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DRIVESTRENGTH`"]
pub struct DRIVESTRENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVESTRENGTH_W<'a> {
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
#[doc = "Reader of field `SLEWRATE`"]
pub type SLEWRATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLEWRATE`"]
pub struct SLEWRATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEWRATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `DINDIS`"]
pub type DINDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DINDIS`"]
pub struct DINDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DINDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `DRIVESTRENGTHALT`"]
pub type DRIVESTRENGTHALT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DRIVESTRENGTHALT`"]
pub struct DRIVESTRENGTHALT_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVESTRENGTHALT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SLEWRATEALT`"]
pub type SLEWRATEALT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLEWRATEALT`"]
pub struct SLEWRATEALT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEWRATEALT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `DINDISALT`"]
pub type DINDISALT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DINDISALT`"]
pub struct DINDISALT_W<'a> {
    w: &'a mut W,
}
impl<'a> DINDISALT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Drive Strength for Port"]
    #[inline(always)]
    pub fn drivestrength(&self) -> DRIVESTRENGTH_R {
        DRIVESTRENGTH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Slewrate Limit for Port"]
    #[inline(always)]
    pub fn slewrate(&self) -> SLEWRATE_R {
        SLEWRATE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Data in Disable"]
    #[inline(always)]
    pub fn dindis(&self) -> DINDIS_R {
        DINDIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Alternate Drive Strength for Port"]
    #[inline(always)]
    pub fn drivestrengthalt(&self) -> DRIVESTRENGTHALT_R {
        DRIVESTRENGTHALT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - Alternate Slewrate Limit for Port"]
    #[inline(always)]
    pub fn slewratealt(&self) -> SLEWRATEALT_R {
        SLEWRATEALT_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 28 - Alternate Data in Disable"]
    #[inline(always)]
    pub fn dindisalt(&self) -> DINDISALT_R {
        DINDISALT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Drive Strength for Port"]
    #[inline(always)]
    pub fn drivestrength(&mut self) -> DRIVESTRENGTH_W {
        DRIVESTRENGTH_W { w: self }
    }
    #[doc = "Bits 4:6 - Slewrate Limit for Port"]
    #[inline(always)]
    pub fn slewrate(&mut self) -> SLEWRATE_W {
        SLEWRATE_W { w: self }
    }
    #[doc = "Bit 12 - Data in Disable"]
    #[inline(always)]
    pub fn dindis(&mut self) -> DINDIS_W {
        DINDIS_W { w: self }
    }
    #[doc = "Bit 16 - Alternate Drive Strength for Port"]
    #[inline(always)]
    pub fn drivestrengthalt(&mut self) -> DRIVESTRENGTHALT_W {
        DRIVESTRENGTHALT_W { w: self }
    }
    #[doc = "Bits 20:22 - Alternate Slewrate Limit for Port"]
    #[inline(always)]
    pub fn slewratealt(&mut self) -> SLEWRATEALT_W {
        SLEWRATEALT_W { w: self }
    }
    #[doc = "Bit 28 - Alternate Data in Disable"]
    #[inline(always)]
    pub fn dindisalt(&mut self) -> DINDISALT_W {
        DINDISALT_W { w: self }
    }
}
