#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `RXENS`"]
pub type RXENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXENS`"]
pub type TXENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXBLOCK`"]
pub type RXBLOCK_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXC`"]
pub type TXC_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXBL`"]
pub type TXBL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXDATAV`"]
pub type RXDATAV_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXIDLE`"]
pub type TXIDLE_R = crate::R<bool, bool>;
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
    #[doc = "Bit 2 - Block Incoming Data"]
    #[inline(always)]
    pub fn rxblock(&self) -> RXBLOCK_R {
        RXBLOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TX Complete"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TX Buffer Level"]
    #[inline(always)]
    pub fn txbl(&self) -> TXBL_R {
        TXBL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RX Data Valid"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RXDATAV_R {
        RXDATAV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TX Idle"]
    #[inline(always)]
    pub fn txidle(&self) -> TXIDLE_R {
        TXIDLE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
