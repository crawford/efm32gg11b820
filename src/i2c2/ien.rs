#[doc = "Reader of register IEN"]
pub type R = crate::R<u32, super::IEN>;
#[doc = "Writer for register IEN"]
pub type W = crate::W<u32, super::IEN>;
#[doc = "Register IEN `reset()`'s with value 0"]
impl crate::ResetValue for super::IEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
#[doc = "Reader of field `RSTART`"]
pub type RSTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTART`"]
pub struct RSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTART_W<'a> {
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
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
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
#[doc = "Reader of field `TXC`"]
pub type TXC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXC`"]
pub struct TXC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXC_W<'a> {
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
#[doc = "Reader of field `TXBL`"]
pub type TXBL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXBL`"]
pub struct TXBL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBL_W<'a> {
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
#[doc = "Reader of field `RXDATAV`"]
pub type RXDATAV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXDATAV`"]
pub struct RXDATAV_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDATAV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `NACK`"]
pub type NACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NACK`"]
pub struct NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> NACK_W<'a> {
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
#[doc = "Reader of field `MSTOP`"]
pub type MSTOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSTOP`"]
pub struct MSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTOP_W<'a> {
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
#[doc = "Reader of field `ARBLOST`"]
pub type ARBLOST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARBLOST`"]
pub struct ARBLOST_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBLOST_W<'a> {
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
#[doc = "Reader of field `BUSERR`"]
pub type BUSERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSERR`"]
pub struct BUSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSERR_W<'a> {
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
#[doc = "Reader of field `BUSHOLD`"]
pub type BUSHOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSHOLD`"]
pub struct BUSHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSHOLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `TXOF`"]
pub type TXOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXOF`"]
pub struct TXOF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOF_W<'a> {
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
#[doc = "Reader of field `RXUF`"]
pub type RXUF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXUF`"]
pub struct RXUF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `BITO`"]
pub type BITO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BITO`"]
pub struct BITO_W<'a> {
    w: &'a mut W,
}
impl<'a> BITO_W<'a> {
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
#[doc = "Reader of field `CLTO`"]
pub type CLTO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLTO`"]
pub struct CLTO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLTO_W<'a> {
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
#[doc = "Reader of field `SSTOP`"]
pub type SSTOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSTOP`"]
pub struct SSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SSTOP_W<'a> {
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
#[doc = "Reader of field `RXFULL`"]
pub type RXFULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFULL`"]
pub struct RXFULL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFULL_W<'a> {
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
#[doc = "Reader of field `CLERR`"]
pub type CLERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLERR`"]
pub struct CLERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLERR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - START Interrupt Enable"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RSTART Interrupt Enable"]
    #[inline(always)]
    pub fn rstart(&self) -> RSTART_R {
        RSTART_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADDR Interrupt Enable"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TXC Interrupt Enable"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TXBL Interrupt Enable"]
    #[inline(always)]
    pub fn txbl(&self) -> TXBL_R {
        TXBL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RXDATAV Interrupt Enable"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RXDATAV_R {
        RXDATAV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ACK Interrupt Enable"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - NACK Interrupt Enable"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MSTOP Interrupt Enable"]
    #[inline(always)]
    pub fn mstop(&self) -> MSTOP_R {
        MSTOP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ARBLOST Interrupt Enable"]
    #[inline(always)]
    pub fn arblost(&self) -> ARBLOST_R {
        ARBLOST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - BUSERR Interrupt Enable"]
    #[inline(always)]
    pub fn buserr(&self) -> BUSERR_R {
        BUSERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BUSHOLD Interrupt Enable"]
    #[inline(always)]
    pub fn bushold(&self) -> BUSHOLD_R {
        BUSHOLD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TXOF Interrupt Enable"]
    #[inline(always)]
    pub fn txof(&self) -> TXOF_R {
        TXOF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RXUF Interrupt Enable"]
    #[inline(always)]
    pub fn rxuf(&self) -> RXUF_R {
        RXUF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - BITO Interrupt Enable"]
    #[inline(always)]
    pub fn bito(&self) -> BITO_R {
        BITO_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - CLTO Interrupt Enable"]
    #[inline(always)]
    pub fn clto(&self) -> CLTO_R {
        CLTO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SSTOP Interrupt Enable"]
    #[inline(always)]
    pub fn sstop(&self) -> SSTOP_R {
        SSTOP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RXFULL Interrupt Enable"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CLERR Interrupt Enable"]
    #[inline(always)]
    pub fn clerr(&self) -> CLERR_R {
        CLERR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - START Interrupt Enable"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 1 - RSTART Interrupt Enable"]
    #[inline(always)]
    pub fn rstart(&mut self) -> RSTART_W {
        RSTART_W { w: self }
    }
    #[doc = "Bit 2 - ADDR Interrupt Enable"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Bit 3 - TXC Interrupt Enable"]
    #[inline(always)]
    pub fn txc(&mut self) -> TXC_W {
        TXC_W { w: self }
    }
    #[doc = "Bit 4 - TXBL Interrupt Enable"]
    #[inline(always)]
    pub fn txbl(&mut self) -> TXBL_W {
        TXBL_W { w: self }
    }
    #[doc = "Bit 5 - RXDATAV Interrupt Enable"]
    #[inline(always)]
    pub fn rxdatav(&mut self) -> RXDATAV_W {
        RXDATAV_W { w: self }
    }
    #[doc = "Bit 6 - ACK Interrupt Enable"]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W {
        ACK_W { w: self }
    }
    #[doc = "Bit 7 - NACK Interrupt Enable"]
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W {
        NACK_W { w: self }
    }
    #[doc = "Bit 8 - MSTOP Interrupt Enable"]
    #[inline(always)]
    pub fn mstop(&mut self) -> MSTOP_W {
        MSTOP_W { w: self }
    }
    #[doc = "Bit 9 - ARBLOST Interrupt Enable"]
    #[inline(always)]
    pub fn arblost(&mut self) -> ARBLOST_W {
        ARBLOST_W { w: self }
    }
    #[doc = "Bit 10 - BUSERR Interrupt Enable"]
    #[inline(always)]
    pub fn buserr(&mut self) -> BUSERR_W {
        BUSERR_W { w: self }
    }
    #[doc = "Bit 11 - BUSHOLD Interrupt Enable"]
    #[inline(always)]
    pub fn bushold(&mut self) -> BUSHOLD_W {
        BUSHOLD_W { w: self }
    }
    #[doc = "Bit 12 - TXOF Interrupt Enable"]
    #[inline(always)]
    pub fn txof(&mut self) -> TXOF_W {
        TXOF_W { w: self }
    }
    #[doc = "Bit 13 - RXUF Interrupt Enable"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RXUF_W {
        RXUF_W { w: self }
    }
    #[doc = "Bit 14 - BITO Interrupt Enable"]
    #[inline(always)]
    pub fn bito(&mut self) -> BITO_W {
        BITO_W { w: self }
    }
    #[doc = "Bit 15 - CLTO Interrupt Enable"]
    #[inline(always)]
    pub fn clto(&mut self) -> CLTO_W {
        CLTO_W { w: self }
    }
    #[doc = "Bit 16 - SSTOP Interrupt Enable"]
    #[inline(always)]
    pub fn sstop(&mut self) -> SSTOP_W {
        SSTOP_W { w: self }
    }
    #[doc = "Bit 17 - RXFULL Interrupt Enable"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RXFULL_W {
        RXFULL_W { w: self }
    }
    #[doc = "Bit 18 - CLERR Interrupt Enable"]
    #[inline(always)]
    pub fn clerr(&mut self) -> CLERR_W {
        CLERR_W { w: self }
    }
}
