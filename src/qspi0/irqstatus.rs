#[doc = "Reader of register IRQSTATUS"]
pub type R = crate::R<u32, super::IRQSTATUS>;
#[doc = "Writer for register IRQSTATUS"]
pub type W = crate::W<u32, super::IRQSTATUS>;
#[doc = "Register IRQSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MODEMFAIL`"]
pub type MODEMFAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODEMFAIL`"]
pub struct MODEMFAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> MODEMFAIL_W<'a> {
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
#[doc = "Reader of field `UNDERFLOWDET`"]
pub type UNDERFLOWDET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNDERFLOWDET`"]
pub struct UNDERFLOWDET_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERFLOWDET_W<'a> {
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
#[doc = "Reader of field `INDIRECTOPDONE`"]
pub type INDIRECTOPDONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INDIRECTOPDONE`"]
pub struct INDIRECTOPDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> INDIRECTOPDONE_W<'a> {
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
#[doc = "Reader of field `INDIRECTREADREJECT`"]
pub type INDIRECTREADREJECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INDIRECTREADREJECT`"]
pub struct INDIRECTREADREJECT_W<'a> {
    w: &'a mut W,
}
impl<'a> INDIRECTREADREJECT_W<'a> {
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
#[doc = "Reader of field `PROTWRATTEMPT`"]
pub type PROTWRATTEMPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROTWRATTEMPT`"]
pub struct PROTWRATTEMPT_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTWRATTEMPT_W<'a> {
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
#[doc = "Reader of field `ILLEGALACCESSDET`"]
pub type ILLEGALACCESSDET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ILLEGALACCESSDET`"]
pub struct ILLEGALACCESSDET_W<'a> {
    w: &'a mut W,
}
impl<'a> ILLEGALACCESSDET_W<'a> {
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
#[doc = "Reader of field `INDIRECTXFERLEVELBREACH`"]
pub type INDIRECTXFERLEVELBREACH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INDIRECTXFERLEVELBREACH`"]
pub struct INDIRECTXFERLEVELBREACH_W<'a> {
    w: &'a mut W,
}
impl<'a> INDIRECTXFERLEVELBREACH_W<'a> {
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
#[doc = "Reader of field `RECVOVERFLOW`"]
pub type RECVOVERFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RECVOVERFLOW`"]
pub struct RECVOVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> RECVOVERFLOW_W<'a> {
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
#[doc = "Reader of field `TXFIFONOTFULL`"]
pub type TXFIFONOTFULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFIFONOTFULL`"]
pub struct TXFIFONOTFULL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFONOTFULL_W<'a> {
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
#[doc = "Reader of field `TXFIFOFULL`"]
pub type TXFIFOFULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFIFOFULL`"]
pub struct TXFIFOFULL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFOFULL_W<'a> {
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
#[doc = "Reader of field `RXFIFONOTEMPTY`"]
pub type RXFIFONOTEMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFIFONOTEMPTY`"]
pub struct RXFIFONOTEMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFONOTEMPTY_W<'a> {
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
#[doc = "Reader of field `RXFIFOFULL`"]
pub type RXFIFOFULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFIFOFULL`"]
pub struct RXFIFOFULL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFOFULL_W<'a> {
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
#[doc = "Reader of field `INDRDSRAMFULL`"]
pub type INDRDSRAMFULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INDRDSRAMFULL`"]
pub struct INDRDSRAMFULL_W<'a> {
    w: &'a mut W,
}
impl<'a> INDRDSRAMFULL_W<'a> {
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
#[doc = "Reader of field `POLLEXPINT`"]
pub type POLLEXPINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POLLEXPINT`"]
pub struct POLLEXPINT_W<'a> {
    w: &'a mut W,
}
impl<'a> POLLEXPINT_W<'a> {
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
#[doc = "Reader of field `STIGREQINT`"]
pub type STIGREQINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STIGREQINT`"]
pub struct STIGREQINT_W<'a> {
    w: &'a mut W,
}
impl<'a> STIGREQINT_W<'a> {
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
#[doc = "Reader of field `RXCRCDATAERR`"]
pub type RXCRCDATAERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXCRCDATAERR`"]
pub struct RXCRCDATAERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCRCDATAERR_W<'a> {
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
#[doc = "Reader of field `RXCRCDATAVAL`"]
pub type RXCRCDATAVAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXCRCDATAVAL`"]
pub struct RXCRCDATAVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCRCDATAVAL_W<'a> {
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
#[doc = "Reader of field `TXCRCCHUNKBRK`"]
pub type TXCRCCHUNKBRK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXCRCCHUNKBRK`"]
pub struct TXCRCCHUNKBRK_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCRCCHUNKBRK_W<'a> {
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
    #[doc = "Bit 0 - Mode M Failure"]
    #[inline(always)]
    pub fn modemfail(&self) -> MODEMFAIL_R {
        MODEMFAIL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Underflow Detected"]
    #[inline(always)]
    pub fn underflowdet(&self) -> UNDERFLOWDET_R {
        UNDERFLOWDET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indirect Operation Complete"]
    #[inline(always)]
    pub fn indirectopdone(&self) -> INDIRECTOPDONE_R {
        INDIRECTOPDONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Indirect Operation Was Requested but Could Not Be Accepted"]
    #[inline(always)]
    pub fn indirectreadreject(&self) -> INDIRECTREADREJECT_R {
        INDIRECTREADREJECT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write to Protected Area Was Attempted and Rejected"]
    #[inline(always)]
    pub fn protwrattempt(&self) -> PROTWRATTEMPT_R {
        PROTWRATTEMPT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Illegal Memory Access Has Been Detected"]
    #[inline(always)]
    pub fn illegalaccessdet(&self) -> ILLEGALACCESSDET_R {
        ILLEGALACCESSDET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Indirect Transfer Watermark Level Breached"]
    #[inline(always)]
    pub fn indirectxferlevelbreach(&self) -> INDIRECTXFERLEVELBREACH_R {
        INDIRECTXFERLEVELBREACH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Receive Overflow"]
    #[inline(always)]
    pub fn recvoverflow(&self) -> RECVOVERFLOW_R {
        RECVOVERFLOW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Small TX FIFO Not Full"]
    #[inline(always)]
    pub fn txfifonotfull(&self) -> TXFIFONOTFULL_R {
        TXFIFONOTFULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Small TX FIFO Full"]
    #[inline(always)]
    pub fn txfifofull(&self) -> TXFIFOFULL_R {
        TXFIFOFULL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Small RX FIFO Not Empty"]
    #[inline(always)]
    pub fn rxfifonotempty(&self) -> RXFIFONOTEMPTY_R {
        RXFIFONOTEMPTY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Small RX FIFO Full"]
    #[inline(always)]
    pub fn rxfifofull(&self) -> RXFIFOFULL_R {
        RXFIFOFULL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Indirect Read Partition Overflow"]
    #[inline(always)]
    pub fn indrdsramfull(&self) -> INDRDSRAMFULL_R {
        INDRDSRAMFULL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - The Maximum Number of Programmed Polls Cycles is Expired"]
    #[inline(always)]
    pub fn pollexpint(&self) -> POLLEXPINT_R {
        POLLEXPINT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - The Controller is Ready for Getting Another STIG Request"]
    #[inline(always)]
    pub fn stigreqint(&self) -> STIGREQINT_R {
        STIGREQINT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RX CRC Data Error"]
    #[inline(always)]
    pub fn rxcrcdataerr(&self) -> RXCRCDATAERR_R {
        RXCRCDATAERR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RX CRC Data Valid"]
    #[inline(always)]
    pub fn rxcrcdataval(&self) -> RXCRCDATAVAL_R {
        RXCRCDATAVAL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TX CRC Chunk Was Broken"]
    #[inline(always)]
    pub fn txcrcchunkbrk(&self) -> TXCRCCHUNKBRK_R {
        TXCRCCHUNKBRK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mode M Failure"]
    #[inline(always)]
    pub fn modemfail(&mut self) -> MODEMFAIL_W {
        MODEMFAIL_W { w: self }
    }
    #[doc = "Bit 1 - Underflow Detected"]
    #[inline(always)]
    pub fn underflowdet(&mut self) -> UNDERFLOWDET_W {
        UNDERFLOWDET_W { w: self }
    }
    #[doc = "Bit 2 - Indirect Operation Complete"]
    #[inline(always)]
    pub fn indirectopdone(&mut self) -> INDIRECTOPDONE_W {
        INDIRECTOPDONE_W { w: self }
    }
    #[doc = "Bit 3 - Indirect Operation Was Requested but Could Not Be Accepted"]
    #[inline(always)]
    pub fn indirectreadreject(&mut self) -> INDIRECTREADREJECT_W {
        INDIRECTREADREJECT_W { w: self }
    }
    #[doc = "Bit 4 - Write to Protected Area Was Attempted and Rejected"]
    #[inline(always)]
    pub fn protwrattempt(&mut self) -> PROTWRATTEMPT_W {
        PROTWRATTEMPT_W { w: self }
    }
    #[doc = "Bit 5 - Illegal Memory Access Has Been Detected"]
    #[inline(always)]
    pub fn illegalaccessdet(&mut self) -> ILLEGALACCESSDET_W {
        ILLEGALACCESSDET_W { w: self }
    }
    #[doc = "Bit 6 - Indirect Transfer Watermark Level Breached"]
    #[inline(always)]
    pub fn indirectxferlevelbreach(&mut self) -> INDIRECTXFERLEVELBREACH_W {
        INDIRECTXFERLEVELBREACH_W { w: self }
    }
    #[doc = "Bit 7 - Receive Overflow"]
    #[inline(always)]
    pub fn recvoverflow(&mut self) -> RECVOVERFLOW_W {
        RECVOVERFLOW_W { w: self }
    }
    #[doc = "Bit 8 - Small TX FIFO Not Full"]
    #[inline(always)]
    pub fn txfifonotfull(&mut self) -> TXFIFONOTFULL_W {
        TXFIFONOTFULL_W { w: self }
    }
    #[doc = "Bit 9 - Small TX FIFO Full"]
    #[inline(always)]
    pub fn txfifofull(&mut self) -> TXFIFOFULL_W {
        TXFIFOFULL_W { w: self }
    }
    #[doc = "Bit 10 - Small RX FIFO Not Empty"]
    #[inline(always)]
    pub fn rxfifonotempty(&mut self) -> RXFIFONOTEMPTY_W {
        RXFIFONOTEMPTY_W { w: self }
    }
    #[doc = "Bit 11 - Small RX FIFO Full"]
    #[inline(always)]
    pub fn rxfifofull(&mut self) -> RXFIFOFULL_W {
        RXFIFOFULL_W { w: self }
    }
    #[doc = "Bit 12 - Indirect Read Partition Overflow"]
    #[inline(always)]
    pub fn indrdsramfull(&mut self) -> INDRDSRAMFULL_W {
        INDRDSRAMFULL_W { w: self }
    }
    #[doc = "Bit 13 - The Maximum Number of Programmed Polls Cycles is Expired"]
    #[inline(always)]
    pub fn pollexpint(&mut self) -> POLLEXPINT_W {
        POLLEXPINT_W { w: self }
    }
    #[doc = "Bit 14 - The Controller is Ready for Getting Another STIG Request"]
    #[inline(always)]
    pub fn stigreqint(&mut self) -> STIGREQINT_W {
        STIGREQINT_W { w: self }
    }
    #[doc = "Bit 16 - RX CRC Data Error"]
    #[inline(always)]
    pub fn rxcrcdataerr(&mut self) -> RXCRCDATAERR_W {
        RXCRCDATAERR_W { w: self }
    }
    #[doc = "Bit 17 - RX CRC Data Valid"]
    #[inline(always)]
    pub fn rxcrcdataval(&mut self) -> RXCRCDATAVAL_W {
        RXCRCDATAVAL_W { w: self }
    }
    #[doc = "Bit 18 - TX CRC Chunk Was Broken"]
    #[inline(always)]
    pub fn txcrcchunkbrk(&mut self) -> TXCRCCHUNKBRK_W {
        TXCRCCHUNKBRK_W { w: self }
    }
}
