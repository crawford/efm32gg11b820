#[doc = "Reader of register GINTMSK"]
pub type R = crate::R<u32, super::GINTMSK>;
#[doc = "Writer for register GINTMSK"]
pub type W = crate::W<u32, super::GINTMSK>;
#[doc = "Register GINTMSK `reset()`'s with value 0"]
impl crate::ResetValue for super::GINTMSK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MODEMISMSK`"]
pub type MODEMISMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODEMISMSK`"]
pub struct MODEMISMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> MODEMISMSK_W<'a> {
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
#[doc = "Reader of field `OTGINTMSK`"]
pub type OTGINTMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTGINTMSK`"]
pub struct OTGINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGINTMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SOFMSK`"]
pub type SOFMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFMSK`"]
pub struct SOFMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFMSK_W<'a> {
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
#[doc = "Reader of field `RXFLVLMSK`"]
pub type RXFLVLMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFLVLMSK`"]
pub struct RXFLVLMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFLVLMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `NPTXFEMPMSK`"]
pub type NPTXFEMPMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NPTXFEMPMSK`"]
pub struct NPTXFEMPMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> NPTXFEMPMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `GINNAKEFFMSK`"]
pub type GINNAKEFFMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GINNAKEFFMSK`"]
pub struct GINNAKEFFMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> GINNAKEFFMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `GOUTNAKEFFMSK`"]
pub type GOUTNAKEFFMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GOUTNAKEFFMSK`"]
pub struct GOUTNAKEFFMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> GOUTNAKEFFMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `ERLYSUSPMSK`"]
pub type ERLYSUSPMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERLYSUSPMSK`"]
pub struct ERLYSUSPMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> ERLYSUSPMSK_W<'a> {
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
#[doc = "Reader of field `USBSUSPMSK`"]
pub type USBSUSPMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBSUSPMSK`"]
pub struct USBSUSPMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSUSPMSK_W<'a> {
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
#[doc = "Reader of field `USBRSTMSK`"]
pub type USBRSTMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBRSTMSK`"]
pub struct USBRSTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRSTMSK_W<'a> {
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
#[doc = "Reader of field `ENUMDONEMSK`"]
pub type ENUMDONEMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENUMDONEMSK`"]
pub struct ENUMDONEMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUMDONEMSK_W<'a> {
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
#[doc = "Reader of field `ISOOUTDROPMSK`"]
pub type ISOOUTDROPMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISOOUTDROPMSK`"]
pub struct ISOOUTDROPMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOOUTDROPMSK_W<'a> {
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
#[doc = "Reader of field `EOPFMSK`"]
pub type EOPFMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOPFMSK`"]
pub struct EOPFMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> EOPFMSK_W<'a> {
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
#[doc = "Reader of field `EPMISMSK`"]
pub type EPMISMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPMISMSK`"]
pub struct EPMISMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> EPMISMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `IEPINTMSK`"]
pub type IEPINTMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEPINTMSK`"]
pub struct IEPINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> IEPINTMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `OEPINTMSK`"]
pub type OEPINTMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OEPINTMSK`"]
pub struct OEPINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> OEPINTMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `INCOMPISOINMSK`"]
pub type INCOMPISOINMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INCOMPISOINMSK`"]
pub struct INCOMPISOINMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> INCOMPISOINMSK_W<'a> {
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
#[doc = "Reader of field `INCOMPLPMSK`"]
pub type INCOMPLPMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INCOMPLPMSK`"]
pub struct INCOMPLPMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> INCOMPLPMSK_W<'a> {
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
#[doc = "Reader of field `FETSUSPMSK`"]
pub type FETSUSPMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FETSUSPMSK`"]
pub struct FETSUSPMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> FETSUSPMSK_W<'a> {
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
#[doc = "Reader of field `RESETDETMSK`"]
pub type RESETDETMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESETDETMSK`"]
pub struct RESETDETMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETDETMSK_W<'a> {
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
#[doc = "Reader of field `PRTINTMSK`"]
pub type PRTINTMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRTINTMSK`"]
pub struct PRTINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTINTMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `HCHINTMSK`"]
pub type HCHINTMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HCHINTMSK`"]
pub struct HCHINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> HCHINTMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `PTXFEMPMSK`"]
pub type PTXFEMPMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTXFEMPMSK`"]
pub struct PTXFEMPMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFEMPMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `CONIDSTSCHNGMSK`"]
pub type CONIDSTSCHNGMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONIDSTSCHNGMSK`"]
pub struct CONIDSTSCHNGMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> CONIDSTSCHNGMSK_W<'a> {
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
#[doc = "Reader of field `DISCONNINTMSK`"]
pub type DISCONNINTMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISCONNINTMSK`"]
pub struct DISCONNINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCONNINTMSK_W<'a> {
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
#[doc = "Reader of field `SESSREQINTMSK`"]
pub type SESSREQINTMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SESSREQINTMSK`"]
pub struct SESSREQINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> SESSREQINTMSK_W<'a> {
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
#[doc = "Reader of field `WKUPINTMSK`"]
pub type WKUPINTMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKUPINTMSK`"]
pub struct WKUPINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPINTMSK_W<'a> {
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
    #[doc = "Bit 1 - Mode Mismatch Interrupt Mask"]
    #[inline(always)]
    pub fn modemismsk(&self) -> MODEMISMSK_R {
        MODEMISMSK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OTG Interrupt Mask"]
    #[inline(always)]
    pub fn otgintmsk(&self) -> OTGINTMSK_R {
        OTGINTMSK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start of (micro)Frame Mask"]
    #[inline(always)]
    pub fn sofmsk(&self) -> SOFMSK_R {
        SOFMSK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask"]
    #[inline(always)]
    pub fn rxflvlmsk(&self) -> RXFLVLMSK_R {
        RXFLVLMSK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Non-periodic TxFIFO Empty Mask"]
    #[inline(always)]
    pub fn nptxfempmsk(&self) -> NPTXFEMPMSK_R {
        NPTXFEMPMSK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Global Non-periodic IN NAK Effective Mask"]
    #[inline(always)]
    pub fn ginnakeffmsk(&self) -> GINNAKEFFMSK_R {
        GINNAKEFFMSK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK Effective Mask"]
    #[inline(always)]
    pub fn goutnakeffmsk(&self) -> GOUTNAKEFFMSK_R {
        GOUTNAKEFFMSK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Early Suspend Mask"]
    #[inline(always)]
    pub fn erlysuspmsk(&self) -> ERLYSUSPMSK_R {
        ERLYSUSPMSK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USB Suspend Mask"]
    #[inline(always)]
    pub fn usbsuspmsk(&self) -> USBSUSPMSK_R {
        USBSUSPMSK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - USB Reset Mask"]
    #[inline(always)]
    pub fn usbrstmsk(&self) -> USBRSTMSK_R {
        USBRSTMSK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enumeration Done Mask"]
    #[inline(always)]
    pub fn enumdonemsk(&self) -> ENUMDONEMSK_R {
        ENUMDONEMSK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt Mask"]
    #[inline(always)]
    pub fn isooutdropmsk(&self) -> ISOOUTDROPMSK_R {
        ISOOUTDROPMSK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt Mask"]
    #[inline(always)]
    pub fn eopfmsk(&self) -> EOPFMSK_R {
        EOPFMSK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Endpoint Mismatch Interrupt Mask"]
    #[inline(always)]
    pub fn epmismsk(&self) -> EPMISMSK_R {
        EPMISMSK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt Mask"]
    #[inline(always)]
    pub fn iepintmsk(&self) -> IEPINTMSK_R {
        IEPINTMSK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt Mask"]
    #[inline(always)]
    pub fn oepintmsk(&self) -> OEPINTMSK_R {
        OEPINTMSK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer Mask"]
    #[inline(always)]
    pub fn incompisoinmsk(&self) -> INCOMPISOINMSK_R {
        INCOMPISOINMSK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer Mask"]
    #[inline(always)]
    pub fn incomplpmsk(&self) -> INCOMPLPMSK_R {
        INCOMPLPMSK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Data Fetch Suspended Mask"]
    #[inline(always)]
    pub fn fetsuspmsk(&self) -> FETSUSPMSK_R {
        FETSUSPMSK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Reset Detected Interrupt Mask"]
    #[inline(always)]
    pub fn resetdetmsk(&self) -> RESETDETMSK_R {
        RESETDETMSK_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Host Port Interrupt Mask"]
    #[inline(always)]
    pub fn prtintmsk(&self) -> PRTINTMSK_R {
        PRTINTMSK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Host Channels Interrupt Mask"]
    #[inline(always)]
    pub fn hchintmsk(&self) -> HCHINTMSK_R {
        HCHINTMSK_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO Empty Mask"]
    #[inline(always)]
    pub fn ptxfempmsk(&self) -> PTXFEMPMSK_R {
        PTXFEMPMSK_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Connector ID Status Change Mask"]
    #[inline(always)]
    pub fn conidstschngmsk(&self) -> CONIDSTSCHNGMSK_R {
        CONIDSTSCHNGMSK_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt Mask"]
    #[inline(always)]
    pub fn disconnintmsk(&self) -> DISCONNINTMSK_R {
        DISCONNINTMSK_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt Mask"]
    #[inline(always)]
    pub fn sessreqintmsk(&self) -> SESSREQINTMSK_R {
        SESSREQINTMSK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask the WakeUp Bit is Used for LPM State Wake Up in a Way Similar to That of Wake Up in Suspend State"]
    #[inline(always)]
    pub fn wkupintmsk(&self) -> WKUPINTMSK_R {
        WKUPINTMSK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode Mismatch Interrupt Mask"]
    #[inline(always)]
    pub fn modemismsk(&mut self) -> MODEMISMSK_W {
        MODEMISMSK_W { w: self }
    }
    #[doc = "Bit 2 - OTG Interrupt Mask"]
    #[inline(always)]
    pub fn otgintmsk(&mut self) -> OTGINTMSK_W {
        OTGINTMSK_W { w: self }
    }
    #[doc = "Bit 3 - Start of (micro)Frame Mask"]
    #[inline(always)]
    pub fn sofmsk(&mut self) -> SOFMSK_W {
        SOFMSK_W { w: self }
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask"]
    #[inline(always)]
    pub fn rxflvlmsk(&mut self) -> RXFLVLMSK_W {
        RXFLVLMSK_W { w: self }
    }
    #[doc = "Bit 5 - Non-periodic TxFIFO Empty Mask"]
    #[inline(always)]
    pub fn nptxfempmsk(&mut self) -> NPTXFEMPMSK_W {
        NPTXFEMPMSK_W { w: self }
    }
    #[doc = "Bit 6 - Global Non-periodic IN NAK Effective Mask"]
    #[inline(always)]
    pub fn ginnakeffmsk(&mut self) -> GINNAKEFFMSK_W {
        GINNAKEFFMSK_W { w: self }
    }
    #[doc = "Bit 7 - Global OUT NAK Effective Mask"]
    #[inline(always)]
    pub fn goutnakeffmsk(&mut self) -> GOUTNAKEFFMSK_W {
        GOUTNAKEFFMSK_W { w: self }
    }
    #[doc = "Bit 10 - Early Suspend Mask"]
    #[inline(always)]
    pub fn erlysuspmsk(&mut self) -> ERLYSUSPMSK_W {
        ERLYSUSPMSK_W { w: self }
    }
    #[doc = "Bit 11 - USB Suspend Mask"]
    #[inline(always)]
    pub fn usbsuspmsk(&mut self) -> USBSUSPMSK_W {
        USBSUSPMSK_W { w: self }
    }
    #[doc = "Bit 12 - USB Reset Mask"]
    #[inline(always)]
    pub fn usbrstmsk(&mut self) -> USBRSTMSK_W {
        USBRSTMSK_W { w: self }
    }
    #[doc = "Bit 13 - Enumeration Done Mask"]
    #[inline(always)]
    pub fn enumdonemsk(&mut self) -> ENUMDONEMSK_W {
        ENUMDONEMSK_W { w: self }
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt Mask"]
    #[inline(always)]
    pub fn isooutdropmsk(&mut self) -> ISOOUTDROPMSK_W {
        ISOOUTDROPMSK_W { w: self }
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt Mask"]
    #[inline(always)]
    pub fn eopfmsk(&mut self) -> EOPFMSK_W {
        EOPFMSK_W { w: self }
    }
    #[doc = "Bit 17 - Endpoint Mismatch Interrupt Mask"]
    #[inline(always)]
    pub fn epmismsk(&mut self) -> EPMISMSK_W {
        EPMISMSK_W { w: self }
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt Mask"]
    #[inline(always)]
    pub fn iepintmsk(&mut self) -> IEPINTMSK_W {
        IEPINTMSK_W { w: self }
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt Mask"]
    #[inline(always)]
    pub fn oepintmsk(&mut self) -> OEPINTMSK_W {
        OEPINTMSK_W { w: self }
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer Mask"]
    #[inline(always)]
    pub fn incompisoinmsk(&mut self) -> INCOMPISOINMSK_W {
        INCOMPISOINMSK_W { w: self }
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer Mask"]
    #[inline(always)]
    pub fn incomplpmsk(&mut self) -> INCOMPLPMSK_W {
        INCOMPLPMSK_W { w: self }
    }
    #[doc = "Bit 22 - Data Fetch Suspended Mask"]
    #[inline(always)]
    pub fn fetsuspmsk(&mut self) -> FETSUSPMSK_W {
        FETSUSPMSK_W { w: self }
    }
    #[doc = "Bit 23 - Reset Detected Interrupt Mask"]
    #[inline(always)]
    pub fn resetdetmsk(&mut self) -> RESETDETMSK_W {
        RESETDETMSK_W { w: self }
    }
    #[doc = "Bit 24 - Host Port Interrupt Mask"]
    #[inline(always)]
    pub fn prtintmsk(&mut self) -> PRTINTMSK_W {
        PRTINTMSK_W { w: self }
    }
    #[doc = "Bit 25 - Host Channels Interrupt Mask"]
    #[inline(always)]
    pub fn hchintmsk(&mut self) -> HCHINTMSK_W {
        HCHINTMSK_W { w: self }
    }
    #[doc = "Bit 26 - Periodic TxFIFO Empty Mask"]
    #[inline(always)]
    pub fn ptxfempmsk(&mut self) -> PTXFEMPMSK_W {
        PTXFEMPMSK_W { w: self }
    }
    #[doc = "Bit 28 - Connector ID Status Change Mask"]
    #[inline(always)]
    pub fn conidstschngmsk(&mut self) -> CONIDSTSCHNGMSK_W {
        CONIDSTSCHNGMSK_W { w: self }
    }
    #[doc = "Bit 29 - Disconnect Detected Interrupt Mask"]
    #[inline(always)]
    pub fn disconnintmsk(&mut self) -> DISCONNINTMSK_W {
        DISCONNINTMSK_W { w: self }
    }
    #[doc = "Bit 30 - Session Request/New Session Detected Interrupt Mask"]
    #[inline(always)]
    pub fn sessreqintmsk(&mut self) -> SESSREQINTMSK_W {
        SESSREQINTMSK_W { w: self }
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask the WakeUp Bit is Used for LPM State Wake Up in a Way Similar to That of Wake Up in Suspend State"]
    #[inline(always)]
    pub fn wkupintmsk(&mut self) -> WKUPINTMSK_W {
        WKUPINTMSK_W { w: self }
    }
}
