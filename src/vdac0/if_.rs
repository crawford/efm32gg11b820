#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `CH0CD`"]
pub type CH0CD_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH1CD`"]
pub type CH1CD_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH0OF`"]
pub type CH0OF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH1OF`"]
pub type CH1OF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH0UF`"]
pub type CH0UF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH1UF`"]
pub type CH1UF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH0BL`"]
pub type CH0BL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH1BL`"]
pub type CH1BL_R = crate::R<bool, bool>;
#[doc = "Reader of field `EM23ERR`"]
pub type EM23ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA0APORTCONFLICT`"]
pub type OPA0APORTCONFLICT_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA1APORTCONFLICT`"]
pub type OPA1APORTCONFLICT_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA2APORTCONFLICT`"]
pub type OPA2APORTCONFLICT_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA3APORTCONFLICT`"]
pub type OPA3APORTCONFLICT_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA0PRSTIMEDERR`"]
pub type OPA0PRSTIMEDERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA1PRSTIMEDERR`"]
pub type OPA1PRSTIMEDERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA2PRSTIMEDERR`"]
pub type OPA2PRSTIMEDERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA3PRSTIMEDERR`"]
pub type OPA3PRSTIMEDERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA0OUTVALID`"]
pub type OPA0OUTVALID_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA1OUTVALID`"]
pub type OPA1OUTVALID_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA2OUTVALID`"]
pub type OPA2OUTVALID_R = crate::R<bool, bool>;
#[doc = "Reader of field `OPA3OUTVALID`"]
pub type OPA3OUTVALID_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel 0 Conversion Done Interrupt Flag"]
    #[inline(always)]
    pub fn ch0cd(&self) -> CH0CD_R {
        CH0CD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Conversion Done Interrupt Flag"]
    #[inline(always)]
    pub fn ch1cd(&self) -> CH1CD_R {
        CH1CD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 0 Data Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch0of(&self) -> CH0OF_R {
        CH0OF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 1 Data Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch1of(&self) -> CH1OF_R {
        CH1OF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 0 Data Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch0uf(&self) -> CH0UF_R {
        CH0UF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 1 Data Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch1uf(&self) -> CH1UF_R {
        CH1UF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 0 Buffer Level Interrupt Flag"]
    #[inline(always)]
    pub fn ch0bl(&self) -> CH0BL_R {
        CH0BL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 1 Buffer Level Interrupt Flag"]
    #[inline(always)]
    pub fn ch1bl(&self) -> CH1BL_R {
        CH1BL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EM2/3 Entry Error Flag"]
    #[inline(always)]
    pub fn em23err(&self) -> EM23ERR_R {
        EM23ERR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - OPA0 Bus Conflict Output Interrupt Flag"]
    #[inline(always)]
    pub fn opa0aportconflict(&self) -> OPA0APORTCONFLICT_R {
        OPA0APORTCONFLICT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - OPA1 Bus Conflict Output Interrupt Flag"]
    #[inline(always)]
    pub fn opa1aportconflict(&self) -> OPA1APORTCONFLICT_R {
        OPA1APORTCONFLICT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - OPA2 Bus Conflict Output Interrupt Flag"]
    #[inline(always)]
    pub fn opa2aportconflict(&self) -> OPA2APORTCONFLICT_R {
        OPA2APORTCONFLICT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OPA3 Bus Conflict Output Interrupt Flag"]
    #[inline(always)]
    pub fn opa3aportconflict(&self) -> OPA3APORTCONFLICT_R {
        OPA3APORTCONFLICT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - OPA0 PRS Trigger Mode Error Interrupt Flag"]
    #[inline(always)]
    pub fn opa0prstimederr(&self) -> OPA0PRSTIMEDERR_R {
        OPA0PRSTIMEDERR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - OPA1 PRS Trigger Mode Error Interrupt Flag"]
    #[inline(always)]
    pub fn opa1prstimederr(&self) -> OPA1PRSTIMEDERR_R {
        OPA1PRSTIMEDERR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - OPA2 PRS Trigger Mode Error Interrupt Flag"]
    #[inline(always)]
    pub fn opa2prstimederr(&self) -> OPA2PRSTIMEDERR_R {
        OPA2PRSTIMEDERR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - OPA3 PRS Trigger Mode Error Interrupt Flag"]
    #[inline(always)]
    pub fn opa3prstimederr(&self) -> OPA3PRSTIMEDERR_R {
        OPA3PRSTIMEDERR_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 28 - OPA0 Output Valid Interrupt Flag"]
    #[inline(always)]
    pub fn opa0outvalid(&self) -> OPA0OUTVALID_R {
        OPA0OUTVALID_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - OPA1 Output Valid Interrupt Flag"]
    #[inline(always)]
    pub fn opa1outvalid(&self) -> OPA1OUTVALID_R {
        OPA1OUTVALID_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - OPA3 Output Valid Interrupt Flag"]
    #[inline(always)]
    pub fn opa2outvalid(&self) -> OPA2OUTVALID_R {
        OPA2OUTVALID_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - OPA3 Output Valid Interrupt Flag"]
    #[inline(always)]
    pub fn opa3outvalid(&self) -> OPA3OUTVALID_R {
        OPA3OUTVALID_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
