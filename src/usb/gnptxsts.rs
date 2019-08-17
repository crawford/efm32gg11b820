#[doc = "Reader of register GNPTXSTS"]
pub type R = crate::R<u32, super::GNPTXSTS>;
#[doc = "Reader of field `NPTXFSPCAVAIL`"]
pub type NPTXFSPCAVAIL_R = crate::R<u16, u16>;
#[doc = "Reader of field `NPTXQSPCAVAIL`"]
pub type NPTXQSPCAVAIL_R = crate::R<u8, u8>;
#[doc = "Reader of field `NPTXQTOP`"]
pub type NPTXQTOP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Non-periodic TxFIFO Space Avail"]
    #[inline(always)]
    pub fn nptxfspcavail(&self) -> NPTXFSPCAVAIL_R {
        NPTXFSPCAVAIL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Non-periodic Transmit Request Queue Space Available"]
    #[inline(always)]
    pub fn nptxqspcavail(&self) -> NPTXQSPCAVAIL_R {
        NPTXQSPCAVAIL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Top of the Non-periodic Transmit Request Queue"]
    #[inline(always)]
    pub fn nptxqtop(&self) -> NPTXQTOP_R {
        NPTXQTOP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
