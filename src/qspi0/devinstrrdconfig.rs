#[doc = "Reader of register DEVINSTRRDCONFIG"]
pub type R = crate::R<u32, super::DEVINSTRRDCONFIG>;
#[doc = "Writer for register DEVINSTRRDCONFIG"]
pub type W = crate::W<u32, super::DEVINSTRRDCONFIG>;
#[doc = "Register DEVINSTRRDCONFIG `reset()`'s with value 0x03"]
impl crate::ResetValue for super::DEVINSTRRDCONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Reader of field `RDOPCODENONXIP`"]
pub type RDOPCODENONXIP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDOPCODENONXIP`"]
pub struct RDOPCODENONXIP_W<'a> {
    w: &'a mut W,
}
impl<'a> RDOPCODENONXIP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `INSTRTYPE`"]
pub type INSTRTYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTRTYPE`"]
pub struct INSTRTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTRTYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `DDREN`"]
pub type DDREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDREN`"]
pub struct DDREN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDREN_W<'a> {
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
#[doc = "Reader of field `ADDRXFERTYPESTDMODE`"]
pub type ADDRXFERTYPESTDMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRXFERTYPESTDMODE`"]
pub struct ADDRXFERTYPESTDMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRXFERTYPESTDMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `DATAXFERTYPEEXTMODE`"]
pub type DATAXFERTYPEEXTMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATAXFERTYPEEXTMODE`"]
pub struct DATAXFERTYPEEXTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAXFERTYPEEXTMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `MODEBITENABLE`"]
pub type MODEBITENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODEBITENABLE`"]
pub struct MODEBITENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODEBITENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `DUMMYRDCLKCYCLES`"]
pub type DUMMYRDCLKCYCLES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DUMMYRDCLKCYCLES`"]
pub struct DUMMYRDCLKCYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> DUMMYRDCLKCYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Read Opcode in Non-XIP Mode"]
    #[inline(always)]
    pub fn rdopcodenonxip(&self) -> RDOPCODENONXIP_R {
        RDOPCODENONXIP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Instruction Type"]
    #[inline(always)]
    pub fn instrtype(&self) -> INSTRTYPE_R {
        INSTRTYPE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - DDR Enable"]
    #[inline(always)]
    pub fn ddren(&self) -> DDREN_R {
        DDREN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Address Transfer Type for Standard SPI Modes"]
    #[inline(always)]
    pub fn addrxfertypestdmode(&self) -> ADDRXFERTYPESTDMODE_R {
        ADDRXFERTYPESTDMODE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Data Transfer Type for Standard SPI Modes"]
    #[inline(always)]
    pub fn dataxfertypeextmode(&self) -> DATAXFERTYPEEXTMODE_R {
        DATAXFERTYPEEXTMODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 20 - Mode Bit Enable"]
    #[inline(always)]
    pub fn modebitenable(&self) -> MODEBITENABLE_R {
        MODEBITENABLE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 24:28 - Dummy Read Clock Cycles"]
    #[inline(always)]
    pub fn dummyrdclkcycles(&self) -> DUMMYRDCLKCYCLES_R {
        DUMMYRDCLKCYCLES_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read Opcode in Non-XIP Mode"]
    #[inline(always)]
    pub fn rdopcodenonxip(&mut self) -> RDOPCODENONXIP_W {
        RDOPCODENONXIP_W { w: self }
    }
    #[doc = "Bits 8:9 - Instruction Type"]
    #[inline(always)]
    pub fn instrtype(&mut self) -> INSTRTYPE_W {
        INSTRTYPE_W { w: self }
    }
    #[doc = "Bit 10 - DDR Enable"]
    #[inline(always)]
    pub fn ddren(&mut self) -> DDREN_W {
        DDREN_W { w: self }
    }
    #[doc = "Bits 12:13 - Address Transfer Type for Standard SPI Modes"]
    #[inline(always)]
    pub fn addrxfertypestdmode(&mut self) -> ADDRXFERTYPESTDMODE_W {
        ADDRXFERTYPESTDMODE_W { w: self }
    }
    #[doc = "Bits 16:17 - Data Transfer Type for Standard SPI Modes"]
    #[inline(always)]
    pub fn dataxfertypeextmode(&mut self) -> DATAXFERTYPEEXTMODE_W {
        DATAXFERTYPEEXTMODE_W { w: self }
    }
    #[doc = "Bit 20 - Mode Bit Enable"]
    #[inline(always)]
    pub fn modebitenable(&mut self) -> MODEBITENABLE_W {
        MODEBITENABLE_W { w: self }
    }
    #[doc = "Bits 24:28 - Dummy Read Clock Cycles"]
    #[inline(always)]
    pub fn dummyrdclkcycles(&mut self) -> DUMMYRDCLKCYCLES_W {
        DUMMYRDCLKCYCLES_W { w: self }
    }
}
