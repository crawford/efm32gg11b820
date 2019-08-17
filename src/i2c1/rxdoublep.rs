#[doc = "Reader of register RXDOUBLEP"]
pub type R = crate::R<u32, super::RXDOUBLEP>;
#[doc = "Reader of field `RXDATAP0`"]
pub type RXDATAP0_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXDATAP1`"]
pub type RXDATAP1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - RX Data 0 Peek"]
    #[inline(always)]
    pub fn rxdatap0(&self) -> RXDATAP0_R {
        RXDATAP0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RX Data 1 Peek"]
    #[inline(always)]
    pub fn rxdatap1(&self) -> RXDATAP1_R {
        RXDATAP1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
