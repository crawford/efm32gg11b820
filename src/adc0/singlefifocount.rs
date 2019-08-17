#[doc = "Reader of register SINGLEFIFOCOUNT"]
pub type R = crate::R<u32, super::SINGLEFIFOCOUNT>;
#[doc = "Reader of field `SINGLEDC`"]
pub type SINGLEDC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Single Data Count"]
    #[inline(always)]
    pub fn singledc(&self) -> SINGLEDC_R {
        SINGLEDC_R::new((self.bits & 0x07) as u8)
    }
}
