#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `RXENS`"]
pub type RXENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXENS`"]
pub type TXENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MASTER`"]
pub type MASTER_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXBLOCK`"]
pub type RXBLOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXTRI`"]
pub type TXTRI_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXC`"]
pub type TXC_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXBL`"]
pub type TXBL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXDATAV`"]
pub type RXDATAV_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFULL`"]
pub type RXFULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXBDRIGHT`"]
pub type TXBDRIGHT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXBSRIGHT`"]
pub type TXBSRIGHT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXDATAVRIGHT`"]
pub type RXDATAVRIGHT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFULLRIGHT`"]
pub type RXFULLRIGHT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXIDLE`"]
pub type TXIDLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIMERRESTARTED`"]
pub type TIMERRESTARTED_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXBUFCNT`"]
pub type TXBUFCNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Receiver Enable Status"]
    #[inline(always)]
    pub fn rxens(&self) -> RXENS_R {
        RXENS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmitter Enable Status"]
    #[inline(always)]
    pub fn txens(&self) -> TXENS_R {
        TXENS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SPI Master Mode"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Block Incoming Data"]
    #[inline(always)]
    pub fn rxblock(&self) -> RXBLOCK_R {
        RXBLOCK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmitter Tristated"]
    #[inline(always)]
    pub fn txtri(&self) -> TXTRI_R {
        TXTRI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TX Complete"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TX Buffer Level"]
    #[inline(always)]
    pub fn txbl(&self) -> TXBL_R {
        TXBL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RX Data Valid"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RXDATAV_R {
        RXDATAV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RX FIFO Full"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TX Buffer Expects Double Right Data"]
    #[inline(always)]
    pub fn txbdright(&self) -> TXBDRIGHT_R {
        TXBDRIGHT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TX Buffer Expects Single Right Data"]
    #[inline(always)]
    pub fn txbsright(&self) -> TXBSRIGHT_R {
        TXBSRIGHT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RX Data Right"]
    #[inline(always)]
    pub fn rxdatavright(&self) -> RXDATAVRIGHT_R {
        RXDATAVRIGHT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RX Full of Right Data"]
    #[inline(always)]
    pub fn rxfullright(&self) -> RXFULLRIGHT_R {
        RXFULLRIGHT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TX Idle"]
    #[inline(always)]
    pub fn txidle(&self) -> TXIDLE_R {
        TXIDLE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - The USART Timer Restarted Itself"]
    #[inline(always)]
    pub fn timerrestarted(&self) -> TIMERRESTARTED_R {
        TIMERRESTARTED_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - TX Buffer Count"]
    #[inline(always)]
    pub fn txbufcnt(&self) -> TXBUFCNT_R {
        TXBUFCNT_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
