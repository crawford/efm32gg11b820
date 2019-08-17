#[doc = "Reader of register SYNCBUSY"]
pub type R = crate::R<u32, super::SYNCBUSY>;
#[doc = "Reader of field `CTRL`"]
pub type CTRL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMD`"]
pub type CMD_R = crate::R<bool, bool>;
#[doc = "Reader of field `PCH0_PRSCTRL`"]
pub type PCH0_PRSCTRL_R = crate::R<bool, bool>;
#[doc = "Reader of field `PCH1_PRSCTRL`"]
pub type PCH1_PRSCTRL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - CTRL Register Busy"]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CMD Register Busy"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PCH0_PRSCTRL Register Busy"]
    #[inline(always)]
    pub fn pch0_prsctrl(&self) -> PCH0_PRSCTRL_R {
        PCH0_PRSCTRL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PCH1_PRSCTRL Register Busy"]
    #[inline(always)]
    pub fn pch1_prsctrl(&self) -> PCH1_PRSCTRL_R {
        PCH1_PRSCTRL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
