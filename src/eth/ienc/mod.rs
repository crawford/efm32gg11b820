#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IENC {
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
}
#[doc = r" Proxy"]
pub struct _MNGMNTDONEW<'a> {
    w: &'a mut W,
}
impl<'a> _MNGMNTDONEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXCMPLTW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCMPLTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXUSEDBITREADW<'a> {
    w: &'a mut W,
}
impl<'a> _RXUSEDBITREADW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXUSEDBITREADW<'a> {
    w: &'a mut W,
}
impl<'a> _TXUSEDBITREADW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXUNDERRUNW<'a> {
    w: &'a mut W,
}
impl<'a> _TXUNDERRUNW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RTRYLMTORLATECOLW<'a> {
    w: &'a mut W,
}
impl<'a> _RTRYLMTORLATECOLW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AMBAERRW<'a> {
    w: &'a mut W,
}
impl<'a> _AMBAERRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXCMPLTW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCMPLTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXOVERRUNW<'a> {
    w: &'a mut W,
}
impl<'a> _RXOVERRUNW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESPNOTOKW<'a> {
    w: &'a mut W,
}
impl<'a> _RESPNOTOKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NONZEROPFRMQUANTW<'a> {
    w: &'a mut W,
}
impl<'a> _NONZEROPFRMQUANTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PAUSETIMEZEROW<'a> {
    w: &'a mut W,
}
impl<'a> _PAUSETIMEZEROW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PFRMTXW<'a> {
    w: &'a mut W,
}
impl<'a> _PFRMTXW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PTPDLYREQFRMRXW<'a> {
    w: &'a mut W,
}
impl<'a> _PTPDLYREQFRMRXW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PTPSYNCFRMRXW<'a> {
    w: &'a mut W,
}
impl<'a> _PTPSYNCFRMRXW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PTPDLYREQFRMTXW<'a> {
    w: &'a mut W,
}
impl<'a> _PTPDLYREQFRMTXW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PTPSYNCFRMTXW<'a> {
    w: &'a mut W,
}
impl<'a> _PTPSYNCFRMTXW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PTPPDLYREQFRMRXW<'a> {
    w: &'a mut W,
}
impl<'a> _PTPPDLYREQFRMRXW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PTPPDLYRESPFRMRXW<'a> {
    w: &'a mut W,
}
impl<'a> _PTPPDLYRESPFRMRXW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PTPPDLYREQFRMTXW<'a> {
    w: &'a mut W,
}
impl<'a> _PTPPDLYREQFRMTXW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PTPPDLYRESPFRMTXW<'a> {
    w: &'a mut W,
}
impl<'a> _PTPPDLYRESPFRMTXW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSUSECREGINCRW<'a> {
    w: &'a mut W,
}
impl<'a> _TSUSECREGINCRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXLPIINDCW<'a> {
    w: &'a mut W,
}
impl<'a> _RXLPIINDCW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WOLEVNTRXW<'a> {
    w: &'a mut W,
}
impl<'a> _WOLEVNTRXW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSUTIMERCOMPW<'a> {
    w: &'a mut W,
}
impl<'a> _TSUTIMERCOMPW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Disable management done interrupt"]
    #[inline]
    pub fn mngmntdone(&mut self) -> _MNGMNTDONEW {
        _MNGMNTDONEW { w: self }
    }
    #[doc = "Bit 1 - Disable receive complete interrupt"]
    #[inline]
    pub fn rxcmplt(&mut self) -> _RXCMPLTW {
        _RXCMPLTW { w: self }
    }
    #[doc = "Bit 2 - Disable receive used bit read interrupt"]
    #[inline]
    pub fn rxusedbitread(&mut self) -> _RXUSEDBITREADW {
        _RXUSEDBITREADW { w: self }
    }
    #[doc = "Bit 3 - Disable transmit used bit read interrupt"]
    #[inline]
    pub fn txusedbitread(&mut self) -> _TXUSEDBITREADW {
        _TXUSEDBITREADW { w: self }
    }
    #[doc = "Bit 4 - Disable transmit buffer under run interrupt"]
    #[inline]
    pub fn txunderrun(&mut self) -> _TXUNDERRUNW {
        _TXUNDERRUNW { w: self }
    }
    #[doc = "Bit 5 - Disable retry limit exceeded or late collision interrupt"]
    #[inline]
    pub fn rtrylmtorlatecol(&mut self) -> _RTRYLMTORLATECOLW {
        _RTRYLMTORLATECOLW { w: self }
    }
    #[doc = "Bit 6 - Disable transmit frame corruption due to AMBA (AHB) error interrupt"]
    #[inline]
    pub fn ambaerr(&mut self) -> _AMBAERRW {
        _AMBAERRW { w: self }
    }
    #[doc = "Bit 7 - Disable transmit complete interrupt"]
    #[inline]
    pub fn txcmplt(&mut self) -> _TXCMPLTW {
        _TXCMPLTW { w: self }
    }
    #[doc = "Bit 10 - Disable receive overrun interrupt"]
    #[inline]
    pub fn rxoverrun(&mut self) -> _RXOVERRUNW {
        _RXOVERRUNW { w: self }
    }
    #[doc = "Bit 11 - Disable bresp/hresp not OK interrupt"]
    #[inline]
    pub fn respnotok(&mut self) -> _RESPNOTOKW {
        _RESPNOTOKW { w: self }
    }
    #[doc = "Bit 12 - Disable pause frame with non-zero pause quantum interrupt"]
    #[inline]
    pub fn nonzeropfrmquant(&mut self) -> _NONZEROPFRMQUANTW {
        _NONZEROPFRMQUANTW { w: self }
    }
    #[doc = "Bit 13 - Disable pause time zero interrupt"]
    #[inline]
    pub fn pausetimezero(&mut self) -> _PAUSETIMEZEROW {
        _PAUSETIMEZEROW { w: self }
    }
    #[doc = "Bit 14 - Disable pause frame transmitted interrupt"]
    #[inline]
    pub fn pfrmtx(&mut self) -> _PFRMTXW {
        _PFRMTXW { w: self }
    }
    #[doc = "Bit 18 - Disable PTP delay_req frame received interrupt"]
    #[inline]
    pub fn ptpdlyreqfrmrx(&mut self) -> _PTPDLYREQFRMRXW {
        _PTPDLYREQFRMRXW { w: self }
    }
    #[doc = "Bit 19 - Disable PTP sync frame received interrupt"]
    #[inline]
    pub fn ptpsyncfrmrx(&mut self) -> _PTPSYNCFRMRXW {
        _PTPSYNCFRMRXW { w: self }
    }
    #[doc = "Bit 20 - Disable PTP delay_req frame transmitted interrupt"]
    #[inline]
    pub fn ptpdlyreqfrmtx(&mut self) -> _PTPDLYREQFRMTXW {
        _PTPDLYREQFRMTXW { w: self }
    }
    #[doc = "Bit 21 - Disable PTP sync frame transmitted interrupt"]
    #[inline]
    pub fn ptpsyncfrmtx(&mut self) -> _PTPSYNCFRMTXW {
        _PTPSYNCFRMTXW { w: self }
    }
    #[doc = "Bit 22 - Disable PTP pdelay_req frame received interrupt"]
    #[inline]
    pub fn ptppdlyreqfrmrx(&mut self) -> _PTPPDLYREQFRMRXW {
        _PTPPDLYREQFRMRXW { w: self }
    }
    #[doc = "Bit 23 - Disable PTP pdelay_resp frame received interrupt"]
    #[inline]
    pub fn ptppdlyrespfrmrx(&mut self) -> _PTPPDLYRESPFRMRXW {
        _PTPPDLYRESPFRMRXW { w: self }
    }
    #[doc = "Bit 24 - Disable PTP pdelay_req frame transmitted interrupt"]
    #[inline]
    pub fn ptppdlyreqfrmtx(&mut self) -> _PTPPDLYREQFRMTXW {
        _PTPPDLYREQFRMTXW { w: self }
    }
    #[doc = "Bit 25 - Disable PTP pdelay_resp frame transmitted interrupt"]
    #[inline]
    pub fn ptppdlyrespfrmtx(&mut self) -> _PTPPDLYRESPFRMTXW {
        _PTPPDLYRESPFRMTXW { w: self }
    }
    #[doc = "Bit 26 - Disable TSU seconds register increment interrupt"]
    #[inline]
    pub fn tsusecregincr(&mut self) -> _TSUSECREGINCRW {
        _TSUSECREGINCRW { w: self }
    }
    #[doc = "Bit 27 - Disable RX LPI indication interrupt"]
    #[inline]
    pub fn rxlpiindc(&mut self) -> _RXLPIINDCW {
        _RXLPIINDCW { w: self }
    }
    #[doc = "Bit 28 - Disable WOL event received interrupt"]
    #[inline]
    pub fn wolevntrx(&mut self) -> _WOLEVNTRXW {
        _WOLEVNTRXW { w: self }
    }
    #[doc = "Bit 29 - Disable TSU timer comparison interrupt."]
    #[inline]
    pub fn tsutimercomp(&mut self) -> _TSUTIMERCOMPW {
        _TSUTIMERCOMPW { w: self }
    }
}
