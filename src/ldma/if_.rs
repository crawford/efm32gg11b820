#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<u32, u32>;
#[doc = "Reader of field `ERROR`"]
pub type ERROR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:23 - DMA Structure Operation Done Interrupt Flag"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 31 - Transfer Error Interrupt Flag"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
