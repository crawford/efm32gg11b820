#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `RUNNING`"]
pub type RUNNING_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - LETIMER Running"]
    #[inline(always)]
    pub fn running(&self) -> RUNNING_R {
        RUNNING_R::new((self.bits & 0x01) != 0)
    }
}
