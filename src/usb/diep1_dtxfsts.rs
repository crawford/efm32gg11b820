#[doc = "Reader of register DIEP1_DTXFSTS"]
pub type R = crate::R<u32, super::DIEP1_DTXFSTS>;
#[doc = "Reader of field `SPCAVAIL`"]
pub type SPCAVAIL_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN Endpoint TxFIFO Space Avail"]
    #[inline(always)]
    pub fn spcavail(&self) -> SPCAVAIL_R {
        SPCAVAIL_R::new((self.bits & 0xffff) as u16)
    }
}
