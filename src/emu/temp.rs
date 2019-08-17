#[doc = "Reader of register TEMP"]
pub type R = crate::R<u32, super::TEMP>;
#[doc = "Reader of field `TEMP`"]
pub type TEMP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Temperature Measurement"]
    #[inline(always)]
    pub fn temp(&self) -> TEMP_R {
        TEMP_R::new((self.bits & 0xff) as u8)
    }
}
