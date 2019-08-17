#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `CH0ENS`"]
pub type CH0ENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH1ENS`"]
pub type CH1ENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH0BL`"]
pub type CH0BL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH1BL`"]
pub type CH1BL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH0WARM`"]
pub type CH0WARM_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH1WARM`"]
pub type CH1WARM_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA0APORTCONFLICT`"]
pub type OPA0APORTCONFLICT_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA1APORTCONFLICT`"]
pub type OPA1APORTCONFLICT_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA2APORTCONFLICT`"]
pub type OPA2APORTCONFLICT_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA3APORTCONFLICT`"]
pub type OPA3APORTCONFLICT_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA0ENS`"]
pub type OPA0ENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA1ENS`"]
pub type OPA1ENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA2ENS`"]
pub type OPA2ENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA3ENS`"]
pub type OPA3ENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA0WARM`"]
pub type OPA0WARM_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA1WARM`"]
pub type OPA1WARM_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA2WARM`"]
pub type OPA2WARM_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA3WARM`"]
pub type OPA3WARM_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA0OUTVALID`"]
pub type OPA0OUTVALID_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA1OUTVALID`"]
pub type OPA1OUTVALID_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA2OUTVALID`"]
pub type OPA2OUTVALID_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA3OUTVALID`"]
pub type OPA3OUTVALID_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel 0 Enabled Status"]
    #[inline(always)]
    pub fn ch0ens(&self) -> CH0ENS_R {
        CH0ENS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Enabled Status"]
    #[inline(always)]
    pub fn ch1ens(&self) -> CH1ENS_R {
        CH1ENS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 0 Buffer Level"]
    #[inline(always)]
    pub fn ch0bl(&self) -> CH0BL_R {
        CH0BL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 1 Buffer Level"]
    #[inline(always)]
    pub fn ch1bl(&self) -> CH1BL_R {
        CH1BL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 0 Warm"]
    #[inline(always)]
    pub fn ch0warm(&self) -> CH0WARM_R {
        CH0WARM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 1 Warm"]
    #[inline(always)]
    pub fn ch1warm(&self) -> CH1WARM_R {
        CH1WARM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 16 - OPA0 Bus Conflict Output"]
    #[inline(always)]
    pub fn opa0aportconflict(&self) -> OPA0APORTCONFLICT_R {
        OPA0APORTCONFLICT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - OPA1 Bus Conflict Output"]
    #[inline(always)]
    pub fn opa1aportconflict(&self) -> OPA1APORTCONFLICT_R {
        OPA1APORTCONFLICT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - OPA2 Bus Conflict Output"]
    #[inline(always)]
    pub fn opa2aportconflict(&self) -> OPA2APORTCONFLICT_R {
        OPA2APORTCONFLICT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OPA3 Bus Conflict Output"]
    #[inline(always)]
    pub fn opa3aportconflict(&self) -> OPA3APORTCONFLICT_R {
        OPA3APORTCONFLICT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - OPA0 Enabled Status"]
    #[inline(always)]
    pub fn opa0ens(&self) -> OPA0ENS_R {
        OPA0ENS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - OPA1 Enabled Status"]
    #[inline(always)]
    pub fn opa1ens(&self) -> OPA1ENS_R {
        OPA1ENS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - OPA2 Enabled Status"]
    #[inline(always)]
    pub fn opa2ens(&self) -> OPA2ENS_R {
        OPA2ENS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - OPA3 Enabled Status"]
    #[inline(always)]
    pub fn opa3ens(&self) -> OPA3ENS_R {
        OPA3ENS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - OPA0 Warm Status"]
    #[inline(always)]
    pub fn opa0warm(&self) -> OPA0WARM_R {
        OPA0WARM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - OPA1 Warm Status"]
    #[inline(always)]
    pub fn opa1warm(&self) -> OPA1WARM_R {
        OPA1WARM_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - OPA2 Warm Status"]
    #[inline(always)]
    pub fn opa2warm(&self) -> OPA2WARM_R {
        OPA2WARM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - OPA3 Warm Status"]
    #[inline(always)]
    pub fn opa3warm(&self) -> OPA3WARM_R {
        OPA3WARM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - OPA0 Output Valid Status"]
    #[inline(always)]
    pub fn opa0outvalid(&self) -> OPA0OUTVALID_R {
        OPA0OUTVALID_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - OPA1 Output Valid Status"]
    #[inline(always)]
    pub fn opa1outvalid(&self) -> OPA1OUTVALID_R {
        OPA1OUTVALID_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - OPA2 Output Valid Status"]
    #[inline(always)]
    pub fn opa2outvalid(&self) -> OPA2OUTVALID_R {
        OPA2OUTVALID_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - OPA3 Output Valid Status"]
    #[inline(always)]
    pub fn opa3outvalid(&self) -> OPA3OUTVALID_R {
        OPA3OUTVALID_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
