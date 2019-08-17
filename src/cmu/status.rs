#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `HFRCOENS`"]
pub type HFRCOENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFRCORDY`"]
pub type HFRCORDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFXOENS`"]
pub type HFXOENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFXORDY`"]
pub type HFXORDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXHFRCOENS`"]
pub type AUXHFRCOENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXHFRCORDY`"]
pub type AUXHFRCORDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFRCOENS`"]
pub type LFRCOENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFRCORDY`"]
pub type LFRCORDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFXOENS`"]
pub type LFXOENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFXORDY`"]
pub type LFXORDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `USHFRCOENS`"]
pub type USHFRCOENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `USHFRCORDY`"]
pub type USHFRCORDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLLENS`"]
pub type DPLLENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLLRDY`"]
pub type DPLLRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `CALRDY`"]
pub type CALRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `SDIOCLKENS`"]
pub type SDIOCLKENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `QSPI0CLKENS`"]
pub type QSPI0CLKENS_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFXOPEAKDETRDY`"]
pub type HFXOPEAKDETRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFXOAMPLOW`"]
pub type HFXOAMPLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFXOPHASE`"]
pub type LFXOPHASE_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFRCOPHASE`"]
pub type LFRCOPHASE_R = crate::R<bool, bool>;
#[doc = "Reader of field `ULFRCOPHASE`"]
pub type ULFRCOPHASE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - HFRCO Enable Status"]
    #[inline(always)]
    pub fn hfrcoens(&self) -> HFRCOENS_R {
        HFRCOENS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HFRCO Ready"]
    #[inline(always)]
    pub fn hfrcordy(&self) -> HFRCORDY_R {
        HFRCORDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HFXO Enable Status"]
    #[inline(always)]
    pub fn hfxoens(&self) -> HFXOENS_R {
        HFXOENS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HFXO Ready"]
    #[inline(always)]
    pub fn hfxordy(&self) -> HFXORDY_R {
        HFXORDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AUXHFRCO Enable Status"]
    #[inline(always)]
    pub fn auxhfrcoens(&self) -> AUXHFRCOENS_R {
        AUXHFRCOENS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AUXHFRCO Ready"]
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AUXHFRCORDY_R {
        AUXHFRCORDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LFRCO Enable Status"]
    #[inline(always)]
    pub fn lfrcoens(&self) -> LFRCOENS_R {
        LFRCOENS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LFRCO Ready"]
    #[inline(always)]
    pub fn lfrcordy(&self) -> LFRCORDY_R {
        LFRCORDY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LFXO Enable Status"]
    #[inline(always)]
    pub fn lfxoens(&self) -> LFXOENS_R {
        LFXOENS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LFXO Ready"]
    #[inline(always)]
    pub fn lfxordy(&self) -> LFXORDY_R {
        LFXORDY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USHFRCO Enable Status"]
    #[inline(always)]
    pub fn ushfrcoens(&self) -> USHFRCOENS_R {
        USHFRCOENS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USHFRCO Ready"]
    #[inline(always)]
    pub fn ushfrcordy(&self) -> USHFRCORDY_R {
        USHFRCORDY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DPLL Enable Status"]
    #[inline(always)]
    pub fn dpllens(&self) -> DPLLENS_R {
        DPLLENS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DPLL Ready"]
    #[inline(always)]
    pub fn dpllrdy(&self) -> DPLLRDY_R {
        DPLLRDY_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Calibration Ready"]
    #[inline(always)]
    pub fn calrdy(&self) -> CALRDY_R {
        CALRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SDIO Clock Enabled Status"]
    #[inline(always)]
    pub fn sdioclkens(&self) -> SDIOCLKENS_R {
        SDIOCLKENS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - QSPI0 Clock Enabled Status"]
    #[inline(always)]
    pub fn qspi0clkens(&self) -> QSPI0CLKENS_R {
        QSPI0CLKENS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 22 - HFXO Peak Detection Ready"]
    #[inline(always)]
    pub fn hfxopeakdetrdy(&self) -> HFXOPEAKDETRDY_R {
        HFXOPEAKDETRDY_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 25 - HFXO Amplitude Tuning Value Too Low"]
    #[inline(always)]
    pub fn hfxoamplow(&self) -> HFXOAMPLOW_R {
        HFXOAMPLOW_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - LFXO Clock Phase"]
    #[inline(always)]
    pub fn lfxophase(&self) -> LFXOPHASE_R {
        LFXOPHASE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - LFRCO Clock Phase"]
    #[inline(always)]
    pub fn lfrcophase(&self) -> LFRCOPHASE_R {
        LFRCOPHASE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - ULFRCO Clock Phase"]
    #[inline(always)]
    pub fn ulfrcophase(&self) -> ULFRCOPHASE_R {
        ULFRCOPHASE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
