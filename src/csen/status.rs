#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `CSENBUSY`"]
pub type CSENBUSY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Busy Flag"]
    #[inline(always)]
    pub fn csenbusy(&self) -> CSENBUSY_R {
        CSENBUSY_R::new((self.bits & 0x01) != 0)
    }
}
