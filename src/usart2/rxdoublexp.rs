#[doc = "Reader of register RXDOUBLEXP"]
pub type R = crate::R<u32, super::RXDOUBLEXP>;
#[doc = "Reader of field `RXDATAP0`"]
pub type RXDATAP0_R = crate::R<u16, u16>;
#[doc = "Reader of field `PERRP0`"]
pub type PERRP0_R = crate::R<bool, bool>;
#[doc = "Reader of field `FERRP0`"]
pub type FERRP0_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXDATAP1`"]
pub type RXDATAP1_R = crate::R<u16, u16>;
#[doc = "Reader of field `PERRP1`"]
pub type PERRP1_R = crate::R<bool, bool>;
#[doc = "Reader of field `FERRP1`"]
pub type FERRP1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:8 - RX Data 0 Peek"]
    #[inline(always)]
    pub fn rxdatap0(&self) -> RXDATAP0_R {
        RXDATAP0_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 14 - Data Parity Error 0 Peek"]
    #[inline(always)]
    pub fn perrp0(&self) -> PERRP0_R {
        PERRP0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Data Framing Error 0 Peek"]
    #[inline(always)]
    pub fn ferrp0(&self) -> FERRP0_R {
        FERRP0_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:24 - RX Data 1 Peek"]
    #[inline(always)]
    pub fn rxdatap1(&self) -> RXDATAP1_R {
        RXDATAP1_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 30 - Data Parity Error 1 Peek"]
    #[inline(always)]
    pub fn perrp1(&self) -> PERRP1_R {
        PERRP1_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Data Framing Error 1 Peek"]
    #[inline(always)]
    pub fn ferrp1(&self) -> FERRP1_R {
        FERRP1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
