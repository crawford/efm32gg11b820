#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `PSTART`"]
pub type PSTART_R = crate::R<bool, bool>;
#[doc = "Reader of field `PSTOP`"]
pub type PSTOP_R = crate::R<bool, bool>;
#[doc = "Reader of field `PACK`"]
pub type PACK_R = crate::R<bool, bool>;
#[doc = "Reader of field `PNACK`"]
pub type PNACK_R = crate::R<bool, bool>;
#[doc = "Reader of field `PCONT`"]
pub type PCONT_R = crate::R<bool, bool>;
#[doc = "Reader of field `PABORT`"]
pub type PABORT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXC`"]
pub type TXC_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXBL`"]
pub type TXBL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXDATAV`"]
pub type RXDATAV_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFULL`"]
pub type RXFULL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Pending START"]
    #[inline(always)]
    pub fn pstart(&self) -> PSTART_R {
        PSTART_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pending STOP"]
    #[inline(always)]
    pub fn pstop(&self) -> PSTOP_R {
        PSTOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pending ACK"]
    #[inline(always)]
    pub fn pack(&self) -> PACK_R {
        PACK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pending NACK"]
    #[inline(always)]
    pub fn pnack(&self) -> PNACK_R {
        PNACK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pending Continue"]
    #[inline(always)]
    pub fn pcont(&self) -> PCONT_R {
        PCONT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pending Abort"]
    #[inline(always)]
    pub fn pabort(&self) -> PABORT_R {
        PABORT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TX Complete"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TX Buffer Level"]
    #[inline(always)]
    pub fn txbl(&self) -> TXBL_R {
        TXBL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RX Data Valid"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RXDATAV_R {
        RXDATAV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RX FIFO Full"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
