#[doc = "Reader of register RXDATAP"]
pub type R = crate::R<u32, super::RXDATAP>;
#[doc = "Reader of field `RXDATAP`"]
pub type RXDATAP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - RX Data Peek"]
    #[inline(always)]
    pub fn rxdatap(&self) -> RXDATAP_R {
        RXDATAP_R::new((self.bits & 0xff) as u8)
    }
}
