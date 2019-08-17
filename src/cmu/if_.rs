#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `HFRCORDY`"]
pub type HFRCORDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFXORDY`"]
pub type HFXORDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFRCORDY`"]
pub type LFRCORDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFXORDY`"]
pub type LFXORDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXHFRCORDY`"]
pub type AUXHFRCORDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `CALRDY`"]
pub type CALRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `CALOF`"]
pub type CALOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `USHFRCORDY`"]
pub type USHFRCORDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFXODISERR`"]
pub type HFXODISERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFXOAUTOSW`"]
pub type HFXOAUTOSW_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFXOPEAKDETRDY`"]
pub type HFXOPEAKDETRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `HFRCODIS`"]
pub type HFRCODIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFTIMEOUTERR`"]
pub type LFTIMEOUTERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLLRDY`"]
pub type DPLLRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLLLOCKFAILLOW`"]
pub type DPLLLOCKFAILLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPLLLOCKFAILHIGH`"]
pub type DPLLLOCKFAILHIGH_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFXOEDGE`"]
pub type LFXOEDGE_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFRCOEDGE`"]
pub type LFRCOEDGE_R = crate::R<bool, bool>;
#[doc = "Reader of field `ULFRCOEDGE`"]
pub type ULFRCOEDGE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMUERR`"]
pub type CMUERR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - HFRCO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn hfrcordy(&self) -> HFRCORDY_R {
        HFRCORDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HFXO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn hfxordy(&self) -> HFXORDY_R {
        HFXORDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LFRCO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn lfrcordy(&self) -> LFRCORDY_R {
        LFRCORDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LFXO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn lfxordy(&self) -> LFXORDY_R {
        LFXORDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AUXHFRCO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AUXHFRCORDY_R {
        AUXHFRCORDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Calibration Ready Interrupt Flag"]
    #[inline(always)]
    pub fn calrdy(&self) -> CALRDY_R {
        CALRDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Calibration Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn calof(&self) -> CALOF_R {
        CALOF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USHFRCO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn ushfrcordy(&self) -> USHFRCORDY_R {
        USHFRCORDY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - HFXO Disable Error Interrupt Flag"]
    #[inline(always)]
    pub fn hfxodiserr(&self) -> HFXODISERR_R {
        HFXODISERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - HFXO Automatic Switch Interrupt Flag"]
    #[inline(always)]
    pub fn hfxoautosw(&self) -> HFXOAUTOSW_R {
        HFXOAUTOSW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HFXO Automatic Peak Detection Ready Interrupt Flag"]
    #[inline(always)]
    pub fn hfxopeakdetrdy(&self) -> HFXOPEAKDETRDY_R {
        HFXOPEAKDETRDY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - HFRCO Disable Interrupt Flag"]
    #[inline(always)]
    pub fn hfrcodis(&self) -> HFRCODIS_R {
        HFRCODIS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Low Frequency Timeout Error Interrupt Flag"]
    #[inline(always)]
    pub fn lftimeouterr(&self) -> LFTIMEOUTERR_R {
        LFTIMEOUTERR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DPLL Lock Interrupt Flag"]
    #[inline(always)]
    pub fn dpllrdy(&self) -> DPLLRDY_R {
        DPLLRDY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DPLL Lock Failure Low Interrupt Flag"]
    #[inline(always)]
    pub fn dplllockfaillow(&self) -> DPLLLOCKFAILLOW_R {
        DPLLLOCKFAILLOW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DPLL Lock Failure Low Interrupt Flag"]
    #[inline(always)]
    pub fn dplllockfailhigh(&self) -> DPLLLOCKFAILHIGH_R {
        DPLLLOCKFAILHIGH_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 27 - LFXO Clock Edge Detected Interrupt Flag"]
    #[inline(always)]
    pub fn lfxoedge(&self) -> LFXOEDGE_R {
        LFXOEDGE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - LFRCO Clock Edge Detected Interrupt Flag"]
    #[inline(always)]
    pub fn lfrcoedge(&self) -> LFRCOEDGE_R {
        LFRCOEDGE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - ULFRCO Clock Edge Detected Interrupt Flag"]
    #[inline(always)]
    pub fn ulfrcoedge(&self) -> ULFRCOEDGE_R {
        ULFRCOEDGE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - CMU Error Interrupt Flag"]
    #[inline(always)]
    pub fn cmuerr(&self) -> CMUERR_R {
        CMUERR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
