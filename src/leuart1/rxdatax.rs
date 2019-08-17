#[doc = "Reader of register RXDATAX"]
pub type R = crate::R<u32, super::RXDATAX>;
#[doc = "Reader of field `RXDATA`"]
pub type RXDATA_R = crate::R<u16, u16>;
#[doc = "Reader of field `PERR`"]
pub type PERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `FERR`"]
pub type FERR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:8 - RX Data"]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 14 - Receive Data Parity Error"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Receive Data Framing Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
