#[doc = "Reader of register INTID"]
pub type R = crate::R<u32, super::INTID>;
#[doc = "Reader of field `INTID`"]
pub type INTID_R = crate::R<u8, u8>;
#[doc = "Reader of field `INTSTAT`"]
pub type INTSTAT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:5 - Interrupt Identifier"]
    #[inline(always)]
    pub fn intid(&self) -> INTID_R {
        INTID_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 15 - Status Interupt"]
    #[inline(always)]
    pub fn intstat(&self) -> INTSTAT_R {
        INTSTAT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
