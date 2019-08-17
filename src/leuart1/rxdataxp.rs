#[doc = "Reader of register RXDATAXP"]
pub type R = crate::R<u32, super::RXDATAXP>;
#[doc = "Reader of field `RXDATAP`"]
pub type RXDATAP_R = crate::R<u16, u16>;
#[doc = "Reader of field `PERRP`"]
pub type PERRP_R = crate::R<bool, bool>;
#[doc = "Reader of field `FERRP`"]
pub type FERRP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:8 - RX Data Peek"]
    #[inline(always)]
    pub fn rxdatap(&self) -> RXDATAP_R {
        RXDATAP_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 14 - Receive Data Parity Error Peek"]
    #[inline(always)]
    pub fn perrp(&self) -> PERRP_R {
        PERRP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Receive Data Framing Error Peek"]
    #[inline(always)]
    pub fn ferrp(&self) -> FERRP_R {
        FERRP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
