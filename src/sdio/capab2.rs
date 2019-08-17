#[doc = "Reader of register CAPAB2"]
pub type R = crate::R<u32, super::CAPAB2>;
#[doc = "Reader of field `SDR50SUP`"]
pub type SDR50SUP_R = crate::R<bool, bool>;
#[doc = "Reader of field `SDR104SUP`"]
pub type SDR104SUP_R = crate::R<bool, bool>;
#[doc = "Reader of field `DDR50SUP`"]
pub type DDR50SUP_R = crate::R<bool, bool>;
#[doc = "Reader of field `DRVTYPASUP`"]
pub type DRVTYPASUP_R = crate::R<bool, bool>;
#[doc = "Reader of field `DRVTYPCSUP`"]
pub type DRVTYPCSUP_R = crate::R<bool, bool>;
#[doc = "Reader of field `DRVTYPDSUP`"]
pub type DRVTYPDSUP_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIMCNTRETUN`"]
pub type TIMCNTRETUN_R = crate::R<u8, u8>;
#[doc = "Reader of field `USETUNSDR50`"]
pub type USETUNSDR50_R = crate::R<bool, bool>;
#[doc = "Reader of field `RETUNEMODES`"]
pub type RETUNEMODES_R = crate::R<u8, u8>;
#[doc = "Reader of field `CLOCKKMUL`"]
pub type CLOCKKMUL_R = crate::R<u8, u8>;
#[doc = "Reader of field `SPIMODE`"]
pub type SPIMODE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPIBLOCKMODE`"]
pub type SPIBLOCKMODE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SDR50 Support"]
    #[inline(always)]
    pub fn sdr50sup(&self) -> SDR50SUP_R {
        SDR50SUP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SDR104 Support"]
    #[inline(always)]
    pub fn sdr104sup(&self) -> SDR104SUP_R {
        SDR104SUP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DDR50 Support"]
    #[inline(always)]
    pub fn ddr50sup(&self) -> DDR50SUP_R {
        DDR50SUP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Driver Type a Support"]
    #[inline(always)]
    pub fn drvtypasup(&self) -> DRVTYPASUP_R {
        DRVTYPASUP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Driver Type C Support"]
    #[inline(always)]
    pub fn drvtypcsup(&self) -> DRVTYPCSUP_R {
        DRVTYPCSUP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Driver Type D Support"]
    #[inline(always)]
    pub fn drvtypdsup(&self) -> DRVTYPDSUP_R {
        DRVTYPDSUP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Timer Count for Re-Tuning"]
    #[inline(always)]
    pub fn timcntretun(&self) -> TIMCNTRETUN_R {
        TIMCNTRETUN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Use Tuning for SDR50"]
    #[inline(always)]
    pub fn usetunsdr50(&self) -> USETUNSDR50_R {
        USETUNSDR50_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Re-tuning Modes"]
    #[inline(always)]
    pub fn retunemodes(&self) -> RETUNEMODES_R {
        RETUNEMODES_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:23 - Clock Multiplier"]
    #[inline(always)]
    pub fn clockkmul(&self) -> CLOCKKMUL_R {
        CLOCKKMUL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - SPI Mode Support"]
    #[inline(always)]
    pub fn spimode(&self) -> SPIMODE_R {
        SPIMODE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - SPI Block Mode Support"]
    #[inline(always)]
    pub fn spiblockmode(&self) -> SPIBLOCKMODE_R {
        SPIBLOCKMODE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
