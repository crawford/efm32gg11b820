#[doc = "Reader of register CONFIG"]
pub type R = crate::R<u32, super::CONFIG>;
#[doc = "Writer for register CONFIG"]
pub type W = crate::W<u32, super::CONFIG>;
#[doc = "Register CONFIG `reset()`'s with value 0x8078_0081"]
impl crate::ResetValue for super::CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8078_0081
    }
}
#[doc = "Reader of field `ENBSPI`"]
pub type ENBSPI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENBSPI`"]
pub struct ENBSPI_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBSPI_W<'a> {
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
#[doc = "Reader of field `SELCLKPOL`"]
pub type SELCLKPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SELCLKPOL`"]
pub struct SELCLKPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SELCLKPOL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SELCLKPHASE`"]
pub type SELCLKPHASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SELCLKPHASE`"]
pub struct SELCLKPHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> SELCLKPHASE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PHYMODEENABLE`"]
pub type PHYMODEENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PHYMODEENABLE`"]
pub struct PHYMODEENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYMODEENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `ENBDIRACCCTLR`"]
pub type ENBDIRACCCTLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENBDIRACCCTLR`"]
pub struct ENBDIRACCCTLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBDIRACCCTLR_W<'a> {
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
#[doc = "Reader of field `ENBLEGACYIPMODE`"]
pub type ENBLEGACYIPMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENBLEGACYIPMODE`"]
pub struct ENBLEGACYIPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBLEGACYIPMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `PERIPHSELDEC`"]
pub type PERIPHSELDEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PERIPHSELDEC`"]
pub struct PERIPHSELDEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIPHSELDEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `PERIPHCSLINES`"]
pub type PERIPHCSLINES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PERIPHCSLINES`"]
pub struct PERIPHCSLINES_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIPHCSLINES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `WRPROTFLASH`"]
pub type WRPROTFLASH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRPROTFLASH`"]
pub struct WRPROTFLASH_W<'a> {
    w: &'a mut W,
}
impl<'a> WRPROTFLASH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `ENBAHBADDRREMAP`"]
pub type ENBAHBADDRREMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENBAHBADDRREMAP`"]
pub struct ENBAHBADDRREMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBAHBADDRREMAP_W<'a> {
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
#[doc = "Reader of field `ENTERXIPMODE`"]
pub type ENTERXIPMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENTERXIPMODE`"]
pub struct ENTERXIPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENTERXIPMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `ENTERXIPMODEIMM`"]
pub type ENTERXIPMODEIMM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENTERXIPMODEIMM`"]
pub struct ENTERXIPMODEIMM_W<'a> {
    w: &'a mut W,
}
impl<'a> ENTERXIPMODEIMM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `MSTRBAUDDIV`"]
pub type MSTRBAUDDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MSTRBAUDDIV`"]
pub struct MSTRBAUDDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTRBAUDDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 19)) | (((value as u32) & 0x0f) << 19);
        self.w
    }
}
#[doc = "Reader of field `ENABLEAHBDECODER`"]
pub type ENABLEAHBDECODER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLEAHBDECODER`"]
pub struct ENABLEAHBDECODER_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLEAHBDECODER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `ENABLEDTRPROTOCOL`"]
pub type ENABLEDTRPROTOCOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLEDTRPROTOCOL`"]
pub struct ENABLEDTRPROTOCOL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLEDTRPROTOCOL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `PIPELINEPHY`"]
pub type PIPELINEPHY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIPELINEPHY`"]
pub struct PIPELINEPHY_W<'a> {
    w: &'a mut W,
}
impl<'a> PIPELINEPHY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `CRCENABLE`"]
pub type CRCENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCENABLE`"]
pub struct CRCENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `DUALBYTEOPCODEEN`"]
pub type DUALBYTEOPCODEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUALBYTEOPCODEEN`"]
pub struct DUALBYTEOPCODEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DUALBYTEOPCODEEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `IDLE`"]
pub type IDLE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - QSPI Enable"]
    #[inline(always)]
    pub fn enbspi(&self) -> ENBSPI_R {
        ENBSPI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock Polarity, CPOL"]
    #[inline(always)]
    pub fn selclkpol(&self) -> SELCLKPOL_R {
        SELCLKPOL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clock Phase, CPHA"]
    #[inline(always)]
    pub fn selclkphase(&self) -> SELCLKPHASE_R {
        SELCLKPHASE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PHY Mode Enable"]
    #[inline(always)]
    pub fn phymodeenable(&self) -> PHYMODEENABLE_R {
        PHYMODEENABLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable Direct Access Controller"]
    #[inline(always)]
    pub fn enbdiraccctlr(&self) -> ENBDIRACCCTLR_R {
        ENBDIRACCCTLR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Legacy IP Mode Enable"]
    #[inline(always)]
    pub fn enblegacyipmode(&self) -> ENBLEGACYIPMODE_R {
        ENBLEGACYIPMODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Peripheral Select Decode"]
    #[inline(always)]
    pub fn periphseldec(&self) -> PERIPHSELDEC_R {
        PERIPHSELDEC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Peripheral Chip Select Lines"]
    #[inline(always)]
    pub fn periphcslines(&self) -> PERIPHCSLINES_R {
        PERIPHCSLINES_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 14 - Write Protect Flash Pin"]
    #[inline(always)]
    pub fn wrprotflash(&self) -> WRPROTFLASH_R {
        WRPROTFLASH_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable Address Remapping"]
    #[inline(always)]
    pub fn enbahbaddrremap(&self) -> ENBAHBADDRREMAP_R {
        ENBAHBADDRREMAP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enter XIP Mode on Next READ"]
    #[inline(always)]
    pub fn enterxipmode(&self) -> ENTERXIPMODE_R {
        ENTERXIPMODE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enter XIP Mode Immediately"]
    #[inline(always)]
    pub fn enterxipmodeimm(&self) -> ENTERXIPMODEIMM_R {
        ENTERXIPMODEIMM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 19:22 - Master Mode Baud Rate Divisor"]
    #[inline(always)]
    pub fn mstrbauddiv(&self) -> MSTRBAUDDIV_R {
        MSTRBAUDDIV_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - Enable Address Decoder"]
    #[inline(always)]
    pub fn enableahbdecoder(&self) -> ENABLEAHBDECODER_R {
        ENABLEAHBDECODER_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable DTR Protocol"]
    #[inline(always)]
    pub fn enabledtrprotocol(&self) -> ENABLEDTRPROTOCOL_R {
        ENABLEDTRPROTOCOL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pipeline PHY Mode Enable"]
    #[inline(always)]
    pub fn pipelinephy(&self) -> PIPELINEPHY_R {
        PIPELINEPHY_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 29 - CRC Enable Bit"]
    #[inline(always)]
    pub fn crcenable(&self) -> CRCENABLE_R {
        CRCENABLE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Dual-byte Opcode Mode Enable Bit"]
    #[inline(always)]
    pub fn dualbyteopcodeen(&self) -> DUALBYTEOPCODEEN_R {
        DUALBYTEOPCODEEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Serial Interface and Low Level SPI Pipeline is IDLE"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - QSPI Enable"]
    #[inline(always)]
    pub fn enbspi(&mut self) -> ENBSPI_W {
        ENBSPI_W { w: self }
    }
    #[doc = "Bit 1 - Clock Polarity, CPOL"]
    #[inline(always)]
    pub fn selclkpol(&mut self) -> SELCLKPOL_W {
        SELCLKPOL_W { w: self }
    }
    #[doc = "Bit 2 - Clock Phase, CPHA"]
    #[inline(always)]
    pub fn selclkphase(&mut self) -> SELCLKPHASE_W {
        SELCLKPHASE_W { w: self }
    }
    #[doc = "Bit 3 - PHY Mode Enable"]
    #[inline(always)]
    pub fn phymodeenable(&mut self) -> PHYMODEENABLE_W {
        PHYMODEENABLE_W { w: self }
    }
    #[doc = "Bit 7 - Enable Direct Access Controller"]
    #[inline(always)]
    pub fn enbdiraccctlr(&mut self) -> ENBDIRACCCTLR_W {
        ENBDIRACCCTLR_W { w: self }
    }
    #[doc = "Bit 8 - Legacy IP Mode Enable"]
    #[inline(always)]
    pub fn enblegacyipmode(&mut self) -> ENBLEGACYIPMODE_W {
        ENBLEGACYIPMODE_W { w: self }
    }
    #[doc = "Bit 9 - Peripheral Select Decode"]
    #[inline(always)]
    pub fn periphseldec(&mut self) -> PERIPHSELDEC_W {
        PERIPHSELDEC_W { w: self }
    }
    #[doc = "Bits 10:11 - Peripheral Chip Select Lines"]
    #[inline(always)]
    pub fn periphcslines(&mut self) -> PERIPHCSLINES_W {
        PERIPHCSLINES_W { w: self }
    }
    #[doc = "Bit 14 - Write Protect Flash Pin"]
    #[inline(always)]
    pub fn wrprotflash(&mut self) -> WRPROTFLASH_W {
        WRPROTFLASH_W { w: self }
    }
    #[doc = "Bit 16 - Enable Address Remapping"]
    #[inline(always)]
    pub fn enbahbaddrremap(&mut self) -> ENBAHBADDRREMAP_W {
        ENBAHBADDRREMAP_W { w: self }
    }
    #[doc = "Bit 17 - Enter XIP Mode on Next READ"]
    #[inline(always)]
    pub fn enterxipmode(&mut self) -> ENTERXIPMODE_W {
        ENTERXIPMODE_W { w: self }
    }
    #[doc = "Bit 18 - Enter XIP Mode Immediately"]
    #[inline(always)]
    pub fn enterxipmodeimm(&mut self) -> ENTERXIPMODEIMM_W {
        ENTERXIPMODEIMM_W { w: self }
    }
    #[doc = "Bits 19:22 - Master Mode Baud Rate Divisor"]
    #[inline(always)]
    pub fn mstrbauddiv(&mut self) -> MSTRBAUDDIV_W {
        MSTRBAUDDIV_W { w: self }
    }
    #[doc = "Bit 23 - Enable Address Decoder"]
    #[inline(always)]
    pub fn enableahbdecoder(&mut self) -> ENABLEAHBDECODER_W {
        ENABLEAHBDECODER_W { w: self }
    }
    #[doc = "Bit 24 - Enable DTR Protocol"]
    #[inline(always)]
    pub fn enabledtrprotocol(&mut self) -> ENABLEDTRPROTOCOL_W {
        ENABLEDTRPROTOCOL_W { w: self }
    }
    #[doc = "Bit 25 - Pipeline PHY Mode Enable"]
    #[inline(always)]
    pub fn pipelinephy(&mut self) -> PIPELINEPHY_W {
        PIPELINEPHY_W { w: self }
    }
    #[doc = "Bit 29 - CRC Enable Bit"]
    #[inline(always)]
    pub fn crcenable(&mut self) -> CRCENABLE_W {
        CRCENABLE_W { w: self }
    }
    #[doc = "Bit 30 - Dual-byte Opcode Mode Enable Bit"]
    #[inline(always)]
    pub fn dualbyteopcodeen(&mut self) -> DUALBYTEOPCODEEN_W {
        DUALBYTEOPCODEEN_W { w: self }
    }
}
