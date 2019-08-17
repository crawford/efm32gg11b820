#[doc = "Reader of register CNT"]
pub type R = crate::R<u32, super::CNT>;
#[doc = "Reader of field `CNT`"]
pub type CNT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter Value"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
