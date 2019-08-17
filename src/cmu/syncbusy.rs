#[doc = "Reader of register SYNCBUSY"]
pub type R = crate::R<u32, super::SYNCBUSY>;
#[doc = "Reader of field `LFACLKEN0`"]
pub type LFACLKEN0_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFAPRESC0`"]
pub type LFAPRESC0_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFBCLKEN0`"]
pub type LFBCLKEN0_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFBPRESC0`"]
pub type LFBPRESC0_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFCCLKEN0`"]
pub type LFCCLKEN0_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFECLKEN0`"]
pub type LFECLKEN0_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFEPRESC0`"]
pub type LFEPRESC0_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFRCOBSY`"]
pub type HFRCOBSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXHFRCOBSY`"]
pub type AUXHFRCOBSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFRCOBSY`"]
pub type LFRCOBSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFRCOVREFBSY`"]
pub type LFRCOVREFBSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFXOBSY`"]
pub type HFXOBSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFXOBSY`"]
pub type LFXOBSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `USHFRCOBSY`"]
pub type USHFRCOBSY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Low Frequency a Clock Enable 0 Busy"]
    #[inline(always)]
    pub fn lfaclken0(&self) -> LFACLKEN0_R {
        LFACLKEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Low Frequency a Prescaler 0 Busy"]
    #[inline(always)]
    pub fn lfapresc0(&self) -> LFAPRESC0_R {
        LFAPRESC0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Low Frequency B Clock Enable 0 Busy"]
    #[inline(always)]
    pub fn lfbclken0(&self) -> LFBCLKEN0_R {
        LFBCLKEN0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Low Frequency B Prescaler 0 Busy"]
    #[inline(always)]
    pub fn lfbpresc0(&self) -> LFBPRESC0_R {
        LFBPRESC0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Low Frequency C Clock Enable 0 Busy"]
    #[inline(always)]
    pub fn lfcclken0(&self) -> LFCCLKEN0_R {
        LFCCLKEN0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Low Frequency E Clock Enable 0 Busy"]
    #[inline(always)]
    pub fn lfeclken0(&self) -> LFECLKEN0_R {
        LFECLKEN0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Low Frequency E Prescaler 0 Busy"]
    #[inline(always)]
    pub fn lfepresc0(&self) -> LFEPRESC0_R {
        LFEPRESC0_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 24 - HFRCO Busy"]
    #[inline(always)]
    pub fn hfrcobsy(&self) -> HFRCOBSY_R {
        HFRCOBSY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - AUXHFRCO Busy"]
    #[inline(always)]
    pub fn auxhfrcobsy(&self) -> AUXHFRCOBSY_R {
        AUXHFRCOBSY_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - LFRCO Busy"]
    #[inline(always)]
    pub fn lfrcobsy(&self) -> LFRCOBSY_R {
        LFRCOBSY_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - LFRCO VREF Busy"]
    #[inline(always)]
    pub fn lfrcovrefbsy(&self) -> LFRCOVREFBSY_R {
        LFRCOVREFBSY_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - HFXO Busy"]
    #[inline(always)]
    pub fn hfxobsy(&self) -> HFXOBSY_R {
        HFXOBSY_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - LFXO Busy"]
    #[inline(always)]
    pub fn lfxobsy(&self) -> LFXOBSY_R {
        LFXOBSY_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - USHFRCO Busy"]
    #[inline(always)]
    pub fn ushfrcobsy(&self) -> USHFRCOBSY_R {
        USHFRCOBSY_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
