#[doc = "Reader of register IF1IF"]
pub type R = crate::R<u32, super::IF1IF>;
#[doc = "Reader of field `STATUS`"]
pub type STATUS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Status Interrupt Flag"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0x01) != 0)
    }
}
