#[doc = "Reader of register ETMCIDR2"]
pub type R = crate::R<u32, super::ETMCIDR2>;
#[doc = "Reader of field `PREAMB`"]
pub type PREAMB_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - CoreSight Preamble"]
    #[inline(always)]
    pub fn preamb(&self) -> PREAMB_R {
        PREAMB_R::new((self.bits & 0xff) as u8)
    }
}
