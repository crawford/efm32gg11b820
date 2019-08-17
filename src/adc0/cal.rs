#[doc = "Reader of register CAL"]
pub type R = crate::R<u32, super::CAL>;
#[doc = "Writer for register CAL"]
pub type W = crate::W<u32, super::CAL>;
#[doc = "Register CAL `reset()`'s with value 0x4078_4078"]
impl crate::ResetValue for super::CAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4078_4078
    }
}
#[doc = "Reader of field `SINGLEOFFSET`"]
pub type SINGLEOFFSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SINGLEOFFSET`"]
pub struct SINGLEOFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLEOFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `SINGLEOFFSETINV`"]
pub type SINGLEOFFSETINV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SINGLEOFFSETINV`"]
pub struct SINGLEOFFSETINV_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLEOFFSETINV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SINGLEGAIN`"]
pub type SINGLEGAIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SINGLEGAIN`"]
pub struct SINGLEGAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLEGAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `OFFSETINVMODE`"]
pub type OFFSETINVMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OFFSETINVMODE`"]
pub struct OFFSETINVMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSETINVMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `SCANOFFSET`"]
pub type SCANOFFSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCANOFFSET`"]
pub struct SCANOFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANOFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `SCANOFFSETINV`"]
pub type SCANOFFSETINV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCANOFFSETINV`"]
pub struct SCANOFFSETINV_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANOFFSETINV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `SCANGAIN`"]
pub type SCANGAIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCANGAIN`"]
pub struct SCANGAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANGAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
#[doc = "Reader of field `CALEN`"]
pub type CALEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CALEN`"]
pub struct CALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CALEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Single Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
    #[inline(always)]
    pub fn singleoffset(&self) -> SINGLEOFFSET_R {
        SINGLEOFFSET_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Single Mode Offset Calibration Value for Negative Single-ended Mode"]
    #[inline(always)]
    pub fn singleoffsetinv(&self) -> SINGLEOFFSETINV_R {
        SINGLEOFFSETINV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - Single Mode Gain Calibration Value"]
    #[inline(always)]
    pub fn singlegain(&self) -> SINGLEGAIN_R {
        SINGLEGAIN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Negative Single-ended Offset Calibration is Enabled"]
    #[inline(always)]
    pub fn offsetinvmode(&self) -> OFFSETINVMODE_R {
        OFFSETINVMODE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Scan Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
    #[inline(always)]
    pub fn scanoffset(&self) -> SCANOFFSET_R {
        SCANOFFSET_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Scan Mode Offset Calibration Value for Negative Single-ended Mode"]
    #[inline(always)]
    pub fn scanoffsetinv(&self) -> SCANOFFSETINV_R {
        SCANOFFSETINV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:30 - Scan Mode Gain Calibration Value"]
    #[inline(always)]
    pub fn scangain(&self) -> SCANGAIN_R {
        SCANGAIN_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Calibration Mode is Enabled"]
    #[inline(always)]
    pub fn calen(&self) -> CALEN_R {
        CALEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Single Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
    #[inline(always)]
    pub fn singleoffset(&mut self) -> SINGLEOFFSET_W {
        SINGLEOFFSET_W { w: self }
    }
    #[doc = "Bits 4:7 - Single Mode Offset Calibration Value for Negative Single-ended Mode"]
    #[inline(always)]
    pub fn singleoffsetinv(&mut self) -> SINGLEOFFSETINV_W {
        SINGLEOFFSETINV_W { w: self }
    }
    #[doc = "Bits 8:14 - Single Mode Gain Calibration Value"]
    #[inline(always)]
    pub fn singlegain(&mut self) -> SINGLEGAIN_W {
        SINGLEGAIN_W { w: self }
    }
    #[doc = "Bit 15 - Negative Single-ended Offset Calibration is Enabled"]
    #[inline(always)]
    pub fn offsetinvmode(&mut self) -> OFFSETINVMODE_W {
        OFFSETINVMODE_W { w: self }
    }
    #[doc = "Bits 16:19 - Scan Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
    #[inline(always)]
    pub fn scanoffset(&mut self) -> SCANOFFSET_W {
        SCANOFFSET_W { w: self }
    }
    #[doc = "Bits 20:23 - Scan Mode Offset Calibration Value for Negative Single-ended Mode"]
    #[inline(always)]
    pub fn scanoffsetinv(&mut self) -> SCANOFFSETINV_W {
        SCANOFFSETINV_W { w: self }
    }
    #[doc = "Bits 24:30 - Scan Mode Gain Calibration Value"]
    #[inline(always)]
    pub fn scangain(&mut self) -> SCANGAIN_W {
        SCANGAIN_W { w: self }
    }
    #[doc = "Bit 31 - Calibration Mode is Enabled"]
    #[inline(always)]
    pub fn calen(&mut self) -> CALEN_W {
        CALEN_W { w: self }
    }
}
