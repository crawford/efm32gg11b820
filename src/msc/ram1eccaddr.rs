#[doc = "Reader of register RAM1ECCADDR"]
pub type R = crate::R<u32, super::RAM1ECCADDR>;
#[doc = "Reader of field `RAM1ECCADDR`"]
pub type RAM1ECCADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - RAM1 ECC Error Address"]
    #[inline(always)]
    pub fn ram1eccaddr(&self) -> RAM1ECCADDR_R {
        RAM1ECCADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
