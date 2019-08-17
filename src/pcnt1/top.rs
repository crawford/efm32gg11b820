#[doc = "Reader of register TOP"]
pub type R = crate::R<u32, super::TOP>;
#[doc = "Reader of field `TOP`"]
pub type TOP_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter Top Value"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new((self.bits & 0xffff) as u16)
    }
}
