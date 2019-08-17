#[doc = "Reader of register GSNPSID"]
pub type R = crate::R<u32, super::GSNPSID>;
#[doc = "Reader of field `SYNOPSYSID`"]
pub type SYNOPSYSID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn synopsysid(&self) -> SYNOPSYSID_R {
        SYNOPSYSID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
