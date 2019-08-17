#[doc = "Reader of register IFCR"]
pub type R = crate::R<u32, super::IFCR>;
#[doc = "Writer for register IFCR"]
pub type W = crate::W<u32, super::IFCR>;
#[doc = "Register IFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::IFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MNGMNTDONE`"]
pub type MNGMNTDONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MNGMNTDONE`"]
pub struct MNGMNTDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> MNGMNTDONE_W<'a> {
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
#[doc = "Reader of field `RXCMPLT`"]
pub type RXCMPLT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXCMPLT`"]
pub struct RXCMPLT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCMPLT_W<'a> {
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
#[doc = "Reader of field `RXUSEDBITREAD`"]
pub type RXUSEDBITREAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXUSEDBITREAD`"]
pub struct RXUSEDBITREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUSEDBITREAD_W<'a> {
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
#[doc = "Reader of field `TXUSEDBITREAD`"]
pub type TXUSEDBITREAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXUSEDBITREAD`"]
pub struct TXUSEDBITREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUSEDBITREAD_W<'a> {
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
#[doc = "Reader of field `TXUNDERRUN`"]
pub type TXUNDERRUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXUNDERRUN`"]
pub struct TXUNDERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNDERRUN_W<'a> {
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
#[doc = "Reader of field `RTRYLMTORLATECOL`"]
pub type RTRYLMTORLATECOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTRYLMTORLATECOL`"]
pub struct RTRYLMTORLATECOL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTRYLMTORLATECOL_W<'a> {
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
#[doc = "Reader of field `AMBAERR`"]
pub type AMBAERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AMBAERR`"]
pub struct AMBAERR_W<'a> {
    w: &'a mut W,
}
impl<'a> AMBAERR_W<'a> {
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
#[doc = "Reader of field `TXCMPLT`"]
pub type TXCMPLT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXCMPLT`"]
pub struct TXCMPLT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCMPLT_W<'a> {
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
#[doc = "Reader of field `RXOVERRUN`"]
pub type RXOVERRUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXOVERRUN`"]
pub struct RXOVERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVERRUN_W<'a> {
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
#[doc = "Reader of field `RESPNOTOK`"]
pub type RESPNOTOK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESPNOTOK`"]
pub struct RESPNOTOK_W<'a> {
    w: &'a mut W,
}
impl<'a> RESPNOTOK_W<'a> {
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
#[doc = "Reader of field `NONZEROPFRMQUANT`"]
pub type NONZEROPFRMQUANT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NONZEROPFRMQUANT`"]
pub struct NONZEROPFRMQUANT_W<'a> {
    w: &'a mut W,
}
impl<'a> NONZEROPFRMQUANT_W<'a> {
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
#[doc = "Reader of field `PAUSETIMEZERO`"]
pub type PAUSETIMEZERO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAUSETIMEZERO`"]
pub struct PAUSETIMEZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSETIMEZERO_W<'a> {
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
#[doc = "Reader of field `PFRMTX`"]
pub type PFRMTX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PFRMTX`"]
pub struct PFRMTX_W<'a> {
    w: &'a mut W,
}
impl<'a> PFRMTX_W<'a> {
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
#[doc = "Reader of field `PTPDLYREQFRMRX`"]
pub type PTPDLYREQFRMRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTPDLYREQFRMRX`"]
pub struct PTPDLYREQFRMRX_W<'a> {
    w: &'a mut W,
}
impl<'a> PTPDLYREQFRMRX_W<'a> {
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
#[doc = "Reader of field `PTPSYNCFRMRX`"]
pub type PTPSYNCFRMRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTPSYNCFRMRX`"]
pub struct PTPSYNCFRMRX_W<'a> {
    w: &'a mut W,
}
impl<'a> PTPSYNCFRMRX_W<'a> {
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
#[doc = "Reader of field `PTPDLYREQFRMTX`"]
pub type PTPDLYREQFRMTX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTPDLYREQFRMTX`"]
pub struct PTPDLYREQFRMTX_W<'a> {
    w: &'a mut W,
}
impl<'a> PTPDLYREQFRMTX_W<'a> {
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
#[doc = "Reader of field `PTPSYNCFRMTX`"]
pub type PTPSYNCFRMTX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTPSYNCFRMTX`"]
pub struct PTPSYNCFRMTX_W<'a> {
    w: &'a mut W,
}
impl<'a> PTPSYNCFRMTX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `PTPPDLYREQFRMRX`"]
pub type PTPPDLYREQFRMRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTPPDLYREQFRMRX`"]
pub struct PTPPDLYREQFRMRX_W<'a> {
    w: &'a mut W,
}
impl<'a> PTPPDLYREQFRMRX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `PTPPDLYRESPFRMRX`"]
pub type PTPPDLYRESPFRMRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTPPDLYRESPFRMRX`"]
pub struct PTPPDLYRESPFRMRX_W<'a> {
    w: &'a mut W,
}
impl<'a> PTPPDLYRESPFRMRX_W<'a> {
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
#[doc = "Reader of field `PTPPDLYREQFRMTX`"]
pub type PTPPDLYREQFRMTX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTPPDLYREQFRMTX`"]
pub struct PTPPDLYREQFRMTX_W<'a> {
    w: &'a mut W,
}
impl<'a> PTPPDLYREQFRMTX_W<'a> {
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
#[doc = "Reader of field `PTPPDLYRESPFRMTX`"]
pub type PTPPDLYRESPFRMTX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTPPDLYRESPFRMTX`"]
pub struct PTPPDLYRESPFRMTX_W<'a> {
    w: &'a mut W,
}
impl<'a> PTPPDLYRESPFRMTX_W<'a> {
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
#[doc = "Reader of field `TSUSECREGINCR`"]
pub type TSUSECREGINCR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSUSECREGINCR`"]
pub struct TSUSECREGINCR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSUSECREGINCR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `RXLPIINDC`"]
pub type RXLPIINDC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXLPIINDC`"]
pub struct RXLPIINDC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXLPIINDC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `WOLEVNTRX`"]
pub type WOLEVNTRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WOLEVNTRX`"]
pub struct WOLEVNTRX_W<'a> {
    w: &'a mut W,
}
impl<'a> WOLEVNTRX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `TSUTIMERCOMP`"]
pub type TSUTIMERCOMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSUTIMERCOMP`"]
pub struct TSUTIMERCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TSUTIMERCOMP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Management frame sent"]
    #[inline(always)]
    pub fn mngmntdone(&self) -> MNGMNTDONE_R {
        MNGMNTDONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive complete"]
    #[inline(always)]
    pub fn rxcmplt(&self) -> RXCMPLT_R {
        RXCMPLT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX used bit read"]
    #[inline(always)]
    pub fn rxusedbitread(&self) -> RXUSEDBITREAD_R {
        RXUSEDBITREAD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TX used bit read"]
    #[inline(always)]
    pub fn txusedbitread(&self) -> TXUSEDBITREAD_R {
        TXUSEDBITREAD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit under run"]
    #[inline(always)]
    pub fn txunderrun(&self) -> TXUNDERRUN_R {
        TXUNDERRUN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Retry limit exceeded or late collision"]
    #[inline(always)]
    pub fn rtrylmtorlatecol(&self) -> RTRYLMTORLATECOL_R {
        RTRYLMTORLATECOL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmit frame corruption due to AMBA (AHB) error."]
    #[inline(always)]
    pub fn ambaerr(&self) -> AMBAERR_R {
        AMBAERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit complete"]
    #[inline(always)]
    pub fn txcmplt(&self) -> TXCMPLT_R {
        TXCMPLT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Receive overrun"]
    #[inline(always)]
    pub fn rxoverrun(&self) -> RXOVERRUN_R {
        RXOVERRUN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Hresp not OK"]
    #[inline(always)]
    pub fn respnotok(&self) -> RESPNOTOK_R {
        RESPNOTOK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pause frame with non-zero pause quantum received"]
    #[inline(always)]
    pub fn nonzeropfrmquant(&self) -> NONZEROPFRMQUANT_R {
        NONZEROPFRMQUANT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pause Time zero"]
    #[inline(always)]
    pub fn pausetimezero(&self) -> PAUSETIMEZERO_R {
        PAUSETIMEZERO_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pause frame transmitted"]
    #[inline(always)]
    pub fn pfrmtx(&self) -> PFRMTX_R {
        PFRMTX_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PTP delay_req frame received"]
    #[inline(always)]
    pub fn ptpdlyreqfrmrx(&self) -> PTPDLYREQFRMRX_R {
        PTPDLYREQFRMRX_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PTP sync frame received"]
    #[inline(always)]
    pub fn ptpsyncfrmrx(&self) -> PTPSYNCFRMRX_R {
        PTPSYNCFRMRX_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PTP delay_req frame transmitted"]
    #[inline(always)]
    pub fn ptpdlyreqfrmtx(&self) -> PTPDLYREQFRMTX_R {
        PTPDLYREQFRMTX_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - PTP sync frame transmitted"]
    #[inline(always)]
    pub fn ptpsyncfrmtx(&self) -> PTPSYNCFRMTX_R {
        PTPSYNCFRMTX_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - PTP pdelay_req frame received"]
    #[inline(always)]
    pub fn ptppdlyreqfrmrx(&self) -> PTPPDLYREQFRMRX_R {
        PTPPDLYREQFRMRX_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - PTP pdelay_resp frame received"]
    #[inline(always)]
    pub fn ptppdlyrespfrmrx(&self) -> PTPPDLYRESPFRMRX_R {
        PTPPDLYRESPFRMRX_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - PTP pdelay_req frame transmitted"]
    #[inline(always)]
    pub fn ptppdlyreqfrmtx(&self) -> PTPPDLYREQFRMTX_R {
        PTPPDLYREQFRMTX_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - PTP pdelay_resp frame transmitted"]
    #[inline(always)]
    pub fn ptppdlyrespfrmtx(&self) -> PTPPDLYRESPFRMTX_R {
        PTPPDLYRESPFRMTX_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TSU seconds register increment"]
    #[inline(always)]
    pub fn tsusecregincr(&self) -> TSUSECREGINCR_R {
        TSUSECREGINCR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Receive LPI indication status bit change"]
    #[inline(always)]
    pub fn rxlpiindc(&self) -> RXLPIINDC_R {
        RXLPIINDC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - WOL event received interrupt."]
    #[inline(always)]
    pub fn wolevntrx(&self) -> WOLEVNTRX_R {
        WOLEVNTRX_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - TSU timer comparison interrupt."]
    #[inline(always)]
    pub fn tsutimercomp(&self) -> TSUTIMERCOMP_R {
        TSUTIMERCOMP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Management frame sent"]
    #[inline(always)]
    pub fn mngmntdone(&mut self) -> MNGMNTDONE_W {
        MNGMNTDONE_W { w: self }
    }
    #[doc = "Bit 1 - Receive complete"]
    #[inline(always)]
    pub fn rxcmplt(&mut self) -> RXCMPLT_W {
        RXCMPLT_W { w: self }
    }
    #[doc = "Bit 2 - RX used bit read"]
    #[inline(always)]
    pub fn rxusedbitread(&mut self) -> RXUSEDBITREAD_W {
        RXUSEDBITREAD_W { w: self }
    }
    #[doc = "Bit 3 - TX used bit read"]
    #[inline(always)]
    pub fn txusedbitread(&mut self) -> TXUSEDBITREAD_W {
        TXUSEDBITREAD_W { w: self }
    }
    #[doc = "Bit 4 - Transmit under run"]
    #[inline(always)]
    pub fn txunderrun(&mut self) -> TXUNDERRUN_W {
        TXUNDERRUN_W { w: self }
    }
    #[doc = "Bit 5 - Retry limit exceeded or late collision"]
    #[inline(always)]
    pub fn rtrylmtorlatecol(&mut self) -> RTRYLMTORLATECOL_W {
        RTRYLMTORLATECOL_W { w: self }
    }
    #[doc = "Bit 6 - Transmit frame corruption due to AMBA (AHB) error."]
    #[inline(always)]
    pub fn ambaerr(&mut self) -> AMBAERR_W {
        AMBAERR_W { w: self }
    }
    #[doc = "Bit 7 - Transmit complete"]
    #[inline(always)]
    pub fn txcmplt(&mut self) -> TXCMPLT_W {
        TXCMPLT_W { w: self }
    }
    #[doc = "Bit 10 - Receive overrun"]
    #[inline(always)]
    pub fn rxoverrun(&mut self) -> RXOVERRUN_W {
        RXOVERRUN_W { w: self }
    }
    #[doc = "Bit 11 - Hresp not OK"]
    #[inline(always)]
    pub fn respnotok(&mut self) -> RESPNOTOK_W {
        RESPNOTOK_W { w: self }
    }
    #[doc = "Bit 12 - Pause frame with non-zero pause quantum received"]
    #[inline(always)]
    pub fn nonzeropfrmquant(&mut self) -> NONZEROPFRMQUANT_W {
        NONZEROPFRMQUANT_W { w: self }
    }
    #[doc = "Bit 13 - Pause Time zero"]
    #[inline(always)]
    pub fn pausetimezero(&mut self) -> PAUSETIMEZERO_W {
        PAUSETIMEZERO_W { w: self }
    }
    #[doc = "Bit 14 - Pause frame transmitted"]
    #[inline(always)]
    pub fn pfrmtx(&mut self) -> PFRMTX_W {
        PFRMTX_W { w: self }
    }
    #[doc = "Bit 18 - PTP delay_req frame received"]
    #[inline(always)]
    pub fn ptpdlyreqfrmrx(&mut self) -> PTPDLYREQFRMRX_W {
        PTPDLYREQFRMRX_W { w: self }
    }
    #[doc = "Bit 19 - PTP sync frame received"]
    #[inline(always)]
    pub fn ptpsyncfrmrx(&mut self) -> PTPSYNCFRMRX_W {
        PTPSYNCFRMRX_W { w: self }
    }
    #[doc = "Bit 20 - PTP delay_req frame transmitted"]
    #[inline(always)]
    pub fn ptpdlyreqfrmtx(&mut self) -> PTPDLYREQFRMTX_W {
        PTPDLYREQFRMTX_W { w: self }
    }
    #[doc = "Bit 21 - PTP sync frame transmitted"]
    #[inline(always)]
    pub fn ptpsyncfrmtx(&mut self) -> PTPSYNCFRMTX_W {
        PTPSYNCFRMTX_W { w: self }
    }
    #[doc = "Bit 22 - PTP pdelay_req frame received"]
    #[inline(always)]
    pub fn ptppdlyreqfrmrx(&mut self) -> PTPPDLYREQFRMRX_W {
        PTPPDLYREQFRMRX_W { w: self }
    }
    #[doc = "Bit 23 - PTP pdelay_resp frame received"]
    #[inline(always)]
    pub fn ptppdlyrespfrmrx(&mut self) -> PTPPDLYRESPFRMRX_W {
        PTPPDLYRESPFRMRX_W { w: self }
    }
    #[doc = "Bit 24 - PTP pdelay_req frame transmitted"]
    #[inline(always)]
    pub fn ptppdlyreqfrmtx(&mut self) -> PTPPDLYREQFRMTX_W {
        PTPPDLYREQFRMTX_W { w: self }
    }
    #[doc = "Bit 25 - PTP pdelay_resp frame transmitted"]
    #[inline(always)]
    pub fn ptppdlyrespfrmtx(&mut self) -> PTPPDLYRESPFRMTX_W {
        PTPPDLYRESPFRMTX_W { w: self }
    }
    #[doc = "Bit 26 - TSU seconds register increment"]
    #[inline(always)]
    pub fn tsusecregincr(&mut self) -> TSUSECREGINCR_W {
        TSUSECREGINCR_W { w: self }
    }
    #[doc = "Bit 27 - Receive LPI indication status bit change"]
    #[inline(always)]
    pub fn rxlpiindc(&mut self) -> RXLPIINDC_W {
        RXLPIINDC_W { w: self }
    }
    #[doc = "Bit 28 - WOL event received interrupt."]
    #[inline(always)]
    pub fn wolevntrx(&mut self) -> WOLEVNTRX_W {
        WOLEVNTRX_W { w: self }
    }
    #[doc = "Bit 29 - TSU timer comparison interrupt."]
    #[inline(always)]
    pub fn tsutimercomp(&mut self) -> TSUTIMERCOMP_W {
        TSUTIMERCOMP_W { w: self }
    }
}
