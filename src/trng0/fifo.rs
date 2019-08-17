#[doc = "Reader of register FIFO"]
pub type R = crate::R<u32, super::FIFO>;
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - FIFO Read Data"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
