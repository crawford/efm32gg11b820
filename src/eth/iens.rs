#[doc = "Writer for register IENS"]
pub type W = crate::W<u32, super::IENS>;
#[doc = "Register IENS `reset()`'s with value 0"]
impl crate::ResetValue for super::IENS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
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
impl W {
    #[doc = "Bit 0 - Enable management done interrupt"]
    #[inline(always)]
    pub fn mngmntdone(&mut self) -> MNGMNTDONE_W {
        MNGMNTDONE_W { w: self }
    }
    #[doc = "Bit 1 - Enable receive complete interrupt"]
    #[inline(always)]
    pub fn rxcmplt(&mut self) -> RXCMPLT_W {
        RXCMPLT_W { w: self }
    }
    #[doc = "Bit 2 - Enable receive used bit read interrupt"]
    #[inline(always)]
    pub fn rxusedbitread(&mut self) -> RXUSEDBITREAD_W {
        RXUSEDBITREAD_W { w: self }
    }
    #[doc = "Bit 3 - Enable transmit used bit read interrupt"]
    #[inline(always)]
    pub fn txusedbitread(&mut self) -> TXUSEDBITREAD_W {
        TXUSEDBITREAD_W { w: self }
    }
    #[doc = "Bit 4 - Enable transmit buffer under run interrupt"]
    #[inline(always)]
    pub fn txunderrun(&mut self) -> TXUNDERRUN_W {
        TXUNDERRUN_W { w: self }
    }
    #[doc = "Bit 5 - Enable retry limit exceeded or late collision interrupt"]
    #[inline(always)]
    pub fn rtrylmtorlatecol(&mut self) -> RTRYLMTORLATECOL_W {
        RTRYLMTORLATECOL_W { w: self }
    }
    #[doc = "Bit 6 - Enable transmit frame corruption due to AMBA (AHB) error interrupt"]
    #[inline(always)]
    pub fn ambaerr(&mut self) -> AMBAERR_W {
        AMBAERR_W { w: self }
    }
    #[doc = "Bit 7 - Enable transmit complete interrupt"]
    #[inline(always)]
    pub fn txcmplt(&mut self) -> TXCMPLT_W {
        TXCMPLT_W { w: self }
    }
    #[doc = "Bit 10 - Enable receive overrun interrupt"]
    #[inline(always)]
    pub fn rxoverrun(&mut self) -> RXOVERRUN_W {
        RXOVERRUN_W { w: self }
    }
    #[doc = "Bit 11 - Enable bresp/hresp not OK interrupt"]
    #[inline(always)]
    pub fn respnotok(&mut self) -> RESPNOTOK_W {
        RESPNOTOK_W { w: self }
    }
    #[doc = "Bit 12 - Enable pause frame with non-zero pause quantum interrupt"]
    #[inline(always)]
    pub fn nonzeropfrmquant(&mut self) -> NONZEROPFRMQUANT_W {
        NONZEROPFRMQUANT_W { w: self }
    }
    #[doc = "Bit 13 - Enable pause time zero interrupt"]
    #[inline(always)]
    pub fn pausetimezero(&mut self) -> PAUSETIMEZERO_W {
        PAUSETIMEZERO_W { w: self }
    }
    #[doc = "Bit 14 - Enable pause frame transmitted interrupt"]
    #[inline(always)]
    pub fn pfrmtx(&mut self) -> PFRMTX_W {
        PFRMTX_W { w: self }
    }
    #[doc = "Bit 18 - Enable PTP delay_req frame received interrupt"]
    #[inline(always)]
    pub fn ptpdlyreqfrmrx(&mut self) -> PTPDLYREQFRMRX_W {
        PTPDLYREQFRMRX_W { w: self }
    }
    #[doc = "Bit 19 - Enable PTP sync frame received interrupt"]
    #[inline(always)]
    pub fn ptpsyncfrmrx(&mut self) -> PTPSYNCFRMRX_W {
        PTPSYNCFRMRX_W { w: self }
    }
    #[doc = "Bit 20 - Enable PTP delay_req frame transmitted interrupt"]
    #[inline(always)]
    pub fn ptpdlyreqfrmtx(&mut self) -> PTPDLYREQFRMTX_W {
        PTPDLYREQFRMTX_W { w: self }
    }
    #[doc = "Bit 21 - Enable PTP sync frame transmitted interrupt"]
    #[inline(always)]
    pub fn ptpsyncfrmtx(&mut self) -> PTPSYNCFRMTX_W {
        PTPSYNCFRMTX_W { w: self }
    }
    #[doc = "Bit 22 - Enable PTP pdelay_req frame received interrupt"]
    #[inline(always)]
    pub fn ptppdlyreqfrmrx(&mut self) -> PTPPDLYREQFRMRX_W {
        PTPPDLYREQFRMRX_W { w: self }
    }
    #[doc = "Bit 23 - Enable PTP pdelay_resp frame received interrupt"]
    #[inline(always)]
    pub fn ptppdlyrespfrmrx(&mut self) -> PTPPDLYRESPFRMRX_W {
        PTPPDLYRESPFRMRX_W { w: self }
    }
    #[doc = "Bit 24 - Enable PTP pdelay_req frame transmitted interrupt"]
    #[inline(always)]
    pub fn ptppdlyreqfrmtx(&mut self) -> PTPPDLYREQFRMTX_W {
        PTPPDLYREQFRMTX_W { w: self }
    }
    #[doc = "Bit 25 - Enable PTP pdelay_resp frame transmitted interrupt"]
    #[inline(always)]
    pub fn ptppdlyrespfrmtx(&mut self) -> PTPPDLYRESPFRMTX_W {
        PTPPDLYRESPFRMTX_W { w: self }
    }
    #[doc = "Bit 26 - Enable TSU seconds register increment interrupt"]
    #[inline(always)]
    pub fn tsusecregincr(&mut self) -> TSUSECREGINCR_W {
        TSUSECREGINCR_W { w: self }
    }
    #[doc = "Bit 27 - Enable RX LPI indication interrupt"]
    #[inline(always)]
    pub fn rxlpiindc(&mut self) -> RXLPIINDC_W {
        RXLPIINDC_W { w: self }
    }
    #[doc = "Bit 28 - Enable WOL event received interrupt"]
    #[inline(always)]
    pub fn wolevntrx(&mut self) -> WOLEVNTRX_W {
        WOLEVNTRX_W { w: self }
    }
    #[doc = "Bit 29 - Enable TSU timer comparison interrupt."]
    #[inline(always)]
    pub fn tsutimercomp(&mut self) -> TSUTIMERCOMP_W {
        TSUTIMERCOMP_W { w: self }
    }
}
