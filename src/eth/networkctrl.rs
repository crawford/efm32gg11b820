#[doc = "Reader of register NETWORKCTRL"]
pub type R = crate::R<u32, super::NETWORKCTRL>;
#[doc = "Writer for register NETWORKCTRL"]
pub type W = crate::W<u32, super::NETWORKCTRL>;
#[doc = "Register NETWORKCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::NETWORKCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LOOPBACKLOCAL`"]
pub type LOOPBACKLOCAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOOPBACKLOCAL`"]
pub struct LOOPBACKLOCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPBACKLOCAL_W<'a> {
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
#[doc = "Reader of field `ENBRX`"]
pub type ENBRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENBRX`"]
pub struct ENBRX_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBRX_W<'a> {
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
#[doc = "Reader of field `ENBTX`"]
pub type ENBTX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENBTX`"]
pub struct ENBTX_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBTX_W<'a> {
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
#[doc = "Reader of field `MANPORTEN`"]
pub type MANPORTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MANPORTEN`"]
pub struct MANPORTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MANPORTEN_W<'a> {
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
#[doc = "Reader of field `CLRALLSTATSREGS`"]
pub type CLRALLSTATSREGS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRALLSTATSREGS`"]
pub struct CLRALLSTATSREGS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRALLSTATSREGS_W<'a> {
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
#[doc = "Reader of field `INCALLSTATSREGS`"]
pub type INCALLSTATSREGS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INCALLSTATSREGS`"]
pub struct INCALLSTATSREGS_W<'a> {
    w: &'a mut W,
}
impl<'a> INCALLSTATSREGS_W<'a> {
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
#[doc = "Reader of field `STATSWREN`"]
pub type STATSWREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATSWREN`"]
pub struct STATSWREN_W<'a> {
    w: &'a mut W,
}
impl<'a> STATSWREN_W<'a> {
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
#[doc = "Reader of field `BACKPRESSURE`"]
pub type BACKPRESSURE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BACKPRESSURE`"]
pub struct BACKPRESSURE_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKPRESSURE_W<'a> {
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
#[doc = "Reader of field `TXSTRT`"]
pub type TXSTRT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXSTRT`"]
pub struct TXSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSTRT_W<'a> {
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
#[doc = "Reader of field `TXHALT`"]
pub type TXHALT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXHALT`"]
pub struct TXHALT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXHALT_W<'a> {
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
#[doc = "Reader of field `TXPFRMREQ`"]
pub type TXPFRMREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXPFRMREQ`"]
pub struct TXPFRMREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPFRMREQ_W<'a> {
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
#[doc = "Reader of field `TXPFRMZERO`"]
pub type TXPFRMZERO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXPFRMZERO`"]
pub struct TXPFRMZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPFRMZERO_W<'a> {
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
#[doc = "Reader of field `STORERXTS`"]
pub type STORERXTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STORERXTS`"]
pub struct STORERXTS_W<'a> {
    w: &'a mut W,
}
impl<'a> STORERXTS_W<'a> {
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
#[doc = "Reader of field `PFCENB`"]
pub type PFCENB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PFCENB`"]
pub struct PFCENB_W<'a> {
    w: &'a mut W,
}
impl<'a> PFCENB_W<'a> {
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
#[doc = "Reader of field `TXPFCPRIORPFRM`"]
pub type TXPFCPRIORPFRM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXPFCPRIORPFRM`"]
pub struct TXPFCPRIORPFRM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPFCPRIORPFRM_W<'a> {
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
#[doc = "Reader of field `FLUSHRXPKT`"]
pub type FLUSHRXPKT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLUSHRXPKT`"]
pub struct FLUSHRXPKT_W<'a> {
    w: &'a mut W,
}
impl<'a> FLUSHRXPKT_W<'a> {
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
#[doc = "Reader of field `TXLPIEN`"]
pub type TXLPIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXLPIEN`"]
pub struct TXLPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXLPIEN_W<'a> {
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
#[doc = "Reader of field `PTPUNICASTEN`"]
pub type PTPUNICASTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTPUNICASTEN`"]
pub struct PTPUNICASTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PTPUNICASTEN_W<'a> {
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
#[doc = "Reader of field `STOREUDPOFFSET`"]
pub type STOREUDPOFFSET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOREUDPOFFSET`"]
pub struct STOREUDPOFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> STOREUDPOFFSET_W<'a> {
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
#[doc = "Reader of field `ONESTEPSYNCMODE`"]
pub type ONESTEPSYNCMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ONESTEPSYNCMODE`"]
pub struct ONESTEPSYNCMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ONESTEPSYNCMODE_W<'a> {
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
#[doc = "Reader of field `PFCCTRL`"]
pub type PFCCTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PFCCTRL`"]
pub struct PFCCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> PFCCTRL_W<'a> {
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
impl R {
    #[doc = "Bit 1 - Loopback local"]
    #[inline(always)]
    pub fn loopbacklocal(&self) -> LOOPBACKLOCAL_R {
        LOOPBACKLOCAL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive enable"]
    #[inline(always)]
    pub fn enbrx(&self) -> ENBRX_R {
        ENBRX_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit enable"]
    #[inline(always)]
    pub fn enbtx(&self) -> ENBTX_R {
        ENBTX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Management port enable"]
    #[inline(always)]
    pub fn manporten(&self) -> MANPORTEN_R {
        MANPORTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clear statistics registers"]
    #[inline(always)]
    pub fn clrallstatsregs(&self) -> CLRALLSTATSREGS_R {
        CLRALLSTATSREGS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Incremental statistics registers"]
    #[inline(always)]
    pub fn incallstatsregs(&self) -> INCALLSTATSREGS_R {
        INCALLSTATSREGS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write enable for statistics registers"]
    #[inline(always)]
    pub fn statswren(&self) -> STATSWREN_R {
        STATSWREN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Back pressure will force collisions on all received frames"]
    #[inline(always)]
    pub fn backpressure(&self) -> BACKPRESSURE_R {
        BACKPRESSURE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Start transmission"]
    #[inline(always)]
    pub fn txstrt(&self) -> TXSTRT_R {
        TXSTRT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmit halt"]
    #[inline(always)]
    pub fn txhalt(&self) -> TXHALT_R {
        TXHALT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmit pause frame"]
    #[inline(always)]
    pub fn txpfrmreq(&self) -> TXPFRMREQ_R {
        TXPFRMREQ_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Transmit zero quantum pause frame"]
    #[inline(always)]
    pub fn txpfrmzero(&self) -> TXPFRMZERO_R {
        TXPFRMZERO_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Store receive time stamp to memory."]
    #[inline(always)]
    pub fn storerxts(&self) -> STORERXTS_R {
        STORERXTS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable PFC Priority Based Pause Reception capabilities."]
    #[inline(always)]
    pub fn pfcenb(&self) -> PFCENB_R {
        PFCENB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Write a one to transmit PFC priority based pause frame."]
    #[inline(always)]
    pub fn txpfcpriorpfrm(&self) -> TXPFCPRIORPFRM_R {
        TXPFCPRIORPFRM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Flush the next packet from the external RX DPRAM."]
    #[inline(always)]
    pub fn flushrxpkt(&self) -> FLUSHRXPKT_R {
        FLUSHRXPKT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable LPI transmission when set LPI (low power idle) is immediately transmitted."]
    #[inline(always)]
    pub fn txlpien(&self) -> TXLPIEN_R {
        TXLPIEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable detection of unicast PTP unicast frames."]
    #[inline(always)]
    pub fn ptpunicasten(&self) -> PTPUNICASTEN_R {
        PTPUNICASTEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Store UDP / TCP offset to memory."]
    #[inline(always)]
    pub fn storeudpoffset(&self) -> STOREUDPOFFSET_R {
        STOREUDPOFFSET_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 1588 One Step Sync Mode."]
    #[inline(always)]
    pub fn onestepsyncmode(&self) -> ONESTEPSYNCMODE_R {
        ONESTEPSYNCMODE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable multiple PFC pause quantums, one per pause priority"]
    #[inline(always)]
    pub fn pfcctrl(&self) -> PFCCTRL_R {
        PFCCTRL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Loopback local"]
    #[inline(always)]
    pub fn loopbacklocal(&mut self) -> LOOPBACKLOCAL_W {
        LOOPBACKLOCAL_W { w: self }
    }
    #[doc = "Bit 2 - Receive enable"]
    #[inline(always)]
    pub fn enbrx(&mut self) -> ENBRX_W {
        ENBRX_W { w: self }
    }
    #[doc = "Bit 3 - Transmit enable"]
    #[inline(always)]
    pub fn enbtx(&mut self) -> ENBTX_W {
        ENBTX_W { w: self }
    }
    #[doc = "Bit 4 - Management port enable"]
    #[inline(always)]
    pub fn manporten(&mut self) -> MANPORTEN_W {
        MANPORTEN_W { w: self }
    }
    #[doc = "Bit 5 - Clear statistics registers"]
    #[inline(always)]
    pub fn clrallstatsregs(&mut self) -> CLRALLSTATSREGS_W {
        CLRALLSTATSREGS_W { w: self }
    }
    #[doc = "Bit 6 - Incremental statistics registers"]
    #[inline(always)]
    pub fn incallstatsregs(&mut self) -> INCALLSTATSREGS_W {
        INCALLSTATSREGS_W { w: self }
    }
    #[doc = "Bit 7 - Write enable for statistics registers"]
    #[inline(always)]
    pub fn statswren(&mut self) -> STATSWREN_W {
        STATSWREN_W { w: self }
    }
    #[doc = "Bit 8 - Back pressure will force collisions on all received frames"]
    #[inline(always)]
    pub fn backpressure(&mut self) -> BACKPRESSURE_W {
        BACKPRESSURE_W { w: self }
    }
    #[doc = "Bit 9 - Start transmission"]
    #[inline(always)]
    pub fn txstrt(&mut self) -> TXSTRT_W {
        TXSTRT_W { w: self }
    }
    #[doc = "Bit 10 - Transmit halt"]
    #[inline(always)]
    pub fn txhalt(&mut self) -> TXHALT_W {
        TXHALT_W { w: self }
    }
    #[doc = "Bit 11 - Transmit pause frame"]
    #[inline(always)]
    pub fn txpfrmreq(&mut self) -> TXPFRMREQ_W {
        TXPFRMREQ_W { w: self }
    }
    #[doc = "Bit 12 - Transmit zero quantum pause frame"]
    #[inline(always)]
    pub fn txpfrmzero(&mut self) -> TXPFRMZERO_W {
        TXPFRMZERO_W { w: self }
    }
    #[doc = "Bit 15 - Store receive time stamp to memory."]
    #[inline(always)]
    pub fn storerxts(&mut self) -> STORERXTS_W {
        STORERXTS_W { w: self }
    }
    #[doc = "Bit 16 - Enable PFC Priority Based Pause Reception capabilities."]
    #[inline(always)]
    pub fn pfcenb(&mut self) -> PFCENB_W {
        PFCENB_W { w: self }
    }
    #[doc = "Bit 17 - Write a one to transmit PFC priority based pause frame."]
    #[inline(always)]
    pub fn txpfcpriorpfrm(&mut self) -> TXPFCPRIORPFRM_W {
        TXPFCPRIORPFRM_W { w: self }
    }
    #[doc = "Bit 18 - Flush the next packet from the external RX DPRAM."]
    #[inline(always)]
    pub fn flushrxpkt(&mut self) -> FLUSHRXPKT_W {
        FLUSHRXPKT_W { w: self }
    }
    #[doc = "Bit 19 - Enable LPI transmission when set LPI (low power idle) is immediately transmitted."]
    #[inline(always)]
    pub fn txlpien(&mut self) -> TXLPIEN_W {
        TXLPIEN_W { w: self }
    }
    #[doc = "Bit 20 - Enable detection of unicast PTP unicast frames."]
    #[inline(always)]
    pub fn ptpunicasten(&mut self) -> PTPUNICASTEN_W {
        PTPUNICASTEN_W { w: self }
    }
    #[doc = "Bit 22 - Store UDP / TCP offset to memory."]
    #[inline(always)]
    pub fn storeudpoffset(&mut self) -> STOREUDPOFFSET_W {
        STOREUDPOFFSET_W { w: self }
    }
    #[doc = "Bit 24 - 1588 One Step Sync Mode."]
    #[inline(always)]
    pub fn onestepsyncmode(&mut self) -> ONESTEPSYNCMODE_W {
        ONESTEPSYNCMODE_W { w: self }
    }
    #[doc = "Bit 25 - Enable multiple PFC pause quantums, one per pause priority"]
    #[inline(always)]
    pub fn pfcctrl(&mut self) -> PFCCTRL_W {
        PFCCTRL_W { w: self }
    }
}
