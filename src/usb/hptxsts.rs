#[doc = "Reader of register HPTXSTS"]
pub type R = crate::R<u32, super::HPTXSTS>;
#[doc = "Reader of field `PTXFSPCAVAIL`"]
pub type PTXFSPCAVAIL_R = crate::R<u16, u16>;
#[doc = "Reader of field `PTXQSPCAVAIL`"]
pub type PTXQSPCAVAIL_R = crate::R<u8, u8>;
#[doc = "Reader of field `PTXQTOP`"]
pub type PTXQTOP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Periodic Transmit Data FIFO Space Available"]
    #[inline(always)]
    pub fn ptxfspcavail(&self) -> PTXFSPCAVAIL_R {
        PTXFSPCAVAIL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Periodic Transmit Request Queue Space Available"]
    #[inline(always)]
    pub fn ptxqspcavail(&self) -> PTXQSPCAVAIL_R {
        PTXQSPCAVAIL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Top of the Periodic Transmit Request Queue"]
    #[inline(always)]
    pub fn ptxqtop(&self) -> PTXQTOP_R {
        PTXQTOP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
