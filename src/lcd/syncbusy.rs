#[doc = "Reader of register SYNCBUSY"]
pub type R = crate::R<u32, super::SYNCBUSY>;
#[doc = "Reader of field `CTRL`"]
pub type CTRL_R = crate::R<bool, bool>;
#[doc = "Reader of field `BACTRL`"]
pub type BACTRL_R = crate::R<bool, bool>;
#[doc = "Reader of field `AREGA`"]
pub type AREGA_R = crate::R<bool, bool>;
#[doc = "Reader of field `AREGB`"]
pub type AREGB_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEGD0L`"]
pub type SEGD0L_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEGD1L`"]
pub type SEGD1L_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEGD2L`"]
pub type SEGD2L_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEGD3L`"]
pub type SEGD3L_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEGD0H`"]
pub type SEGD0H_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEGD1H`"]
pub type SEGD1H_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEGD2H`"]
pub type SEGD2H_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEGD3H`"]
pub type SEGD3H_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEGD4L`"]
pub type SEGD4L_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEGD5L`"]
pub type SEGD5L_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEGD6L`"]
pub type SEGD6L_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEGD7L`"]
pub type SEGD7L_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEGD4H`"]
pub type SEGD4H_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEGD5H`"]
pub type SEGD5H_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEGD6H`"]
pub type SEGD6H_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEGD7H`"]
pub type SEGD7H_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - CTRL Register Busy"]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BACTRL Register Busy"]
    #[inline(always)]
    pub fn bactrl(&self) -> BACTRL_R {
        BACTRL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AREGA Register Busy"]
    #[inline(always)]
    pub fn arega(&self) -> AREGA_R {
        AREGA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AREGB Register Busy"]
    #[inline(always)]
    pub fn aregb(&self) -> AREGB_R {
        AREGB_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SEGD0L Register Busy"]
    #[inline(always)]
    pub fn segd0l(&self) -> SEGD0L_R {
        SEGD0L_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SEGD1L Register Busy"]
    #[inline(always)]
    pub fn segd1l(&self) -> SEGD1L_R {
        SEGD1L_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SEGD2L Register Busy"]
    #[inline(always)]
    pub fn segd2l(&self) -> SEGD2L_R {
        SEGD2L_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SEGD3L Register Busy"]
    #[inline(always)]
    pub fn segd3l(&self) -> SEGD3L_R {
        SEGD3L_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SEGD0H Register Busy"]
    #[inline(always)]
    pub fn segd0h(&self) -> SEGD0H_R {
        SEGD0H_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SEGD1H Register Busy"]
    #[inline(always)]
    pub fn segd1h(&self) -> SEGD1H_R {
        SEGD1H_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SEGD2H Register Busy"]
    #[inline(always)]
    pub fn segd2h(&self) -> SEGD2H_R {
        SEGD2H_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SEGD3H Register Busy"]
    #[inline(always)]
    pub fn segd3h(&self) -> SEGD3H_R {
        SEGD3H_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SEGD4L Register Busy"]
    #[inline(always)]
    pub fn segd4l(&self) -> SEGD4L_R {
        SEGD4L_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SEGD5L Register Busy"]
    #[inline(always)]
    pub fn segd5l(&self) -> SEGD5L_R {
        SEGD5L_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SEGD6L Register Busy"]
    #[inline(always)]
    pub fn segd6l(&self) -> SEGD6L_R {
        SEGD6L_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SEGD7L Register Busy"]
    #[inline(always)]
    pub fn segd7l(&self) -> SEGD7L_R {
        SEGD7L_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SEGD4H Register Busy"]
    #[inline(always)]
    pub fn segd4h(&self) -> SEGD4H_R {
        SEGD4H_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SEGD5H Register Busy"]
    #[inline(always)]
    pub fn segd5h(&self) -> SEGD5H_R {
        SEGD5H_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SEGD6H Register Busy"]
    #[inline(always)]
    pub fn segd6h(&self) -> SEGD6H_R {
        SEGD6H_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SEGD7H Register Busy"]
    #[inline(always)]
    pub fn segd7h(&self) -> SEGD7H_R {
        SEGD7H_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
