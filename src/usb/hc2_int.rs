#[doc = "Reader of register HC2_INT"]
pub type R = crate::R<u32, super::HC2_INT>;
#[doc = "Writer for register HC2_INT"]
pub type W = crate::W<u32, super::HC2_INT>;
#[doc = "Register HC2_INT `reset()`'s with value 0"]
impl crate::ResetValue for super::HC2_INT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XFERCOMPL`"]
pub type XFERCOMPL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XFERCOMPL`"]
pub struct XFERCOMPL_W<'a> {
    w: &'a mut W,
}
impl<'a> XFERCOMPL_W<'a> {
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
#[doc = "Reader of field `CHHLTD`"]
pub type CHHLTD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHHLTD`"]
pub struct CHHLTD_W<'a> {
    w: &'a mut W,
}
impl<'a> CHHLTD_W<'a> {
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
#[doc = "Reader of field `AHBERR`"]
pub type AHBERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHBERR`"]
pub struct AHBERR_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBERR_W<'a> {
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
#[doc = "Reader of field `STALL`"]
pub type STALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STALL`"]
pub struct STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_W<'a> {
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
#[doc = "Reader of field `NAK`"]
pub type NAK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NAK`"]
pub struct NAK_W<'a> {
    w: &'a mut W,
}
impl<'a> NAK_W<'a> {
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
#[doc = "Reader of field `ACK`"]
pub type ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACK`"]
pub struct ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_W<'a> {
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
#[doc = "Reader of field `XACTERR`"]
pub type XACTERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XACTERR`"]
pub struct XACTERR_W<'a> {
    w: &'a mut W,
}
impl<'a> XACTERR_W<'a> {
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
#[doc = "Reader of field `BBLERR`"]
pub type BBLERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BBLERR`"]
pub struct BBLERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BBLERR_W<'a> {
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
#[doc = "Reader of field `FRMOVRUN`"]
pub type FRMOVRUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRMOVRUN`"]
pub struct FRMOVRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMOVRUN_W<'a> {
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
#[doc = "Reader of field `DATATGLERR`"]
pub type DATATGLERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATATGLERR`"]
pub struct DATATGLERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATATGLERR_W<'a> {
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
    #[doc = "Bit 0 - Transfer Completed"]
    #[inline(always)]
    pub fn xfercompl(&self) -> XFERCOMPL_R {
        XFERCOMPL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel Halted"]
    #[inline(always)]
    pub fn chhltd(&self) -> CHHLTD_R {
        CHHLTD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Response Received Interrupt"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Response Received Interrupt"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Response Received/Transmitted Interrupt"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transaction Error"]
    #[inline(always)]
    pub fn xacterr(&self) -> XACTERR_R {
        XACTERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Babble Error"]
    #[inline(always)]
    pub fn bblerr(&self) -> BBLERR_R {
        BBLERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn frmovrun(&self) -> FRMOVRUN_R {
        FRMOVRUN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn datatglerr(&self) -> DATATGLERR_R {
        DATATGLERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed"]
    #[inline(always)]
    pub fn xfercompl(&mut self) -> XFERCOMPL_W {
        XFERCOMPL_W { w: self }
    }
    #[doc = "Bit 1 - Channel Halted"]
    #[inline(always)]
    pub fn chhltd(&mut self) -> CHHLTD_W {
        CHHLTD_W { w: self }
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&mut self) -> AHBERR_W {
        AHBERR_W { w: self }
    }
    #[doc = "Bit 3 - Response Received Interrupt"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
    #[doc = "Bit 4 - Response Received Interrupt"]
    #[inline(always)]
    pub fn nak(&mut self) -> NAK_W {
        NAK_W { w: self }
    }
    #[doc = "Bit 5 - Response Received/Transmitted Interrupt"]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W {
        ACK_W { w: self }
    }
    #[doc = "Bit 7 - Transaction Error"]
    #[inline(always)]
    pub fn xacterr(&mut self) -> XACTERR_W {
        XACTERR_W { w: self }
    }
    #[doc = "Bit 8 - Babble Error"]
    #[inline(always)]
    pub fn bblerr(&mut self) -> BBLERR_W {
        BBLERR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn frmovrun(&mut self) -> FRMOVRUN_W {
        FRMOVRUN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn datatglerr(&mut self) -> DATATGLERR_W {
        DATATGLERR_W { w: self }
    }
}
