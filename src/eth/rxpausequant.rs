#[doc = "Reader of register RXPAUSEQUANT"]
pub type R = crate::R<u32, super::RXPAUSEQUANT>;
#[doc = "Reader of field `QUANT`"]
pub type QUANT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Received pause quantum"]
    #[inline(always)]
    pub fn quant(&self) -> QUANT_R {
        QUANT_R::new((self.bits & 0xffff) as u16)
    }
}
