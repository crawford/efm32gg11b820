#[doc = "Reader of register R5VSYNC"]
pub type R = crate::R<u32, super::R5VSYNC>;
#[doc = "Reader of field `OUTLEVELBUSY`"]
pub type OUTLEVELBUSY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - 5V Regulator Voltage Register Transfer Busy"]
    #[inline(always)]
    pub fn outlevelbusy(&self) -> OUTLEVELBUSY_R {
        OUTLEVELBUSY_R::new((self.bits & 0x01) != 0)
    }
}
