#[doc = "Reader of register SYNCBUSY"]
pub type R = crate::R<u32, super::SYNCBUSY>;
#[doc = "Reader of field `CTRL`"]
pub type CTRL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMD`"]
pub type CMD_R = crate::R<bool, bool>;
#[doc = "Reader of field `TOPB`"]
pub type TOPB_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVSCFG`"]
pub type OVSCFG_R = crate::R<bool, bool>;
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
    #[doc = "Bit 2 - TOPB Register Busy"]
    #[inline(always)]
    pub fn topb(&self) -> TOPB_R {
        TOPB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OVSCFG Register Busy"]
    #[inline(always)]
    pub fn ovscfg(&self) -> OVSCFG_R {
        OVSCFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
