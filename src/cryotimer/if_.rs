#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `PERIOD`"]
pub type PERIOD_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Wakeup Event/Interrupt"]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new((self.bits & 0x01) != 0)
    }
}
