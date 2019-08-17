#[doc = "Reader of register SCANFIFOCOUNT"]
pub type R = crate::R<u32, super::SCANFIFOCOUNT>;
#[doc = "Reader of field `SCANDC`"]
pub type SCANDC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Scan Data Count"]
    #[inline(always)]
    pub fn scandc(&self) -> SCANDC_R {
        SCANDC_R::new((self.bits & 0x07) as u8)
    }
}
