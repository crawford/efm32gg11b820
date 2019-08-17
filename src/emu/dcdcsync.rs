#[doc = "Reader of register DCDCSYNC"]
pub type R = crate::R<u32, super::DCDCSYNC>;
#[doc = "Reader of field `DCDCCTRLBUSY`"]
pub type DCDCCTRLBUSY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - DCDC CTRL Register Transfer Busy"]
    #[inline(always)]
    pub fn dcdcctrlbusy(&self) -> DCDCCTRLBUSY_R {
        DCDCCTRLBUSY_R::new((self.bits & 0x01) != 0)
    }
}
