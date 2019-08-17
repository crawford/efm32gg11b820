#[doc = "Reader of register RAMECCADDR"]
pub type R = crate::R<u32, super::RAMECCADDR>;
#[doc = "Reader of field `RAMECCADDR`"]
pub type RAMECCADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - RAM ECC Error Address"]
    #[inline(always)]
    pub fn rameccaddr(&self) -> RAMECCADDR_R {
        RAMECCADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
