#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Current Counter Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits & 0x01) != 0)
    }
}
