#[doc = "Reader of register IFCR"]
pub type R = crate::R<u32, super::IFCR>;
#[doc = "Writer for register IFCR"]
pub type W = crate::W<u32, super::IFCR>;
#[doc = "Register IFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::IFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMDCOM`"]
pub type CMDCOM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDCOM`"]
pub struct CMDCOM_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDCOM_W<'a> {
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
#[doc = "Reader of field `TRANCOM`"]
pub type TRANCOM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRANCOM`"]
pub struct TRANCOM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANCOM_W<'a> {
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
#[doc = "Reader of field `BLKGAPEVT`"]
pub type BLKGAPEVT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLKGAPEVT`"]
pub struct BLKGAPEVT_W<'a> {
    w: &'a mut W,
}
impl<'a> BLKGAPEVT_W<'a> {
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
#[doc = "Reader of field `DMAINT`"]
pub type DMAINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAINT`"]
pub struct DMAINT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAINT_W<'a> {
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
#[doc = "Reader of field `BFRWRRDY`"]
pub type BFRWRRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BFRWRRDY`"]
pub struct BFRWRRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BFRWRRDY_W<'a> {
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
#[doc = "Reader of field `BFRRDRDY`"]
pub type BFRRDRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BFRRDRDY`"]
pub struct BFRRDRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BFRRDRDY_W<'a> {
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
#[doc = "Reader of field `CARDINS`"]
pub type CARDINS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARDINS`"]
pub struct CARDINS_W<'a> {
    w: &'a mut W,
}
impl<'a> CARDINS_W<'a> {
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
#[doc = "Reader of field `CARDRM`"]
pub type CARDRM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARDRM`"]
pub struct CARDRM_W<'a> {
    w: &'a mut W,
}
impl<'a> CARDRM_W<'a> {
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
#[doc = "Reader of field `CARDINT`"]
pub type CARDINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RETUNINGEVT`"]
pub type RETUNINGEVT_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOOTACKRCV`"]
pub type BOOTACKRCV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOTACKRCV`"]
pub struct BOOTACKRCV_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTACKRCV_W<'a> {
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
#[doc = "Reader of field `BOOTTERMINATE`"]
pub type BOOTTERMINATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOTTERMINATE`"]
pub struct BOOTTERMINATE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTTERMINATE_W<'a> {
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
#[doc = "Reader of field `ERRINT`"]
pub type ERRINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMDTOUTERR`"]
pub type CMDTOUTERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDTOUTERR`"]
pub struct CMDTOUTERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDTOUTERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `CMDCRCERR`"]
pub type CMDCRCERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDCRCERR`"]
pub struct CMDCRCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDCRCERR_W<'a> {
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
#[doc = "Reader of field `CMDENDBITERR`"]
pub type CMDENDBITERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDENDBITERR`"]
pub struct CMDENDBITERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDENDBITERR_W<'a> {
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
#[doc = "Reader of field `CMDINDEXERR`"]
pub type CMDINDEXERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDINDEXERR`"]
pub struct CMDINDEXERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDINDEXERR_W<'a> {
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
#[doc = "Reader of field `DATTOUTERR`"]
pub type DATTOUTERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATTOUTERR`"]
pub struct DATTOUTERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATTOUTERR_W<'a> {
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
#[doc = "Reader of field `DATCRCERR`"]
pub type DATCRCERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATCRCERR`"]
pub struct DATCRCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATCRCERR_W<'a> {
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
#[doc = "Reader of field `DATENDBITERR`"]
pub type DATENDBITERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATENDBITERR`"]
pub struct DATENDBITERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATENDBITERR_W<'a> {
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
#[doc = "Reader of field `CURRENTLIMITERR`"]
pub type CURRENTLIMITERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CURRENTLIMITERR`"]
pub struct CURRENTLIMITERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CURRENTLIMITERR_W<'a> {
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
#[doc = "Reader of field `AUTOCMDERR`"]
pub type AUTOCMDERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOCMDERR`"]
pub struct AUTOCMDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOCMDERR_W<'a> {
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
#[doc = "Reader of field `ADMAERR`"]
pub type ADMAERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADMAERR`"]
pub struct ADMAERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADMAERR_W<'a> {
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
#[doc = "Reader of field `TARGETRESP`"]
pub type TARGETRESP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TARGETRESP`"]
pub struct TARGETRESP_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGETRESP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Command Complete"]
    #[inline(always)]
    pub fn cmdcom(&self) -> CMDCOM_R {
        CMDCOM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    pub fn trancom(&self) -> TRANCOM_R {
        TRANCOM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    pub fn blkgapevt(&self) -> BLKGAPEVT_R {
        BLKGAPEVT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline(always)]
    pub fn dmaint(&self) -> DMAINT_R {
        DMAINT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    pub fn bfrwrrdy(&self) -> BFRWRRDY_R {
        BFRWRRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    pub fn bfrrdrdy(&self) -> BFRRDRDY_R {
        BFRRDRDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline(always)]
    pub fn cardins(&self) -> CARDINS_R {
        CARDINS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline(always)]
    pub fn cardrm(&self) -> CARDRM_R {
        CARDRM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt"]
    #[inline(always)]
    pub fn cardint(&self) -> CARDINT_R {
        CARDINT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Re-Tunning Event"]
    #[inline(always)]
    pub fn retuningevt(&self) -> RETUNINGEVT_R {
        RETUNINGEVT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Boot Ack Received"]
    #[inline(always)]
    pub fn bootackrcv(&self) -> BOOTACKRCV_R {
        BOOTACKRCV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Boot Terminate Interrupt"]
    #[inline(always)]
    pub fn bootterminate(&self) -> BOOTTERMINATE_R {
        BOOTTERMINATE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Error Interrupt"]
    #[inline(always)]
    pub fn errint(&self) -> ERRINT_R {
        ERRINT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Command Timeout Error"]
    #[inline(always)]
    pub fn cmdtouterr(&self) -> CMDTOUTERR_R {
        CMDTOUTERR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CMD CRC Error"]
    #[inline(always)]
    pub fn cmdcrcerr(&self) -> CMDCRCERR_R {
        CMDCRCERR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Command End Bit Error"]
    #[inline(always)]
    pub fn cmdendbiterr(&self) -> CMDENDBITERR_R {
        CMDENDBITERR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Command Index Error"]
    #[inline(always)]
    pub fn cmdindexerr(&self) -> CMDINDEXERR_R {
        CMDINDEXERR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Data Time-out Error"]
    #[inline(always)]
    pub fn dattouterr(&self) -> DATTOUTERR_R {
        DATTOUTERR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error"]
    #[inline(always)]
    pub fn datcrcerr(&self) -> DATCRCERR_R {
        DATCRCERR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Data End Bit Error"]
    #[inline(always)]
    pub fn datendbiterr(&self) -> DATENDBITERR_R {
        DATENDBITERR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Current Limit Error"]
    #[inline(always)]
    pub fn currentlimiterr(&self) -> CURRENTLIMITERR_R {
        CURRENTLIMITERR_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Auto CMD Error"]
    #[inline(always)]
    pub fn autocmderr(&self) -> AUTOCMDERR_R {
        AUTOCMDERR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - ADMA Error"]
    #[inline(always)]
    pub fn admaerr(&self) -> ADMAERR_R {
        ADMAERR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Specific Error STAT"]
    #[inline(always)]
    pub fn targetresp(&self) -> TARGETRESP_R {
        TARGETRESP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete"]
    #[inline(always)]
    pub fn cmdcom(&mut self) -> CMDCOM_W {
        CMDCOM_W { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    pub fn trancom(&mut self) -> TRANCOM_W {
        TRANCOM_W { w: self }
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    pub fn blkgapevt(&mut self) -> BLKGAPEVT_W {
        BLKGAPEVT_W { w: self }
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline(always)]
    pub fn dmaint(&mut self) -> DMAINT_W {
        DMAINT_W { w: self }
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    pub fn bfrwrrdy(&mut self) -> BFRWRRDY_W {
        BFRWRRDY_W { w: self }
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    pub fn bfrrdrdy(&mut self) -> BFRRDRDY_W {
        BFRRDRDY_W { w: self }
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline(always)]
    pub fn cardins(&mut self) -> CARDINS_W {
        CARDINS_W { w: self }
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline(always)]
    pub fn cardrm(&mut self) -> CARDRM_W {
        CARDRM_W { w: self }
    }
    #[doc = "Bit 13 - Boot Ack Received"]
    #[inline(always)]
    pub fn bootackrcv(&mut self) -> BOOTACKRCV_W {
        BOOTACKRCV_W { w: self }
    }
    #[doc = "Bit 14 - Boot Terminate Interrupt"]
    #[inline(always)]
    pub fn bootterminate(&mut self) -> BOOTTERMINATE_W {
        BOOTTERMINATE_W { w: self }
    }
    #[doc = "Bit 16 - Command Timeout Error"]
    #[inline(always)]
    pub fn cmdtouterr(&mut self) -> CMDTOUTERR_W {
        CMDTOUTERR_W { w: self }
    }
    #[doc = "Bit 17 - CMD CRC Error"]
    #[inline(always)]
    pub fn cmdcrcerr(&mut self) -> CMDCRCERR_W {
        CMDCRCERR_W { w: self }
    }
    #[doc = "Bit 18 - Command End Bit Error"]
    #[inline(always)]
    pub fn cmdendbiterr(&mut self) -> CMDENDBITERR_W {
        CMDENDBITERR_W { w: self }
    }
    #[doc = "Bit 19 - Command Index Error"]
    #[inline(always)]
    pub fn cmdindexerr(&mut self) -> CMDINDEXERR_W {
        CMDINDEXERR_W { w: self }
    }
    #[doc = "Bit 20 - Data Time-out Error"]
    #[inline(always)]
    pub fn dattouterr(&mut self) -> DATTOUTERR_W {
        DATTOUTERR_W { w: self }
    }
    #[doc = "Bit 21 - Data CRC Error"]
    #[inline(always)]
    pub fn datcrcerr(&mut self) -> DATCRCERR_W {
        DATCRCERR_W { w: self }
    }
    #[doc = "Bit 22 - Data End Bit Error"]
    #[inline(always)]
    pub fn datendbiterr(&mut self) -> DATENDBITERR_W {
        DATENDBITERR_W { w: self }
    }
    #[doc = "Bit 23 - Current Limit Error"]
    #[inline(always)]
    pub fn currentlimiterr(&mut self) -> CURRENTLIMITERR_W {
        CURRENTLIMITERR_W { w: self }
    }
    #[doc = "Bit 24 - Auto CMD Error"]
    #[inline(always)]
    pub fn autocmderr(&mut self) -> AUTOCMDERR_W {
        AUTOCMDERR_W { w: self }
    }
    #[doc = "Bit 25 - ADMA Error"]
    #[inline(always)]
    pub fn admaerr(&mut self) -> ADMAERR_W {
        ADMAERR_W { w: self }
    }
    #[doc = "Bit 28 - Specific Error STAT"]
    #[inline(always)]
    pub fn targetresp(&mut self) -> TARGETRESP_W {
        TARGETRESP_W { w: self }
    }
}
