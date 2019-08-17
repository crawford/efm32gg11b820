#[doc = "Reader of register RXDOUBLEX"]
pub type R = crate::R<u32, super::RXDOUBLEX>;
#[doc = "Reader of field `RXDATA0`"]
pub type RXDATA0_R = crate::R<u16, u16>;
#[doc = "Reader of field `PERR0`"]
pub type PERR0_R = crate::R<bool, bool>;
#[doc = "Reader of field `FERR0`"]
pub type FERR0_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXDATA1`"]
pub type RXDATA1_R = crate::R<u16, u16>;
#[doc = "Reader of field `PERR1`"]
pub type PERR1_R = crate::R<bool, bool>;
#[doc = "Reader of field `FERR1`"]
pub type FERR1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:8 - RX Data 0"]
    #[inline(always)]
    pub fn rxdata0(&self) -> RXDATA0_R {
        RXDATA0_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 14 - Data Parity Error 0"]
    #[inline(always)]
    pub fn perr0(&self) -> PERR0_R {
        PERR0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Data Framing Error 0"]
    #[inline(always)]
    pub fn ferr0(&self) -> FERR0_R {
        FERR0_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:24 - RX Data 1"]
    #[inline(always)]
    pub fn rxdata1(&self) -> RXDATA1_R {
        RXDATA1_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 30 - Data Parity Error 1"]
    #[inline(always)]
    pub fn perr1(&self) -> PERR1_R {
        PERR1_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Data Framing Error 1"]
    #[inline(always)]
    pub fn ferr1(&self) -> FERR1_R {
        FERR1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
