#[doc = "Reader of register PRSSTAT"]
pub type R = crate::R<u32, super::PRSSTAT>;
#[doc = "Reader of field `CMDINHIBITCMD`"]
pub type CMDINHIBITCMD_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMDINHIBITDAT`"]
pub type CMDINHIBITDAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DATLINEACTIVE`"]
pub type DATLINEACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RETUNINGREQ`"]
pub type RETUNINGREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRTRANACT`"]
pub type WRTRANACT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RDTRANACT`"]
pub type RDTRANACT_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUFFERWRITEENABLE`"]
pub type BUFFERWRITEENABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUFRDEN`"]
pub type BUFRDEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `CARDINS`"]
pub type CARDINS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CARDSTATESTABLE`"]
pub type CARDSTATESTABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CARDDETPINLVL`"]
pub type CARDDETPINLVL_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRPROTSWPINLVL`"]
pub type WRPROTSWPINLVL_R = crate::R<bool, bool>;
#[doc = "Reader of field `DAT3TO0SIGLVL`"]
pub type DAT3TO0SIGLVL_R = crate::R<u8, u8>;
#[doc = "Reader of field `CMDSIGLVL`"]
pub type CMDSIGLVL_R = crate::R<bool, bool>;
#[doc = "Reader of field `DAT7TO4SIGLVL`"]
pub type DAT7TO4SIGLVL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Command Inhibit (CMD)"]
    #[inline(always)]
    pub fn cmdinhibitcmd(&self) -> CMDINHIBITCMD_R {
        CMDINHIBITCMD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Command Inhibit (DAT)"]
    #[inline(always)]
    pub fn cmdinhibitdat(&self) -> CMDINHIBITDAT_R {
        CMDINHIBITDAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DAT Line Active"]
    #[inline(always)]
    pub fn datlineactive(&self) -> DATLINEACTIVE_R {
        DATLINEACTIVE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Re-Tuning Request"]
    #[inline(always)]
    pub fn retuningreq(&self) -> RETUNINGREQ_R {
        RETUNINGREQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Write Transfer Active"]
    #[inline(always)]
    pub fn wrtranact(&self) -> WRTRANACT_R {
        WRTRANACT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Read Transfer Active"]
    #[inline(always)]
    pub fn rdtranact(&self) -> RDTRANACT_R {
        RDTRANACT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Buffer Write Enable"]
    #[inline(always)]
    pub fn bufferwriteenable(&self) -> BUFFERWRITEENABLE_R {
        BUFFERWRITEENABLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Buffer Read Enable"]
    #[inline(always)]
    pub fn bufrden(&self) -> BUFRDEN_R {
        BUFRDEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Card Inserted Status"]
    #[inline(always)]
    pub fn cardins(&self) -> CARDINS_R {
        CARDINS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Card State Stable Status"]
    #[inline(always)]
    pub fn cardstatestable(&self) -> CARDSTATESTABLE_R {
        CARDSTATESTABLE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Card Detect Pin Level"]
    #[inline(always)]
    pub fn carddetpinlvl(&self) -> CARDDETPINLVL_R {
        CARDDETPINLVL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Write Protect Switch Pin Level"]
    #[inline(always)]
    pub fn wrprotswpinlvl(&self) -> WRPROTSWPINLVL_R {
        WRPROTSWPINLVL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - DAT\\[3:0\\] Line Signal Level"]
    #[inline(always)]
    pub fn dat3to0siglvl(&self) -> DAT3TO0SIGLVL_R {
        DAT3TO0SIGLVL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Command Line Signal Level"]
    #[inline(always)]
    pub fn cmdsiglvl(&self) -> CMDSIGLVL_R {
        CMDSIGLVL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:28 - DAT\\[7:4\\] Line Signal Level"]
    #[inline(always)]
    pub fn dat7to4siglvl(&self) -> DAT7TO4SIGLVL_R {
        DAT7TO4SIGLVL_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
