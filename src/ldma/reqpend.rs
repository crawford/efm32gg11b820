#[doc = "Reader of register REQPEND"]
pub type R = crate::R<u32, super::REQPEND>;
#[doc = "Reader of field `REQPEND`"]
pub type REQPEND_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - DMA Requests Pending"]
    #[inline(always)]
    pub fn reqpend(&self) -> REQPEND_R {
        REQPEND_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
