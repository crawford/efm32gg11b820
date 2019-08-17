#[doc = "Reader of register GINTSTS"]
pub type R = crate::R<u32, super::GINTSTS>;
#[doc = "Writer for register GINTSTS"]
pub type W = crate::W<u32, super::GINTSTS>;
#[doc = "Register GINTSTS `reset()`'s with value 0x1400_0020"]
impl crate::ResetValue for super::GINTSTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1400_0020
    }
}
#[doc = "Reader of field `CURMOD`"]
pub type CURMOD_R = crate::R<bool, bool>;
#[doc = "Reader of field `MODEMIS`"]
pub type MODEMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODEMIS`"]
pub struct MODEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MODEMIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `OTGINT`"]
pub type OTGINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `SOF`"]
pub type SOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOF`"]
pub struct SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `RXFLVL`"]
pub type RXFLVL_R = crate::R<bool, bool>;
#[doc = "Reader of field `NPTXFEMP`"]
pub type NPTXFEMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `GINNAKEFF`"]
pub type GINNAKEFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `GOUTNAKEFF`"]
pub type GOUTNAKEFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERLYSUSP`"]
pub type ERLYSUSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERLYSUSP`"]
pub struct ERLYSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> ERLYSUSP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `USBSUSP`"]
pub type USBSUSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBSUSP`"]
pub struct USBSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSUSP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `USBRST`"]
pub type USBRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBRST`"]
pub struct USBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `ENUMDONE`"]
pub type ENUMDONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENUMDONE`"]
pub struct ENUMDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUMDONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `ISOOUTDROP`"]
pub type ISOOUTDROP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISOOUTDROP`"]
pub struct ISOOUTDROP_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOOUTDROP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `EOPF`"]
pub type EOPF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOPF`"]
pub struct EOPF_W<'a> {
    w: &'a mut W,
}
impl<'a> EOPF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `IEPINT`"]
pub type IEPINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `OEPINT`"]
pub type OEPINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `INCOMPISOIN`"]
pub type INCOMPISOIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INCOMPISOIN`"]
pub struct INCOMPISOIN_W<'a> {
    w: &'a mut W,
}
impl<'a> INCOMPISOIN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `INCOMPLP`"]
pub type INCOMPLP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INCOMPLP`"]
pub struct INCOMPLP_W<'a> {
    w: &'a mut W,
}
impl<'a> INCOMPLP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `FETSUSP`"]
pub type FETSUSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FETSUSP`"]
pub struct FETSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> FETSUSP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `RESETDET`"]
pub type RESETDET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESETDET`"]
pub struct RESETDET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETDET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `PRTINT`"]
pub type PRTINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `HCHINT`"]
pub type HCHINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `PTXFEMP`"]
pub type PTXFEMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `CONIDSTSCHNG`"]
pub type CONIDSTSCHNG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONIDSTSCHNG`"]
pub struct CONIDSTSCHNG_W<'a> {
    w: &'a mut W,
}
impl<'a> CONIDSTSCHNG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `DISCONNINT`"]
pub type DISCONNINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISCONNINT`"]
pub struct DISCONNINT_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCONNINT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `SESSREQINT`"]
pub type SESSREQINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SESSREQINT`"]
pub struct SESSREQINT_W<'a> {
    w: &'a mut W,
}
impl<'a> SESSREQINT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `WKUPINT`"]
pub type WKUPINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKUPINT`"]
pub struct WKUPINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPINT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Current Mode of Operation"]
    #[inline(always)]
    pub fn curmod(&self) -> CURMOD_R {
        CURMOD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mode Mismatch Interrupt"]
    #[inline(always)]
    pub fn modemis(&self) -> MODEMIS_R {
        MODEMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OTG Interrupt"]
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start of (micro)Frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RxFIFO Non-Empty"]
    #[inline(always)]
    pub fn rxflvl(&self) -> RXFLVL_R {
        RXFLVL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Non-periodic TxFIFO Empty"]
    #[inline(always)]
    pub fn nptxfemp(&self) -> NPTXFEMP_R {
        NPTXFEMP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Global IN Non-periodic NAK Effective"]
    #[inline(always)]
    pub fn ginnakeff(&self) -> GINNAKEFF_R {
        GINNAKEFF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK Effective"]
    #[inline(always)]
    pub fn goutnakeff(&self) -> GOUTNAKEFF_R {
        GOUTNAKEFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Early Suspend"]
    #[inline(always)]
    pub fn erlysusp(&self) -> ERLYSUSP_R {
        ERLYSUSP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USB Suspend"]
    #[inline(always)]
    pub fn usbsusp(&self) -> USBSUSP_R {
        USBSUSP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - USB Reset"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enumeration Done"]
    #[inline(always)]
    pub fn enumdone(&self) -> ENUMDONE_R {
        ENUMDONE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt"]
    #[inline(always)]
    pub fn isooutdrop(&self) -> ISOOUTDROP_R {
        ISOOUTDROP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt"]
    #[inline(always)]
    pub fn eopf(&self) -> EOPF_R {
        EOPF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer"]
    #[inline(always)]
    pub fn incompisoin(&self) -> INCOMPISOIN_R {
        INCOMPISOIN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer"]
    #[inline(always)]
    pub fn incomplp(&self) -> INCOMPLP_R {
        INCOMPLP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Data Fetch Suspended"]
    #[inline(always)]
    pub fn fetsusp(&self) -> FETSUSP_R {
        FETSUSP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Reset Detected Interrupt"]
    #[inline(always)]
    pub fn resetdet(&self) -> RESETDET_R {
        RESETDET_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Host Port Interrupt"]
    #[inline(always)]
    pub fn prtint(&self) -> PRTINT_R {
        PRTINT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Host Channels Interrupt"]
    #[inline(always)]
    pub fn hchint(&self) -> HCHINT_R {
        HCHINT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO Empty"]
    #[inline(always)]
    pub fn ptxfemp(&self) -> PTXFEMP_R {
        PTXFEMP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Connector ID Status Change"]
    #[inline(always)]
    pub fn conidstschng(&self) -> CONIDSTSCHNG_R {
        CONIDSTSCHNG_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt"]
    #[inline(always)]
    pub fn disconnint(&self) -> DISCONNINT_R {
        DISCONNINT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt"]
    #[inline(always)]
    pub fn sessreqint(&self) -> SESSREQINT_R {
        SESSREQINT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt"]
    #[inline(always)]
    pub fn wkupint(&self) -> WKUPINT_R {
        WKUPINT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode Mismatch Interrupt"]
    #[inline(always)]
    pub fn modemis(&mut self) -> MODEMIS_W {
        MODEMIS_W { w: self }
    }
    #[doc = "Bit 3 - Start of (micro)Frame"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W {
        SOF_W { w: self }
    }
    #[doc = "Bit 10 - Early Suspend"]
    #[inline(always)]
    pub fn erlysusp(&mut self) -> ERLYSUSP_W {
        ERLYSUSP_W { w: self }
    }
    #[doc = "Bit 11 - USB Suspend"]
    #[inline(always)]
    pub fn usbsusp(&mut self) -> USBSUSP_W {
        USBSUSP_W { w: self }
    }
    #[doc = "Bit 12 - USB Reset"]
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W {
        USBRST_W { w: self }
    }
    #[doc = "Bit 13 - Enumeration Done"]
    #[inline(always)]
    pub fn enumdone(&mut self) -> ENUMDONE_W {
        ENUMDONE_W { w: self }
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt"]
    #[inline(always)]
    pub fn isooutdrop(&mut self) -> ISOOUTDROP_W {
        ISOOUTDROP_W { w: self }
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt"]
    #[inline(always)]
    pub fn eopf(&mut self) -> EOPF_W {
        EOPF_W { w: self }
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer"]
    #[inline(always)]
    pub fn incompisoin(&mut self) -> INCOMPISOIN_W {
        INCOMPISOIN_W { w: self }
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer"]
    #[inline(always)]
    pub fn incomplp(&mut self) -> INCOMPLP_W {
        INCOMPLP_W { w: self }
    }
    #[doc = "Bit 22 - Data Fetch Suspended"]
    #[inline(always)]
    pub fn fetsusp(&mut self) -> FETSUSP_W {
        FETSUSP_W { w: self }
    }
    #[doc = "Bit 23 - Reset Detected Interrupt"]
    #[inline(always)]
    pub fn resetdet(&mut self) -> RESETDET_W {
        RESETDET_W { w: self }
    }
    #[doc = "Bit 28 - Connector ID Status Change"]
    #[inline(always)]
    pub fn conidstschng(&mut self) -> CONIDSTSCHNG_W {
        CONIDSTSCHNG_W { w: self }
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt"]
    #[inline(always)]
    pub fn disconnint(&mut self) -> DISCONNINT_W {
        DISCONNINT_W { w: self }
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt"]
    #[inline(always)]
    pub fn sessreqint(&mut self) -> SESSREQINT_W {
        SESSREQINT_W { w: self }
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt"]
    #[inline(always)]
    pub fn wkupint(&mut self) -> WKUPINT_W {
        WKUPINT_W { w: self }
    }
}
