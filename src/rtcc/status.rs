#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `BUMODETS`"]
pub type BUMODETS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Timestamp for Backup Mode Entry Stored"]
    #[inline(always)]
    pub fn bumodets(&self) -> BUMODETS_R {
        BUMODETS_R::new((self.bits & 0x01) != 0)
    }
}
