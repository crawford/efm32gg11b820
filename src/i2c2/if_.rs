#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSTART`"]
pub type RSTART_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXC`"]
pub type TXC_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXBL`"]
pub type TXBL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXDATAV`"]
pub type RXDATAV_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACK`"]
pub type ACK_R = crate::R<bool, bool>;
#[doc = "Reader of field `NACK`"]
pub type NACK_R = crate::R<bool, bool>;
#[doc = "Reader of field `MSTOP`"]
pub type MSTOP_R = crate::R<bool, bool>;
#[doc = "Reader of field `ARBLOST`"]
pub type ARBLOST_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSERR`"]
pub type BUSERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSHOLD`"]
pub type BUSHOLD_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXOF`"]
pub type TXOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXUF`"]
pub type RXUF_R = crate::R<bool, bool>;
#[doc = "Reader of field `BITO`"]
pub type BITO_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLTO`"]
pub type CLTO_R = crate::R<bool, bool>;
#[doc = "Reader of field `SSTOP`"]
pub type SSTOP_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFULL`"]
pub type RXFULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLERR`"]
pub type CLERR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - START Condition Interrupt Flag"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Repeated START Condition Interrupt Flag"]
    #[inline(always)]
    pub fn rstart(&self) -> RSTART_R {
        RSTART_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Address Interrupt Flag"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transfer Completed Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit Buffer Level Interrupt Flag"]
    #[inline(always)]
    pub fn txbl(&self) -> TXBL_R {
        TXBL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive Data Valid Interrupt Flag"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RXDATAV_R {
        RXDATAV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Acknowledge Received Interrupt Flag"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Not Acknowledge Received Interrupt Flag"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Master STOP Condition Interrupt Flag"]
    #[inline(always)]
    pub fn mstop(&self) -> MSTOP_R {
        MSTOP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Arbitration Lost Interrupt Flag"]
    #[inline(always)]
    pub fn arblost(&self) -> ARBLOST_R {
        ARBLOST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Bus Error Interrupt Flag"]
    #[inline(always)]
    pub fn buserr(&self) -> BUSERR_R {
        BUSERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Bus Held Interrupt Flag"]
    #[inline(always)]
    pub fn bushold(&self) -> BUSHOLD_R {
        BUSHOLD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Transmit Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&self) -> TXOF_R {
        TXOF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Receive Buffer Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&self) -> RXUF_R {
        RXUF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Bus Idle Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn bito(&self) -> BITO_R {
        BITO_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Clock Low Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn clto(&self) -> CLTO_R {
        CLTO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Slave STOP Condition Interrupt Flag"]
    #[inline(always)]
    pub fn sstop(&self) -> SSTOP_R {
        SSTOP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Receive Buffer Full Interrupt Flag"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Clock Low Error Interrupt Flag"]
    #[inline(always)]
    pub fn clerr(&self) -> CLERR_R {
        CLERR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
