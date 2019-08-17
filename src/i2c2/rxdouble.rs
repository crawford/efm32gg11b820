#[doc = "Reader of register RXDOUBLE"]
pub type R = crate::R<u32, super::RXDOUBLE>;
#[doc = "Reader of field `RXDATA0`"]
pub type RXDATA0_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXDATA1`"]
pub type RXDATA1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - RX Data 0"]
    #[inline(always)]
    pub fn rxdata0(&self) -> RXDATA0_R {
        RXDATA0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RX Data 1"]
    #[inline(always)]
    pub fn rxdata1(&self) -> RXDATA1_R {
        RXDATA1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
