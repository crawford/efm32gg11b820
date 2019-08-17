#[doc = "Reader of register IRQMASK"]
pub type R = crate::R<u32, super::IRQMASK>;
#[doc = "Writer for register IRQMASK"]
pub type W = crate::W<u32, super::IRQMASK>;
#[doc = "Register IRQMASK `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MODEMFAILMASK`"]
pub type MODEMFAILMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODEMFAILMASK`"]
pub struct MODEMFAILMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MODEMFAILMASK_W<'a> {
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
#[doc = "Reader of field `UNDERFLOWDETMASK`"]
pub type UNDERFLOWDETMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNDERFLOWDETMASK`"]
pub struct UNDERFLOWDETMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERFLOWDETMASK_W<'a> {
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
#[doc = "Reader of field `INDIRECTOPDONEMASK`"]
pub type INDIRECTOPDONEMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INDIRECTOPDONEMASK`"]
pub struct INDIRECTOPDONEMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> INDIRECTOPDONEMASK_W<'a> {
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
#[doc = "Reader of field `INDIRECTREADREJECTMASK`"]
pub type INDIRECTREADREJECTMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INDIRECTREADREJECTMASK`"]
pub struct INDIRECTREADREJECTMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> INDIRECTREADREJECTMASK_W<'a> {
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
#[doc = "Reader of field `PROTWRATTEMPTMASK`"]
pub type PROTWRATTEMPTMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROTWRATTEMPTMASK`"]
pub struct PROTWRATTEMPTMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTWRATTEMPTMASK_W<'a> {
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
#[doc = "Reader of field `ILLEGALACCESSDETMASK`"]
pub type ILLEGALACCESSDETMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ILLEGALACCESSDETMASK`"]
pub struct ILLEGALACCESSDETMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> ILLEGALACCESSDETMASK_W<'a> {
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
#[doc = "Reader of field `INDIRECTXFERLEVELBREACHMASK`"]
pub type INDIRECTXFERLEVELBREACHMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INDIRECTXFERLEVELBREACHMASK`"]
pub struct INDIRECTXFERLEVELBREACHMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> INDIRECTXFERLEVELBREACHMASK_W<'a> {
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
#[doc = "Reader of field `RECVOVERFLOWMASK`"]
pub type RECVOVERFLOWMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RECVOVERFLOWMASK`"]
pub struct RECVOVERFLOWMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RECVOVERFLOWMASK_W<'a> {
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
#[doc = "Reader of field `TXFIFONOTFULLMASK`"]
pub type TXFIFONOTFULLMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFIFONOTFULLMASK`"]
pub struct TXFIFONOTFULLMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFONOTFULLMASK_W<'a> {
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
#[doc = "Reader of field `TXFIFOFULLMASK`"]
pub type TXFIFOFULLMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFIFOFULLMASK`"]
pub struct TXFIFOFULLMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFOFULLMASK_W<'a> {
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
#[doc = "Reader of field `RXFIFONOTEMPTYMASK`"]
pub type RXFIFONOTEMPTYMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFIFONOTEMPTYMASK`"]
pub struct RXFIFONOTEMPTYMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFONOTEMPTYMASK_W<'a> {
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
#[doc = "Reader of field `RXFIFOFULLMASK`"]
pub type RXFIFOFULLMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFIFOFULLMASK`"]
pub struct RXFIFOFULLMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFOFULLMASK_W<'a> {
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
#[doc = "Reader of field `INDRDSRAMFULLMASK`"]
pub type INDRDSRAMFULLMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INDRDSRAMFULLMASK`"]
pub struct INDRDSRAMFULLMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> INDRDSRAMFULLMASK_W<'a> {
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
#[doc = "Reader of field `POLLEXPINTMASK`"]
pub type POLLEXPINTMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POLLEXPINTMASK`"]
pub struct POLLEXPINTMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> POLLEXPINTMASK_W<'a> {
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
#[doc = "Reader of field `STIGREQMASK`"]
pub type STIGREQMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STIGREQMASK`"]
pub struct STIGREQMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> STIGREQMASK_W<'a> {
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
#[doc = "Reader of field `RXCRCDATAERRMASK`"]
pub type RXCRCDATAERRMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXCRCDATAERRMASK`"]
pub struct RXCRCDATAERRMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCRCDATAERRMASK_W<'a> {
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
#[doc = "Reader of field `RXCRCDATAVALMASK`"]
pub type RXCRCDATAVALMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXCRCDATAVALMASK`"]
pub struct RXCRCDATAVALMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCRCDATAVALMASK_W<'a> {
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
#[doc = "Reader of field `TXCRCCHUNKBRKMASK`"]
pub type TXCRCCHUNKBRKMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXCRCCHUNKBRKMASK`"]
pub struct TXCRCCHUNKBRKMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCRCCHUNKBRKMASK_W<'a> {
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
    #[doc = "Bit 0 - Mode M Failure Mask"]
    #[inline(always)]
    pub fn modemfailmask(&self) -> MODEMFAILMASK_R {
        MODEMFAILMASK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Underflow Detected Mask"]
    #[inline(always)]
    pub fn underflowdetmask(&self) -> UNDERFLOWDETMASK_R {
        UNDERFLOWDETMASK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indirect Complete Mask"]
    #[inline(always)]
    pub fn indirectopdonemask(&self) -> INDIRECTOPDONEMASK_R {
        INDIRECTOPDONEMASK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Indirect Read Reject Mask"]
    #[inline(always)]
    pub fn indirectreadrejectmask(&self) -> INDIRECTREADREJECTMASK_R {
        INDIRECTREADREJECTMASK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Protected Area Write Attempt Mask"]
    #[inline(always)]
    pub fn protwrattemptmask(&self) -> PROTWRATTEMPTMASK_R {
        PROTWRATTEMPTMASK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Illegal Access Detected Mask"]
    #[inline(always)]
    pub fn illegalaccessdetmask(&self) -> ILLEGALACCESSDETMASK_R {
        ILLEGALACCESSDETMASK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transfer Watermark Breach Mask"]
    #[inline(always)]
    pub fn indirectxferlevelbreachmask(&self) -> INDIRECTXFERLEVELBREACHMASK_R {
        INDIRECTXFERLEVELBREACHMASK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Receive Overflow Mask"]
    #[inline(always)]
    pub fn recvoverflowmask(&self) -> RECVOVERFLOWMASK_R {
        RECVOVERFLOWMASK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Small TX FIFO Not Full Mask"]
    #[inline(always)]
    pub fn txfifonotfullmask(&self) -> TXFIFONOTFULLMASK_R {
        TXFIFONOTFULLMASK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Small TX FIFO Full Mask"]
    #[inline(always)]
    pub fn txfifofullmask(&self) -> TXFIFOFULLMASK_R {
        TXFIFOFULLMASK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Small RX FIFO Not Empty Mask"]
    #[inline(always)]
    pub fn rxfifonotemptymask(&self) -> RXFIFONOTEMPTYMASK_R {
        RXFIFONOTEMPTYMASK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Small RX FIFO Full Mask"]
    #[inline(always)]
    pub fn rxfifofullmask(&self) -> RXFIFOFULLMASK_R {
        RXFIFOFULLMASK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Indirect Read Partition Overflow Mask"]
    #[inline(always)]
    pub fn indrdsramfullmask(&self) -> INDRDSRAMFULLMASK_R {
        INDRDSRAMFULLMASK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Polling Expiration Detected Mask"]
    #[inline(always)]
    pub fn pollexpintmask(&self) -> POLLEXPINTMASK_R {
        POLLEXPINTMASK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - STIG Request Completion Mask"]
    #[inline(always)]
    pub fn stigreqmask(&self) -> STIGREQMASK_R {
        STIGREQMASK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RX CRC Data Error Mask"]
    #[inline(always)]
    pub fn rxcrcdataerrmask(&self) -> RXCRCDATAERRMASK_R {
        RXCRCDATAERRMASK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RX CRC Data Valid Mask"]
    #[inline(always)]
    pub fn rxcrcdatavalmask(&self) -> RXCRCDATAVALMASK_R {
        RXCRCDATAVALMASK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TX CRC Chunk Was Broken Mask"]
    #[inline(always)]
    pub fn txcrcchunkbrkmask(&self) -> TXCRCCHUNKBRKMASK_R {
        TXCRCCHUNKBRKMASK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mode M Failure Mask"]
    #[inline(always)]
    pub fn modemfailmask(&mut self) -> MODEMFAILMASK_W {
        MODEMFAILMASK_W { w: self }
    }
    #[doc = "Bit 1 - Underflow Detected Mask"]
    #[inline(always)]
    pub fn underflowdetmask(&mut self) -> UNDERFLOWDETMASK_W {
        UNDERFLOWDETMASK_W { w: self }
    }
    #[doc = "Bit 2 - Indirect Complete Mask"]
    #[inline(always)]
    pub fn indirectopdonemask(&mut self) -> INDIRECTOPDONEMASK_W {
        INDIRECTOPDONEMASK_W { w: self }
    }
    #[doc = "Bit 3 - Indirect Read Reject Mask"]
    #[inline(always)]
    pub fn indirectreadrejectmask(&mut self) -> INDIRECTREADREJECTMASK_W {
        INDIRECTREADREJECTMASK_W { w: self }
    }
    #[doc = "Bit 4 - Protected Area Write Attempt Mask"]
    #[inline(always)]
    pub fn protwrattemptmask(&mut self) -> PROTWRATTEMPTMASK_W {
        PROTWRATTEMPTMASK_W { w: self }
    }
    #[doc = "Bit 5 - Illegal Access Detected Mask"]
    #[inline(always)]
    pub fn illegalaccessdetmask(&mut self) -> ILLEGALACCESSDETMASK_W {
        ILLEGALACCESSDETMASK_W { w: self }
    }
    #[doc = "Bit 6 - Transfer Watermark Breach Mask"]
    #[inline(always)]
    pub fn indirectxferlevelbreachmask(&mut self) -> INDIRECTXFERLEVELBREACHMASK_W {
        INDIRECTXFERLEVELBREACHMASK_W { w: self }
    }
    #[doc = "Bit 7 - Receive Overflow Mask"]
    #[inline(always)]
    pub fn recvoverflowmask(&mut self) -> RECVOVERFLOWMASK_W {
        RECVOVERFLOWMASK_W { w: self }
    }
    #[doc = "Bit 8 - Small TX FIFO Not Full Mask"]
    #[inline(always)]
    pub fn txfifonotfullmask(&mut self) -> TXFIFONOTFULLMASK_W {
        TXFIFONOTFULLMASK_W { w: self }
    }
    #[doc = "Bit 9 - Small TX FIFO Full Mask"]
    #[inline(always)]
    pub fn txfifofullmask(&mut self) -> TXFIFOFULLMASK_W {
        TXFIFOFULLMASK_W { w: self }
    }
    #[doc = "Bit 10 - Small RX FIFO Not Empty Mask"]
    #[inline(always)]
    pub fn rxfifonotemptymask(&mut self) -> RXFIFONOTEMPTYMASK_W {
        RXFIFONOTEMPTYMASK_W { w: self }
    }
    #[doc = "Bit 11 - Small RX FIFO Full Mask"]
    #[inline(always)]
    pub fn rxfifofullmask(&mut self) -> RXFIFOFULLMASK_W {
        RXFIFOFULLMASK_W { w: self }
    }
    #[doc = "Bit 12 - Indirect Read Partition Overflow Mask"]
    #[inline(always)]
    pub fn indrdsramfullmask(&mut self) -> INDRDSRAMFULLMASK_W {
        INDRDSRAMFULLMASK_W { w: self }
    }
    #[doc = "Bit 13 - Polling Expiration Detected Mask"]
    #[inline(always)]
    pub fn pollexpintmask(&mut self) -> POLLEXPINTMASK_W {
        POLLEXPINTMASK_W { w: self }
    }
    #[doc = "Bit 14 - STIG Request Completion Mask"]
    #[inline(always)]
    pub fn stigreqmask(&mut self) -> STIGREQMASK_W {
        STIGREQMASK_W { w: self }
    }
    #[doc = "Bit 16 - RX CRC Data Error Mask"]
    #[inline(always)]
    pub fn rxcrcdataerrmask(&mut self) -> RXCRCDATAERRMASK_W {
        RXCRCDATAERRMASK_W { w: self }
    }
    #[doc = "Bit 17 - RX CRC Data Valid Mask"]
    #[inline(always)]
    pub fn rxcrcdatavalmask(&mut self) -> RXCRCDATAVALMASK_W {
        RXCRCDATAVALMASK_W { w: self }
    }
    #[doc = "Bit 18 - TX CRC Chunk Was Broken Mask"]
    #[inline(always)]
    pub fn txcrcchunkbrkmask(&mut self) -> TXCRCCHUNKBRKMASK_W {
        TXCRCCHUNKBRKMASK_W { w: self }
    }
}
