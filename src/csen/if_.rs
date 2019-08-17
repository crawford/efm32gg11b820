#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `CMP`"]
pub type CMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `CONV`"]
pub type CONV_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOS`"]
pub type EOS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMAOF`"]
pub type DMAOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `APORTCONFLICT`"]
pub type APORTCONFLICT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Digital Comparator Interrupt Flag"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Conversion Done Interrupt Flag"]
    #[inline(always)]
    pub fn conv(&self) -> CONV_R {
        CONV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - End of Scan Interrupt Flag."]
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA Overflow Interrupt Flag."]
    #[inline(always)]
    pub fn dmaof(&self) -> DMAOF_R {
        DMAOF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - APORT Conflict Interrupt Flag"]
    #[inline(always)]
    pub fn aportconflict(&self) -> APORTCONFLICT_R {
        APORTCONFLICT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
