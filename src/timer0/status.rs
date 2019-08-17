#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `RUNNING`"]
pub type RUNNING_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, bool>;
#[doc = "Reader of field `TOPBV`"]
pub type TOPBV_R = crate::R<bool, bool>;
#[doc = "Reader of field `CCVBV0`"]
pub type CCVBV0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CCVBV1`"]
pub type CCVBV1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CCVBV2`"]
pub type CCVBV2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CCVBV3`"]
pub type CCVBV3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ICV0`"]
pub type ICV0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ICV1`"]
pub type ICV1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ICV2`"]
pub type ICV2_R = crate::R<bool, bool>;
#[doc = "Reader of field `ICV3`"]
pub type ICV3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CCPOL0`"]
pub type CCPOL0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CCPOL1`"]
pub type CCPOL1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CCPOL2`"]
pub type CCPOL2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CCPOL3`"]
pub type CCPOL3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Running"]
    #[inline(always)]
    pub fn running(&self) -> RUNNING_R {
        RUNNING_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TOPB Valid"]
    #[inline(always)]
    pub fn topbv(&self) -> TOPBV_R {
        TOPBV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CC0 CCVB Valid"]
    #[inline(always)]
    pub fn ccvbv0(&self) -> CCVBV0_R {
        CCVBV0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CC1 CCVB Valid"]
    #[inline(always)]
    pub fn ccvbv1(&self) -> CCVBV1_R {
        CCVBV1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CC2 CCVB Valid"]
    #[inline(always)]
    pub fn ccvbv2(&self) -> CCVBV2_R {
        CCVBV2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CC3 CCVB Valid"]
    #[inline(always)]
    pub fn ccvbv3(&self) -> CCVBV3_R {
        CCVBV3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CC0 Input Capture Valid"]
    #[inline(always)]
    pub fn icv0(&self) -> ICV0_R {
        ICV0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CC1 Input Capture Valid"]
    #[inline(always)]
    pub fn icv1(&self) -> ICV1_R {
        ICV1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CC2 Input Capture Valid"]
    #[inline(always)]
    pub fn icv2(&self) -> ICV2_R {
        ICV2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CC3 Input Capture Valid"]
    #[inline(always)]
    pub fn icv3(&self) -> ICV3_R {
        ICV3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - CC0 Polarity"]
    #[inline(always)]
    pub fn ccpol0(&self) -> CCPOL0_R {
        CCPOL0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CC1 Polarity"]
    #[inline(always)]
    pub fn ccpol1(&self) -> CCPOL1_R {
        CCPOL1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - CC2 Polarity"]
    #[inline(always)]
    pub fn ccpol2(&self) -> CCPOL2_R {
        CCPOL2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - CC3 Polarity"]
    #[inline(always)]
    pub fn ccpol3(&self) -> CCPOL3_R {
        CCPOL3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
