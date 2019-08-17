#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `TXC`"]
pub type TXC_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXBL`"]
pub type TXBL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXDATAV`"]
pub type RXDATAV_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFULL`"]
pub type RXFULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXOF`"]
pub type RXOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXUF`"]
pub type RXUF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXOF`"]
pub type TXOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXUF`"]
pub type TXUF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PERR`"]
pub type PERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `FERR`"]
pub type FERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `MPAF`"]
pub type MPAF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SSM`"]
pub type SSM_R = crate::R<bool, bool>;
#[doc = "Reader of field `CCF`"]
pub type CCF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXIDLE`"]
pub type TXIDLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCMP0`"]
pub type TCMP0_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCMP1`"]
pub type TCMP1_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCMP2`"]
pub type TCMP2_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TX Complete Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TX Buffer Level Interrupt Flag"]
    #[inline(always)]
    pub fn txbl(&self) -> TXBL_R {
        TXBL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX Data Valid Interrupt Flag"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RXDATAV_R {
        RXDATAV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RX Buffer Full Interrupt Flag"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxof(&self) -> RXOF_R {
        RXOF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RX Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&self) -> RXUF_R {
        RXUF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&self) -> TXOF_R {
        TXOF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TX Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn txuf(&self) -> TXUF_R {
        TXUF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Parity Error Interrupt Flag"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Framing Error Interrupt Flag"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Multi-Processor Address Frame Interrupt Flag"]
    #[inline(always)]
    pub fn mpaf(&self) -> MPAF_R {
        MPAF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Slave-Select in Master Mode Interrupt Flag"]
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Collision Check Fail Interrupt Flag"]
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TX Idle Interrupt Flag"]
    #[inline(always)]
    pub fn txidle(&self) -> TXIDLE_R {
        TXIDLE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Timer Comparator 0 Interrupt Flag"]
    #[inline(always)]
    pub fn tcmp0(&self) -> TCMP0_R {
        TCMP0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Timer Comparator 1 Interrupt Flag"]
    #[inline(always)]
    pub fn tcmp1(&self) -> TCMP1_R {
        TCMP1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Timer Comparator 2 Interrupt Flag"]
    #[inline(always)]
    pub fn tcmp2(&self) -> TCMP2_R {
        TCMP2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
