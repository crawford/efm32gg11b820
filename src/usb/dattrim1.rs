#[doc = "Reader of register DATTRIM1"]
pub type R = crate::R<u32, super::DATTRIM1>;
#[doc = "Writer for register DATTRIM1"]
pub type W = crate::W<u32, super::DATTRIM1>;
#[doc = "Register DATTRIM1 `reset()`'s with value 0x24"]
impl crate::ResetValue for super::DATTRIM1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x24
    }
}
#[doc = "Reader of field `ROUT`"]
pub type ROUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ROUT`"]
pub struct ROUT_W<'a> {
    w: &'a mut W,
}
impl<'a> ROUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `ENDLYPULLUP`"]
pub type ENDLYPULLUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENDLYPULLUP`"]
pub struct ENDLYPULLUP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDLYPULLUP_W<'a> {
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
#[doc = "Reader of field `DLYPULLUPFS`"]
pub type DLYPULLUPFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLYPULLUPFS`"]
pub struct DLYPULLUPFS_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYPULLUPFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `VCRSFS`"]
pub type VCRSFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VCRSFS`"]
pub struct VCRSFS_W<'a> {
    w: &'a mut W,
}
impl<'a> VCRSFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `TFDMFS`"]
pub type TFDMFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TFDMFS`"]
pub struct TFDMFS_W<'a> {
    w: &'a mut W,
}
impl<'a> TFDMFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `TRDMFS`"]
pub type TRDMFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRDMFS`"]
pub struct TRDMFS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRDMFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `TFDPFS`"]
pub type TFDPFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TFDPFS`"]
pub struct TFDPFS_W<'a> {
    w: &'a mut W,
}
impl<'a> TFDPFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `TRDPFS`"]
pub type TRDPFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRDPFS`"]
pub struct TRDPFS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRDPFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Trim for DP and DM Output Impedance for Both FS and LS"]
    #[inline(always)]
    pub fn rout(&self) -> ROUT_R {
        ROUT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Enables Delay of Pull in TX Mode for Both FS and LS"]
    #[inline(always)]
    pub fn endlypullup(&self) -> ENDLYPULLUP_R {
        ENDLYPULLUP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Trim for Rising Crossover Voltage in FS"]
    #[inline(always)]
    pub fn dlypullupfs(&self) -> DLYPULLUPFS_R {
        DLYPULLUPFS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Trim for Falling Crossover Voltage in FS"]
    #[inline(always)]
    pub fn vcrsfs(&self) -> VCRSFS_R {
        VCRSFS_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Trim for DM Fall Time in FS"]
    #[inline(always)]
    pub fn tfdmfs(&self) -> TFDMFS_R {
        TFDMFS_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Trim for DM Rise Time in FS"]
    #[inline(always)]
    pub fn trdmfs(&self) -> TRDMFS_R {
        TRDMFS_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Trim for DP Fall Time in FS"]
    #[inline(always)]
    pub fn tfdpfs(&self) -> TFDPFS_R {
        TFDPFS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Trim for DP Rise Time in FS"]
    #[inline(always)]
    pub fn trdpfs(&self) -> TRDPFS_R {
        TRDPFS_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Trim for DP and DM Output Impedance for Both FS and LS"]
    #[inline(always)]
    pub fn rout(&mut self) -> ROUT_W {
        ROUT_W { w: self }
    }
    #[doc = "Bit 7 - Enables Delay of Pull in TX Mode for Both FS and LS"]
    #[inline(always)]
    pub fn endlypullup(&mut self) -> ENDLYPULLUP_W {
        ENDLYPULLUP_W { w: self }
    }
    #[doc = "Bits 8:9 - Trim for Rising Crossover Voltage in FS"]
    #[inline(always)]
    pub fn dlypullupfs(&mut self) -> DLYPULLUPFS_W {
        DLYPULLUPFS_W { w: self }
    }
    #[doc = "Bits 10:11 - Trim for Falling Crossover Voltage in FS"]
    #[inline(always)]
    pub fn vcrsfs(&mut self) -> VCRSFS_W {
        VCRSFS_W { w: self }
    }
    #[doc = "Bits 12:13 - Trim for DM Fall Time in FS"]
    #[inline(always)]
    pub fn tfdmfs(&mut self) -> TFDMFS_W {
        TFDMFS_W { w: self }
    }
    #[doc = "Bits 14:15 - Trim for DM Rise Time in FS"]
    #[inline(always)]
    pub fn trdmfs(&mut self) -> TRDMFS_W {
        TRDMFS_W { w: self }
    }
    #[doc = "Bits 16:17 - Trim for DP Fall Time in FS"]
    #[inline(always)]
    pub fn tfdpfs(&mut self) -> TFDPFS_W {
        TFDPFS_W { w: self }
    }
    #[doc = "Bits 18:19 - Trim for DP Rise Time in FS"]
    #[inline(always)]
    pub fn trdpfs(&mut self) -> TRDPFS_W {
        TRDPFS_W { w: self }
    }
}
