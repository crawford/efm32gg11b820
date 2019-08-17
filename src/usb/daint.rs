#[doc = "Reader of register DAINT"]
pub type R = crate::R<u32, super::DAINT>;
#[doc = "Reader of field `INEPINT0`"]
pub type INEPINT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `INEPINT1`"]
pub type INEPINT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `INEPINT2`"]
pub type INEPINT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `INEPINT3`"]
pub type INEPINT3_R = crate::R<bool, bool>;
#[doc = "Reader of field `INEPINT4`"]
pub type INEPINT4_R = crate::R<bool, bool>;
#[doc = "Reader of field `INEPINT5`"]
pub type INEPINT5_R = crate::R<bool, bool>;
#[doc = "Reader of field `INEPINT6`"]
pub type INEPINT6_R = crate::R<bool, bool>;
#[doc = "Reader of field `OUTEPINT0`"]
pub type OUTEPINT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `OUTEPINT1`"]
pub type OUTEPINT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `OUTEPINT2`"]
pub type OUTEPINT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `OUTEPINT3`"]
pub type OUTEPINT3_R = crate::R<bool, bool>;
#[doc = "Reader of field `OUTEPINT4`"]
pub type OUTEPINT4_R = crate::R<bool, bool>;
#[doc = "Reader of field `OUTEPINT5`"]
pub type OUTEPINT5_R = crate::R<bool, bool>;
#[doc = "Reader of field `OUTEPINT6`"]
pub type OUTEPINT6_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn inepint0(&self) -> INEPINT0_R {
        INEPINT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn inepint1(&self) -> INEPINT1_R {
        INEPINT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn inepint2(&self) -> INEPINT2_R {
        INEPINT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn inepint3(&self) -> INEPINT3_R {
        INEPINT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn inepint4(&self) -> INEPINT4_R {
        INEPINT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn inepint5(&self) -> INEPINT5_R {
        INEPINT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn inepint6(&self) -> INEPINT6_R {
        INEPINT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn outepint0(&self) -> OUTEPINT0_R {
        OUTEPINT0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn outepint1(&self) -> OUTEPINT1_R {
        OUTEPINT1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn outepint2(&self) -> OUTEPINT2_R {
        OUTEPINT2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn outepint3(&self) -> OUTEPINT3_R {
        OUTEPINT3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn outepint4(&self) -> OUTEPINT4_R {
        OUTEPINT4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn outepint5(&self) -> OUTEPINT5_R {
        OUTEPINT5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn outepint6(&self) -> OUTEPINT6_R {
        OUTEPINT6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
