#[doc = "Reader of register HC10_INTMSK"]
pub type R = crate::R<u32, super::HC10_INTMSK>;
#[doc = "Writer for register HC10_INTMSK"]
pub type W = crate::W<u32, super::HC10_INTMSK>;
#[doc = "Register HC10_INTMSK `reset()`'s with value 0"]
impl crate::ResetValue for super::HC10_INTMSK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XFERCOMPLMSK`"]
pub type XFERCOMPLMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XFERCOMPLMSK`"]
pub struct XFERCOMPLMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> XFERCOMPLMSK_W<'a> {
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
#[doc = "Reader of field `CHHLTDMSK`"]
pub type CHHLTDMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHHLTDMSK`"]
pub struct CHHLTDMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> CHHLTDMSK_W<'a> {
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
#[doc = "Reader of field `AHBERRMSK`"]
pub type AHBERRMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHBERRMSK`"]
pub struct AHBERRMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBERRMSK_W<'a> {
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
#[doc = "Reader of field `STALLMSK`"]
pub type STALLMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STALLMSK`"]
pub struct STALLMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLMSK_W<'a> {
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
#[doc = "Reader of field `NAKMSK`"]
pub type NAKMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NAKMSK`"]
pub struct NAKMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `ACKMSK`"]
pub type ACKMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACKMSK`"]
pub struct ACKMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `XACTERRMSK`"]
pub type XACTERRMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XACTERRMSK`"]
pub struct XACTERRMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> XACTERRMSK_W<'a> {
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
#[doc = "Reader of field `BBLERRMSK`"]
pub type BBLERRMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BBLERRMSK`"]
pub struct BBLERRMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> BBLERRMSK_W<'a> {
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
#[doc = "Reader of field `FRMOVRUNMSK`"]
pub type FRMOVRUNMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRMOVRUNMSK`"]
pub struct FRMOVRUNMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMOVRUNMSK_W<'a> {
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
#[doc = "Reader of field `DATATGLERRMSK`"]
pub type DATATGLERRMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATATGLERRMSK`"]
pub struct DATATGLERRMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> DATATGLERRMSK_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Transfer Completed Mask"]
    #[inline(always)]
    pub fn xfercomplmsk(&self) -> XFERCOMPLMSK_R {
        XFERCOMPLMSK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel Halted Mask"]
    #[inline(always)]
    pub fn chhltdmsk(&self) -> CHHLTDMSK_R {
        CHHLTDMSK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AHB Error Mask"]
    #[inline(always)]
    pub fn ahberrmsk(&self) -> AHBERRMSK_R {
        AHBERRMSK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt Mask"]
    #[inline(always)]
    pub fn stallmsk(&self) -> STALLMSK_R {
        STALLMSK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt Mask"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt Mask"]
    #[inline(always)]
    pub fn ackmsk(&self) -> ACKMSK_R {
        ACKMSK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transaction Error Mask"]
    #[inline(always)]
    pub fn xacterrmsk(&self) -> XACTERRMSK_R {
        XACTERRMSK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Babble Error Mask"]
    #[inline(always)]
    pub fn bblerrmsk(&self) -> BBLERRMSK_R {
        BBLERRMSK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Frame Overrun Mask"]
    #[inline(always)]
    pub fn frmovrunmsk(&self) -> FRMOVRUNMSK_R {
        FRMOVRUNMSK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Data Toggle Error Mask"]
    #[inline(always)]
    pub fn datatglerrmsk(&self) -> DATATGLERRMSK_R {
        DATATGLERRMSK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Mask"]
    #[inline(always)]
    pub fn xfercomplmsk(&mut self) -> XFERCOMPLMSK_W {
        XFERCOMPLMSK_W { w: self }
    }
    #[doc = "Bit 1 - Channel Halted Mask"]
    #[inline(always)]
    pub fn chhltdmsk(&mut self) -> CHHLTDMSK_W {
        CHHLTDMSK_W { w: self }
    }
    #[doc = "Bit 2 - AHB Error Mask"]
    #[inline(always)]
    pub fn ahberrmsk(&mut self) -> AHBERRMSK_W {
        AHBERRMSK_W { w: self }
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt Mask"]
    #[inline(always)]
    pub fn stallmsk(&mut self) -> STALLMSK_W {
        STALLMSK_W { w: self }
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt Mask"]
    #[inline(always)]
    pub fn nakmsk(&mut self) -> NAKMSK_W {
        NAKMSK_W { w: self }
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt Mask"]
    #[inline(always)]
    pub fn ackmsk(&mut self) -> ACKMSK_W {
        ACKMSK_W { w: self }
    }
    #[doc = "Bit 7 - Transaction Error Mask"]
    #[inline(always)]
    pub fn xacterrmsk(&mut self) -> XACTERRMSK_W {
        XACTERRMSK_W { w: self }
    }
    #[doc = "Bit 8 - Babble Error Mask"]
    #[inline(always)]
    pub fn bblerrmsk(&mut self) -> BBLERRMSK_W {
        BBLERRMSK_W { w: self }
    }
    #[doc = "Bit 9 - Frame Overrun Mask"]
    #[inline(always)]
    pub fn frmovrunmsk(&mut self) -> FRMOVRUNMSK_W {
        FRMOVRUNMSK_W { w: self }
    }
    #[doc = "Bit 10 - Data Toggle Error Mask"]
    #[inline(always)]
    pub fn datatglerrmsk(&mut self) -> DATATGLERRMSK_W {
        DATATGLERRMSK_W { w: self }
    }
}
