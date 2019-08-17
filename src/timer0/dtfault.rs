#[doc = "Reader of register DTFAULT"]
pub type R = crate::R<u32, super::DTFAULT>;
#[doc = "Reader of field `DTPRS0F`"]
pub type DTPRS0F_R = crate::R<bool, bool>;
#[doc = "Reader of field `DTPRS1F`"]
pub type DTPRS1F_R = crate::R<bool, bool>;
#[doc = "Reader of field `DTDBGF`"]
pub type DTDBGF_R = crate::R<bool, bool>;
#[doc = "Reader of field `DTLOCKUPF`"]
pub type DTLOCKUPF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - DTI PRS 0 Fault"]
    #[inline(always)]
    pub fn dtprs0f(&self) -> DTPRS0F_R {
        DTPRS0F_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DTI PRS 1 Fault"]
    #[inline(always)]
    pub fn dtprs1f(&self) -> DTPRS1F_R {
        DTPRS1F_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DTI Debugger Fault"]
    #[inline(always)]
    pub fn dtdbgf(&self) -> DTDBGF_R {
        DTDBGF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DTI Lockup Fault"]
    #[inline(always)]
    pub fn dtlockupf(&self) -> DTLOCKUPF_R {
        DTLOCKUPF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
