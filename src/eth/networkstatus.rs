#[doc = "Reader of register NETWORKSTATUS"]
pub type R = crate::R<u32, super::NETWORKSTATUS>;
#[doc = "Reader of field `MDIOIN`"]
pub type MDIOIN_R = crate::R<bool, bool>;
#[doc = "Reader of field `MANDONE`"]
pub type MANDONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `PFCNEGOTIATE`"]
pub type PFCNEGOTIATE_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPIINDICATE`"]
pub type LPIINDICATE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - Returns status of the mdio_in pin."]
    #[inline(always)]
    pub fn mdioin(&self) -> MDIOIN_R {
        MDIOIN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The PHY management logic is idle (i.e. has completed)."]
    #[inline(always)]
    pub fn mandone(&self) -> MANDONE_R {
        MANDONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Set when PFC Priority Based Pause has been negotiated."]
    #[inline(always)]
    pub fn pfcnegotiate(&self) -> PFCNEGOTIATE_R {
        PFCNEGOTIATE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LPI Indication"]
    #[inline(always)]
    pub fn lpiindicate(&self) -> LPIINDICATE_R {
        LPIINDICATE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
