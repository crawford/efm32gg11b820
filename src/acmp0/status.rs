#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `ACMPACT`"]
pub type ACMPACT_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACMPOUT`"]
pub type ACMPOUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `APORTCONFLICT`"]
pub type APORTCONFLICT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTIFACT`"]
pub type EXTIFACT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Analog Comparator Active"]
    #[inline(always)]
    pub fn acmpact(&self) -> ACMPACT_R {
        ACMPACT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Analog Comparator Output"]
    #[inline(always)]
    pub fn acmpout(&self) -> ACMPOUT_R {
        ACMPOUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - APORT Conflict Output"]
    #[inline(always)]
    pub fn aportconflict(&self) -> APORTCONFLICT_R {
        APORTCONFLICT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Override Interface Active"]
    #[inline(always)]
    pub fn extifact(&self) -> EXTIFACT_R {
        EXTIFACT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
