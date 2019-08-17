#[doc = "Reader of register FLASHCMDCTRL"]
pub type R = crate::R<u32, super::FLASHCMDCTRL>;
#[doc = "Writer for register FLASHCMDCTRL"]
pub type W = crate::W<u32, super::FLASHCMDCTRL>;
#[doc = "Register FLASHCMDCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASHCMDCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CMDEXEC`"]
pub struct CMDEXEC_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDEXEC_W<'a> {
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
#[doc = "Reader of field `CMDEXECSTATUS`"]
pub type CMDEXECSTATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `STIGMEMBANKEN`"]
pub type STIGMEMBANKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STIGMEMBANKEN`"]
pub struct STIGMEMBANKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STIGMEMBANKEN_W<'a> {
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
#[doc = "Reader of field `NUMDUMMYCYCLES`"]
pub type NUMDUMMYCYCLES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NUMDUMMYCYCLES`"]
pub struct NUMDUMMYCYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> NUMDUMMYCYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 7)) | (((value as u32) & 0x1f) << 7);
        self.w
    }
}
#[doc = "Reader of field `NUMWRDATABYTES`"]
pub type NUMWRDATABYTES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NUMWRDATABYTES`"]
pub struct NUMWRDATABYTES_W<'a> {
    w: &'a mut W,
}
impl<'a> NUMWRDATABYTES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `ENBWRITEDATA`"]
pub type ENBWRITEDATA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENBWRITEDATA`"]
pub struct ENBWRITEDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBWRITEDATA_W<'a> {
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
#[doc = "Reader of field `NUMADDRBYTES`"]
pub type NUMADDRBYTES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NUMADDRBYTES`"]
pub struct NUMADDRBYTES_W<'a> {
    w: &'a mut W,
}
impl<'a> NUMADDRBYTES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `ENBMODEBIT`"]
pub type ENBMODEBIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENBMODEBIT`"]
pub struct ENBMODEBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBMODEBIT_W<'a> {
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
#[doc = "Reader of field `ENBCOMDADDR`"]
pub type ENBCOMDADDR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENBCOMDADDR`"]
pub struct ENBCOMDADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBCOMDADDR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `NUMRDDATABYTES`"]
pub type NUMRDDATABYTES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NUMRDDATABYTES`"]
pub struct NUMRDDATABYTES_W<'a> {
    w: &'a mut W,
}
impl<'a> NUMRDDATABYTES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `ENBREADDATA`"]
pub type ENBREADDATA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENBREADDATA`"]
pub struct ENBREADDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBREADDATA_W<'a> {
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
#[doc = "Reader of field `CMDOPCODE`"]
pub type CMDOPCODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMDOPCODE`"]
pub struct CMDOPCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDOPCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Command Execution in Progress"]
    #[inline(always)]
    pub fn cmdexecstatus(&self) -> CMDEXECSTATUS_R {
        CMDEXECSTATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - STIG Memory Bank Enable Bit"]
    #[inline(always)]
    pub fn stigmembanken(&self) -> STIGMEMBANKEN_R {
        STIGMEMBANKEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 7:11 - Number of Dummy Cycles"]
    #[inline(always)]
    pub fn numdummycycles(&self) -> NUMDUMMYCYCLES_R {
        NUMDUMMYCYCLES_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14 - Number of Write Data Bytes"]
    #[inline(always)]
    pub fn numwrdatabytes(&self) -> NUMWRDATABYTES_R {
        NUMWRDATABYTES_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Write Data Enable"]
    #[inline(always)]
    pub fn enbwritedata(&self) -> ENBWRITEDATA_R {
        ENBWRITEDATA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Number of Address Bytes"]
    #[inline(always)]
    pub fn numaddrbytes(&self) -> NUMADDRBYTES_R {
        NUMADDRBYTES_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Mode Bit Enable"]
    #[inline(always)]
    pub fn enbmodebit(&self) -> ENBMODEBIT_R {
        ENBMODEBIT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Command Address Enable"]
    #[inline(always)]
    pub fn enbcomdaddr(&self) -> ENBCOMDADDR_R {
        ENBCOMDADDR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - Number of Read Data Bytes"]
    #[inline(always)]
    pub fn numrddatabytes(&self) -> NUMRDDATABYTES_R {
        NUMRDDATABYTES_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - Read Data Enable"]
    #[inline(always)]
    pub fn enbreaddata(&self) -> ENBREADDATA_R {
        ENBREADDATA_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - Command Opcode"]
    #[inline(always)]
    pub fn cmdopcode(&self) -> CMDOPCODE_R {
        CMDOPCODE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Execute the Command"]
    #[inline(always)]
    pub fn cmdexec(&mut self) -> CMDEXEC_W {
        CMDEXEC_W { w: self }
    }
    #[doc = "Bit 2 - STIG Memory Bank Enable Bit"]
    #[inline(always)]
    pub fn stigmembanken(&mut self) -> STIGMEMBANKEN_W {
        STIGMEMBANKEN_W { w: self }
    }
    #[doc = "Bits 7:11 - Number of Dummy Cycles"]
    #[inline(always)]
    pub fn numdummycycles(&mut self) -> NUMDUMMYCYCLES_W {
        NUMDUMMYCYCLES_W { w: self }
    }
    #[doc = "Bits 12:14 - Number of Write Data Bytes"]
    #[inline(always)]
    pub fn numwrdatabytes(&mut self) -> NUMWRDATABYTES_W {
        NUMWRDATABYTES_W { w: self }
    }
    #[doc = "Bit 15 - Write Data Enable"]
    #[inline(always)]
    pub fn enbwritedata(&mut self) -> ENBWRITEDATA_W {
        ENBWRITEDATA_W { w: self }
    }
    #[doc = "Bits 16:17 - Number of Address Bytes"]
    #[inline(always)]
    pub fn numaddrbytes(&mut self) -> NUMADDRBYTES_W {
        NUMADDRBYTES_W { w: self }
    }
    #[doc = "Bit 18 - Mode Bit Enable"]
    #[inline(always)]
    pub fn enbmodebit(&mut self) -> ENBMODEBIT_W {
        ENBMODEBIT_W { w: self }
    }
    #[doc = "Bit 19 - Command Address Enable"]
    #[inline(always)]
    pub fn enbcomdaddr(&mut self) -> ENBCOMDADDR_W {
        ENBCOMDADDR_W { w: self }
    }
    #[doc = "Bits 20:22 - Number of Read Data Bytes"]
    #[inline(always)]
    pub fn numrddatabytes(&mut self) -> NUMRDDATABYTES_W {
        NUMRDDATABYTES_W { w: self }
    }
    #[doc = "Bit 23 - Read Data Enable"]
    #[inline(always)]
    pub fn enbreaddata(&mut self) -> ENBREADDATA_W {
        ENBREADDATA_W { w: self }
    }
    #[doc = "Bits 24:31 - Command Opcode"]
    #[inline(always)]
    pub fn cmdopcode(&mut self) -> CMDOPCODE_W {
        CMDOPCODE_W { w: self }
    }
}
