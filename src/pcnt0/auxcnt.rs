#[doc = "Reader of register AUXCNT"]
pub type R = crate::R<u32, super::AUXCNT>;
#[doc = "Reader of field `AUXCNT`"]
pub type AUXCNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Auxiliary Counter Value"]
    #[inline(always)]
    pub fn auxcnt(&self) -> AUXCNT_R {
        AUXCNT_R::new((self.bits & 0xffff) as u16)
    }
}
