#[doc = "Reader of register ETMPIDR3"]
pub type R = crate::R<u32, super::ETMPIDR3>;
#[doc = "Reader of field `CUSTMOD`"]
pub type CUSTMOD_R = crate::R<u8, u8>;
#[doc = "Reader of field `REVAND`"]
pub type REVAND_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Customer Modified"]
    #[inline(always)]
    pub fn custmod(&self) -> CUSTMOD_R {
        CUSTMOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RevAnd"]
    #[inline(always)]
    pub fn revand(&self) -> REVAND_R {
        REVAND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
