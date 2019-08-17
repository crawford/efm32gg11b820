#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `SEQRUNNING`"]
pub type SEQRUNNING_R = crate::R<bool, bool>;
#[doc = "Reader of field `INSTRRUNNING`"]
pub type INSTRRUNNING_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMAACTIVE`"]
pub type DMAACTIVE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - AES SEQUENCE Running"]
    #[inline(always)]
    pub fn seqrunning(&self) -> SEQRUNNING_R {
        SEQRUNNING_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Action is Active"]
    #[inline(always)]
    pub fn instrrunning(&self) -> INSTRRUNNING_R {
        INSTRRUNNING_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA Action is Active"]
    #[inline(always)]
    pub fn dmaactive(&self) -> DMAACTIVE_R {
        DMAACTIVE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
