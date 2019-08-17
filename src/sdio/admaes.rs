#[doc = "Reader of register ADMAES"]
pub type R = crate::R<u32, super::ADMAES>;
#[doc = "Reader of field `ADMAES`"]
pub type ADMAES_R = crate::R<u8, u8>;
#[doc = "Reader of field `ADMALME`"]
pub type ADMALME_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:1 - ADMA Error State"]
    #[inline(always)]
    pub fn admaes(&self) -> ADMAES_R {
        ADMAES_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - ADMA Length Mismatch Error"]
    #[inline(always)]
    pub fn admalme(&self) -> ADMALME_R {
        ADMALME_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
