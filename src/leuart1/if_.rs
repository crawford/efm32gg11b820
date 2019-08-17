#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `TXC`"]
pub type TXC_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXBL`"]
pub type TXBL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXDATAV`"]
pub type RXDATAV_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXOF`"]
pub type RXOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXUF`"]
pub type RXUF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXOF`"]
pub type TXOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PERR`"]
pub type PERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `FERR`"]
pub type FERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `MPAF`"]
pub type MPAF_R = crate::R<bool, bool>;
#[doc = "Reader of field `STARTF`"]
pub type STARTF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SIGF`"]
pub type SIGF_R = crate::R<bool, bool>;
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
    #[doc = "Bit 3 - RX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxof(&self) -> RXOF_R {
        RXOF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RX Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&self) -> RXUF_R {
        RXUF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TX Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&self) -> TXOF_R {
        TXOF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Parity Error Interrupt Flag"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Framing Error Interrupt Flag"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Multi-Processor Address Frame Interrupt Flag"]
    #[inline(always)]
    pub fn mpaf(&self) -> MPAF_R {
        MPAF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Start Frame Interrupt Flag"]
    #[inline(always)]
    pub fn startf(&self) -> STARTF_R {
        STARTF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Signal Frame Interrupt Flag"]
    #[inline(always)]
    pub fn sigf(&self) -> SIGF_R {
        SIGF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
