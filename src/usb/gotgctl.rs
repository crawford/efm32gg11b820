#[doc = "Reader of register GOTGCTL"]
pub type R = crate::R<u32, super::GOTGCTL>;
#[doc = "Writer for register GOTGCTL"]
pub type W = crate::W<u32, super::GOTGCTL>;
#[doc = "Register GOTGCTL `reset()`'s with value 0x0001_0000"]
impl crate::ResetValue for super::GOTGCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0000
    }
}
#[doc = "Reader of field `SESREQSCS`"]
pub type SESREQSCS_R = crate::R<bool, bool>;
#[doc = "Reader of field `SESREQ`"]
pub type SESREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SESREQ`"]
pub struct SESREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SESREQ_W<'a> {
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
#[doc = "Reader of field `VBVALIDOVEN`"]
pub type VBVALIDOVEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBVALIDOVEN`"]
pub struct VBVALIDOVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBVALIDOVEN_W<'a> {
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
#[doc = "Reader of field `VBVALIDOVVAL`"]
pub type VBVALIDOVVAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBVALIDOVVAL`"]
pub struct VBVALIDOVVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> VBVALIDOVVAL_W<'a> {
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
#[doc = "Reader of field `AVALIDOVEN`"]
pub type AVALIDOVEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AVALIDOVEN`"]
pub struct AVALIDOVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AVALIDOVEN_W<'a> {
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
#[doc = "Reader of field `AVALIDOVVAL`"]
pub type AVALIDOVVAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AVALIDOVVAL`"]
pub struct AVALIDOVVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> AVALIDOVVAL_W<'a> {
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
#[doc = "Reader of field `BVALIDOVEN`"]
pub type BVALIDOVEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BVALIDOVEN`"]
pub struct BVALIDOVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BVALIDOVEN_W<'a> {
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
#[doc = "Reader of field `BVALIDOVVAL`"]
pub type BVALIDOVVAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BVALIDOVVAL`"]
pub struct BVALIDOVVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BVALIDOVVAL_W<'a> {
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
#[doc = "Reader of field `HSTNEGSCS`"]
pub type HSTNEGSCS_R = crate::R<bool, bool>;
#[doc = "Reader of field `HNPREQ`"]
pub type HNPREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HNPREQ`"]
pub struct HNPREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HNPREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `HSTSETHNPEN`"]
pub type HSTSETHNPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSTSETHNPEN`"]
pub struct HSTSETHNPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTSETHNPEN_W<'a> {
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
#[doc = "Reader of field `DEVHNPEN`"]
pub type DEVHNPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEVHNPEN`"]
pub struct DEVHNPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVHNPEN_W<'a> {
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
#[doc = "Reader of field `EHEN`"]
pub type EHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EHEN`"]
pub struct EHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EHEN_W<'a> {
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
#[doc = "Reader of field `DBNCEFLTRBYPASS`"]
pub type DBNCEFLTRBYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBNCEFLTRBYPASS`"]
pub struct DBNCEFLTRBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> DBNCEFLTRBYPASS_W<'a> {
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
#[doc = "Reader of field `CONIDSTS`"]
pub type CONIDSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DBNCTIME`"]
pub type DBNCTIME_R = crate::R<bool, bool>;
#[doc = "Reader of field `ASESVLD`"]
pub type ASESVLD_R = crate::R<bool, bool>;
#[doc = "Reader of field `BSESVLD`"]
pub type BSESVLD_R = crate::R<bool, bool>;
#[doc = "Reader of field `OTGVER`"]
pub type OTGVER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTGVER`"]
pub struct OTGVER_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGVER_W<'a> {
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
#[doc = "Reader of field `CURMOD`"]
pub type CURMOD_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Session Request Success"]
    #[inline(always)]
    pub fn sesreqscs(&self) -> SESREQSCS_R {
        SESREQSCS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Session Request"]
    #[inline(always)]
    pub fn sesreq(&self) -> SESREQ_R {
        SESREQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - VBUS Valid Override Enable"]
    #[inline(always)]
    pub fn vbvalidoven(&self) -> VBVALIDOVEN_R {
        VBVALIDOVEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VBUS Valid OverrideValue"]
    #[inline(always)]
    pub fn vbvalidovval(&self) -> VBVALIDOVVAL_R {
        VBVALIDOVVAL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - A-Peripheral Session Valid Override Enable"]
    #[inline(always)]
    pub fn avalidoven(&self) -> AVALIDOVEN_R {
        AVALIDOVEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - A-Peripheral Session Valid OverrideValue"]
    #[inline(always)]
    pub fn avalidovval(&self) -> AVALIDOVVAL_R {
        AVALIDOVVAL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B-Peripheral Session Valid Override Enable"]
    #[inline(always)]
    pub fn bvalidoven(&self) -> BVALIDOVEN_R {
        BVALIDOVEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B-Peripheral Session Valid OverrideValue"]
    #[inline(always)]
    pub fn bvalidovval(&self) -> BVALIDOVVAL_R {
        BVALIDOVVAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Host Negotiation Success"]
    #[inline(always)]
    pub fn hstnegscs(&self) -> HSTNEGSCS_R {
        HSTNEGSCS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - HNP Request"]
    #[inline(always)]
    pub fn hnpreq(&self) -> HNPREQ_R {
        HNPREQ_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Host Set HNP Enable"]
    #[inline(always)]
    pub fn hstsethnpen(&self) -> HSTSETHNPEN_R {
        HSTSETHNPEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Device HNP Enabled"]
    #[inline(always)]
    pub fn devhnpen(&self) -> DEVHNPEN_R {
        DEVHNPEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Embedded Host Enable"]
    #[inline(always)]
    pub fn ehen(&self) -> EHEN_R {
        EHEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dbncefltrbypass(&self) -> DBNCEFLTRBYPASS_R {
        DBNCEFLTRBYPASS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Connector ID Status"]
    #[inline(always)]
    pub fn conidsts(&self) -> CONIDSTS_R {
        CONIDSTS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Long/Short Debounce Time"]
    #[inline(always)]
    pub fn dbnctime(&self) -> DBNCTIME_R {
        DBNCTIME_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - A-Session Valid"]
    #[inline(always)]
    pub fn asesvld(&self) -> ASESVLD_R {
        ASESVLD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B-Session Valid"]
    #[inline(always)]
    pub fn bsesvld(&self) -> BSESVLD_R {
        BSESVLD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - OTG Version"]
    #[inline(always)]
    pub fn otgver(&self) -> OTGVER_R {
        OTGVER_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Current Mode of Operation"]
    #[inline(always)]
    pub fn curmod(&self) -> CURMOD_R {
        CURMOD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Session Request"]
    #[inline(always)]
    pub fn sesreq(&mut self) -> SESREQ_W {
        SESREQ_W { w: self }
    }
    #[doc = "Bit 2 - VBUS Valid Override Enable"]
    #[inline(always)]
    pub fn vbvalidoven(&mut self) -> VBVALIDOVEN_W {
        VBVALIDOVEN_W { w: self }
    }
    #[doc = "Bit 3 - VBUS Valid OverrideValue"]
    #[inline(always)]
    pub fn vbvalidovval(&mut self) -> VBVALIDOVVAL_W {
        VBVALIDOVVAL_W { w: self }
    }
    #[doc = "Bit 4 - A-Peripheral Session Valid Override Enable"]
    #[inline(always)]
    pub fn avalidoven(&mut self) -> AVALIDOVEN_W {
        AVALIDOVEN_W { w: self }
    }
    #[doc = "Bit 5 - A-Peripheral Session Valid OverrideValue"]
    #[inline(always)]
    pub fn avalidovval(&mut self) -> AVALIDOVVAL_W {
        AVALIDOVVAL_W { w: self }
    }
    #[doc = "Bit 6 - B-Peripheral Session Valid Override Enable"]
    #[inline(always)]
    pub fn bvalidoven(&mut self) -> BVALIDOVEN_W {
        BVALIDOVEN_W { w: self }
    }
    #[doc = "Bit 7 - B-Peripheral Session Valid OverrideValue"]
    #[inline(always)]
    pub fn bvalidovval(&mut self) -> BVALIDOVVAL_W {
        BVALIDOVVAL_W { w: self }
    }
    #[doc = "Bit 9 - HNP Request"]
    #[inline(always)]
    pub fn hnpreq(&mut self) -> HNPREQ_W {
        HNPREQ_W { w: self }
    }
    #[doc = "Bit 10 - Host Set HNP Enable"]
    #[inline(always)]
    pub fn hstsethnpen(&mut self) -> HSTSETHNPEN_W {
        HSTSETHNPEN_W { w: self }
    }
    #[doc = "Bit 11 - Device HNP Enabled"]
    #[inline(always)]
    pub fn devhnpen(&mut self) -> DEVHNPEN_W {
        DEVHNPEN_W { w: self }
    }
    #[doc = "Bit 12 - Embedded Host Enable"]
    #[inline(always)]
    pub fn ehen(&mut self) -> EHEN_W {
        EHEN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dbncefltrbypass(&mut self) -> DBNCEFLTRBYPASS_W {
        DBNCEFLTRBYPASS_W { w: self }
    }
    #[doc = "Bit 20 - OTG Version"]
    #[inline(always)]
    pub fn otgver(&mut self) -> OTGVER_W {
        OTGVER_W { w: self }
    }
}
