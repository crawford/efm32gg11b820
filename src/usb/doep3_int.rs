#[doc = "Reader of register DOEP3_INT"]
pub type R = crate::R<u32, super::DOEP3_INT>;
#[doc = "Writer for register DOEP3_INT"]
pub type W = crate::W<u32, super::DOEP3_INT>;
#[doc = "Register DOEP3_INT `reset()`'s with value 0"]
impl crate::ResetValue for super::DOEP3_INT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XFERCOMPL`"]
pub type XFERCOMPL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XFERCOMPL`"]
pub struct XFERCOMPL_W<'a> {
    w: &'a mut W,
}
impl<'a> XFERCOMPL_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `EPDISBLD`"]
pub type EPDISBLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPDISBLD`"]
pub struct EPDISBLD_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDISBLD_W<'a> {
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
#[doc = "Reader of field `AHBERR`"]
pub type AHBERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHBERR`"]
pub struct AHBERR_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBERR_W<'a> {
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
#[doc = "Reader of field `SETUP`"]
pub type SETUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETUP`"]
pub struct SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUP_W<'a> {
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
#[doc = "Reader of field `OUTTKNEPDIS`"]
pub type OUTTKNEPDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTTKNEPDIS`"]
pub struct OUTTKNEPDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTKNEPDIS_W<'a> {
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
#[doc = "Reader of field `STSPHSERCVD`"]
pub type STSPHSERCVD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STSPHSERCVD`"]
pub struct STSPHSERCVD_W<'a> {
    w: &'a mut W,
}
impl<'a> STSPHSERCVD_W<'a> {
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
#[doc = "Reader of field `BACK2BACKSETUP`"]
pub type BACK2BACKSETUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BACK2BACKSETUP`"]
pub struct BACK2BACKSETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> BACK2BACKSETUP_W<'a> {
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
#[doc = "Reader of field `OUTPKTERR`"]
pub type OUTPKTERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTPKTERR`"]
pub struct OUTPKTERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPKTERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `PKTDRPSTS`"]
pub type PKTDRPSTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PKTDRPSTS`"]
pub struct PKTDRPSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> PKTDRPSTS_W<'a> {
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
#[doc = "Reader of field `BBLEERR`"]
pub type BBLEERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BBLEERR`"]
pub struct BBLEERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BBLEERR_W<'a> {
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
#[doc = "Reader of field `NAKINTRPT`"]
pub type NAKINTRPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NAKINTRPT`"]
pub struct NAKINTRPT_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKINTRPT_W<'a> {
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
#[doc = "Reader of field `STUPPKTRCVD`"]
pub type STUPPKTRCVD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STUPPKTRCVD`"]
pub struct STUPPKTRCVD_W<'a> {
    w: &'a mut W,
}
impl<'a> STUPPKTRCVD_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Transfer Completed Interrupt"]
    #[inline(always)]
    pub fn xfercompl(&self) -> XFERCOMPL_R {
        XFERCOMPL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    pub fn epdisbld(&self) -> EPDISBLD_R {
        EPDISBLD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Phase Done"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OUT Token Received When Endpoint Disabled"]
    #[inline(always)]
    pub fn outtknepdis(&self) -> OUTTKNEPDIS_R {
        OUTTKNEPDIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Status Phase Received for Control Write"]
    #[inline(always)]
    pub fn stsphsercvd(&self) -> STSPHSERCVD_R {
        STSPHSERCVD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received"]
    #[inline(always)]
    pub fn back2backsetup(&self) -> BACK2BACKSETUP_R {
        BACK2BACKSETUP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - OUT Packet Error"]
    #[inline(always)]
    pub fn outpkterr(&self) -> OUTPKTERR_R {
        OUTPKTERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Packet Drop Status"]
    #[inline(always)]
    pub fn pktdrpsts(&self) -> PKTDRPSTS_R {
        PKTDRPSTS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - NAK Interrupt"]
    #[inline(always)]
    pub fn bbleerr(&self) -> BBLEERR_R {
        BBLEERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn nakintrpt(&self) -> NAKINTRPT_R {
        NAKINTRPT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn stuppktrcvd(&self) -> STUPPKTRCVD_R {
        STUPPKTRCVD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt"]
    #[inline(always)]
    pub fn xfercompl(&mut self) -> XFERCOMPL_W {
        XFERCOMPL_W { w: self }
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt"]
    #[inline(always)]
    pub fn epdisbld(&mut self) -> EPDISBLD_W {
        EPDISBLD_W { w: self }
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&mut self) -> AHBERR_W {
        AHBERR_W { w: self }
    }
    #[doc = "Bit 3 - Phase Done"]
    #[inline(always)]
    pub fn setup(&mut self) -> SETUP_W {
        SETUP_W { w: self }
    }
    #[doc = "Bit 4 - OUT Token Received When Endpoint Disabled"]
    #[inline(always)]
    pub fn outtknepdis(&mut self) -> OUTTKNEPDIS_W {
        OUTTKNEPDIS_W { w: self }
    }
    #[doc = "Bit 5 - Status Phase Received for Control Write"]
    #[inline(always)]
    pub fn stsphsercvd(&mut self) -> STSPHSERCVD_W {
        STSPHSERCVD_W { w: self }
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received"]
    #[inline(always)]
    pub fn back2backsetup(&mut self) -> BACK2BACKSETUP_W {
        BACK2BACKSETUP_W { w: self }
    }
    #[doc = "Bit 8 - OUT Packet Error"]
    #[inline(always)]
    pub fn outpkterr(&mut self) -> OUTPKTERR_W {
        OUTPKTERR_W { w: self }
    }
    #[doc = "Bit 11 - Packet Drop Status"]
    #[inline(always)]
    pub fn pktdrpsts(&mut self) -> PKTDRPSTS_W {
        PKTDRPSTS_W { w: self }
    }
    #[doc = "Bit 12 - NAK Interrupt"]
    #[inline(always)]
    pub fn bbleerr(&mut self) -> BBLEERR_W {
        BBLEERR_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn nakintrpt(&mut self) -> NAKINTRPT_W {
        NAKINTRPT_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn stuppktrcvd(&mut self) -> STUPPKTRCVD_W {
        STUPPKTRCVD_W { w: self }
    }
}
