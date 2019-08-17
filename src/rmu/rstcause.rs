#[doc = "Reader of register RSTCAUSE"]
pub type R = crate::R<u32, super::RSTCAUSE>;
#[doc = "Reader of field `PORST`"]
pub type PORST_R = crate::R<bool, bool>;
#[doc = "Reader of field `AVDDBOD`"]
pub type AVDDBOD_R = crate::R<bool, bool>;
#[doc = "Reader of field `DVDDBOD`"]
pub type DVDDBOD_R = crate::R<bool, bool>;
#[doc = "Reader of field `DECBOD`"]
pub type DECBOD_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTRST`"]
pub type EXTRST_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCKUPRST`"]
pub type LOCKUPRST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYSREQRST`"]
pub type SYSREQRST_R = crate::R<bool, bool>;
#[doc = "Reader of field `WDOGRST`"]
pub type WDOGRST_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUMODERST`"]
pub type BUMODERST_R = crate::R<bool, bool>;
#[doc = "Reader of field `EM4RST`"]
pub type EM4RST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Power on Reset"]
    #[inline(always)]
    pub fn porst(&self) -> PORST_R {
        PORST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Brown Out Detector AVDD Reset"]
    #[inline(always)]
    pub fn avddbod(&self) -> AVDDBOD_R {
        AVDDBOD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Brown Out Detector DVDD Reset"]
    #[inline(always)]
    pub fn dvddbod(&self) -> DVDDBOD_R {
        DVDDBOD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Brown Out Detector Decouple Domain Reset"]
    #[inline(always)]
    pub fn decbod(&self) -> DECBOD_R {
        DECBOD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - External Pin Reset"]
    #[inline(always)]
    pub fn extrst(&self) -> EXTRST_R {
        EXTRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LOCKUP Reset"]
    #[inline(always)]
    pub fn lockuprst(&self) -> LOCKUPRST_R {
        LOCKUPRST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - System Request Reset"]
    #[inline(always)]
    pub fn sysreqrst(&self) -> SYSREQRST_R {
        SYSREQRST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Watchdog Reset"]
    #[inline(always)]
    pub fn wdogrst(&self) -> WDOGRST_R {
        WDOGRST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Backup Mode Reset"]
    #[inline(always)]
    pub fn bumoderst(&self) -> BUMODERST_R {
        BUMODERST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - EM4 Reset"]
    #[inline(always)]
    pub fn em4rst(&self) -> EM4RST_R {
        EM4RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
