#[doc = "Reader of register IFENC"]
pub type R = crate::R<u32, super::IFENC>;
#[doc = "Writer for register IFENC"]
pub type W = crate::W<u32, super::IFENC>;
#[doc = "Register IFENC `reset()`'s with value 0"]
impl crate::ResetValue for super::IFENC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMDCOMEN`"]
pub type CMDCOMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDCOMEN`"]
pub struct CMDCOMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDCOMEN_W<'a> {
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
#[doc = "Reader of field `TRANCOMEN`"]
pub type TRANCOMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRANCOMEN`"]
pub struct TRANCOMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANCOMEN_W<'a> {
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
#[doc = "Reader of field `BLKGAPEVTEN`"]
pub type BLKGAPEVTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLKGAPEVTEN`"]
pub struct BLKGAPEVTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLKGAPEVTEN_W<'a> {
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
#[doc = "Reader of field `DMAINTEN`"]
pub type DMAINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAINTEN`"]
pub struct DMAINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAINTEN_W<'a> {
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
#[doc = "Reader of field `BUFWRRDYEN`"]
pub type BUFWRRDYEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUFWRRDYEN`"]
pub struct BUFWRRDYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFWRRDYEN_W<'a> {
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
#[doc = "Reader of field `BUFRDRDYEN`"]
pub type BUFRDRDYEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUFRDRDYEN`"]
pub struct BUFRDRDYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFRDRDYEN_W<'a> {
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
#[doc = "Reader of field `CARDINSEN`"]
pub type CARDINSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARDINSEN`"]
pub struct CARDINSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARDINSEN_W<'a> {
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
#[doc = "Reader of field `CARDRMEN`"]
pub type CARDRMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARDRMEN`"]
pub struct CARDRMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARDRMEN_W<'a> {
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
#[doc = "Reader of field `CARDINTEN`"]
pub type CARDINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARDINTEN`"]
pub struct CARDINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARDINTEN_W<'a> {
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
#[doc = "Reader of field `RETUNINGEVTEN`"]
pub type RETUNINGEVTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RETUNINGEVTEN`"]
pub struct RETUNINGEVTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RETUNINGEVTEN_W<'a> {
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
#[doc = "Reader of field `BOOTACKRCVEN`"]
pub type BOOTACKRCVEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOTACKRCVEN`"]
pub struct BOOTACKRCVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTACKRCVEN_W<'a> {
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
#[doc = "Reader of field `BOOTTERMINATEEN`"]
pub type BOOTTERMINATEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOTTERMINATEEN`"]
pub struct BOOTTERMINATEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTTERMINATEEN_W<'a> {
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
#[doc = "Reader of field `CMDTOUTERREN`"]
pub type CMDTOUTERREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDTOUTERREN`"]
pub struct CMDTOUTERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDTOUTERREN_W<'a> {
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
#[doc = "Reader of field `CMDCRCERREN`"]
pub type CMDCRCERREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDCRCERREN`"]
pub struct CMDCRCERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDCRCERREN_W<'a> {
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
#[doc = "Reader of field `CMDENDBITERREN`"]
pub type CMDENDBITERREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDENDBITERREN`"]
pub struct CMDENDBITERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDENDBITERREN_W<'a> {
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
#[doc = "Reader of field `CMDINDEXERREN`"]
pub type CMDINDEXERREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDINDEXERREN`"]
pub struct CMDINDEXERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDINDEXERREN_W<'a> {
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
#[doc = "Reader of field `DATTOUTERREN`"]
pub type DATTOUTERREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATTOUTERREN`"]
pub struct DATTOUTERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATTOUTERREN_W<'a> {
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
#[doc = "Reader of field `DATCRCERREN`"]
pub type DATCRCERREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATCRCERREN`"]
pub struct DATCRCERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATCRCERREN_W<'a> {
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
#[doc = "Reader of field `DATENDBITERREN`"]
pub type DATENDBITERREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATENDBITERREN`"]
pub struct DATENDBITERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATENDBITERREN_W<'a> {
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
#[doc = "Reader of field `CURRENTLIMITERREN`"]
pub type CURRENTLIMITERREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CURRENTLIMITERREN`"]
pub struct CURRENTLIMITERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> CURRENTLIMITERREN_W<'a> {
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
#[doc = "Reader of field `AUTOCMDERREN`"]
pub type AUTOCMDERREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOCMDERREN`"]
pub struct AUTOCMDERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOCMDERREN_W<'a> {
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
#[doc = "Reader of field `ADMAERREN`"]
pub type ADMAERREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADMAERREN`"]
pub struct ADMAERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADMAERREN_W<'a> {
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
#[doc = "Reader of field `TUNINGERREN`"]
pub type TUNINGERREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TUNINGERREN`"]
pub struct TUNINGERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNINGERREN_W<'a> {
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
#[doc = "Reader of field `TARGETRESPEN`"]
pub type TARGETRESPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TARGETRESPEN`"]
pub struct TARGETRESPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGETRESPEN_W<'a> {
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
    #[doc = "Bit 0 - Command Complete Signal Enable"]
    #[inline(always)]
    pub fn cmdcomen(&self) -> CMDCOMEN_R {
        CMDCOMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable"]
    #[inline(always)]
    pub fn trancomen(&self) -> TRANCOMEN_R {
        TRANCOMEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable"]
    #[inline(always)]
    pub fn blkgapevten(&self) -> BLKGAPEVTEN_R {
        BLKGAPEVTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Signal Enable"]
    #[inline(always)]
    pub fn dmainten(&self) -> DMAINTEN_R {
        DMAINTEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable"]
    #[inline(always)]
    pub fn bufwrrdyen(&self) -> BUFWRRDYEN_R {
        BUFWRRDYEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable"]
    #[inline(always)]
    pub fn bufrdrdyen(&self) -> BUFRDRDYEN_R {
        BUFRDRDYEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Signal Enable"]
    #[inline(always)]
    pub fn cardinsen(&self) -> CARDINSEN_R {
        CARDINSEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Card Removal Signal Enable"]
    #[inline(always)]
    pub fn cardrmen(&self) -> CARDRMEN_R {
        CARDRMEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Signal Enable"]
    #[inline(always)]
    pub fn cardinten(&self) -> CARDINTEN_R {
        CARDINTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Re-Tunning Event Signal Enable"]
    #[inline(always)]
    pub fn retuningevten(&self) -> RETUNINGEVTEN_R {
        RETUNINGEVTEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Boot Ack Received Signal Enable"]
    #[inline(always)]
    pub fn bootackrcven(&self) -> BOOTACKRCVEN_R {
        BOOTACKRCVEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Boot Terminate Interrupt Signal Enable"]
    #[inline(always)]
    pub fn bootterminateen(&self) -> BOOTTERMINATEEN_R {
        BOOTTERMINATEEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Command Time-out Error Status Enable"]
    #[inline(always)]
    pub fn cmdtouterren(&self) -> CMDTOUTERREN_R {
        CMDTOUTERREN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Command CRC Error Status Enable"]
    #[inline(always)]
    pub fn cmdcrcerren(&self) -> CMDCRCERREN_R {
        CMDCRCERREN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Command End Bit Error Status Enable"]
    #[inline(always)]
    pub fn cmdendbiterren(&self) -> CMDENDBITERREN_R {
        CMDENDBITERREN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Command Index Error Status Enable"]
    #[inline(always)]
    pub fn cmdindexerren(&self) -> CMDINDEXERREN_R {
        CMDINDEXERREN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Data Timeout Error Status Enable"]
    #[inline(always)]
    pub fn dattouterren(&self) -> DATTOUTERREN_R {
        DATTOUTERREN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error Status Enable"]
    #[inline(always)]
    pub fn datcrcerren(&self) -> DATCRCERREN_R {
        DATCRCERREN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Data End Bit Error Status Enable"]
    #[inline(always)]
    pub fn datendbiterren(&self) -> DATENDBITERREN_R {
        DATENDBITERREN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Current Limit Error Status Enable"]
    #[inline(always)]
    pub fn currentlimiterren(&self) -> CURRENTLIMITERREN_R {
        CURRENTLIMITERREN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Auto CMD12 Error Status Enable"]
    #[inline(always)]
    pub fn autocmderren(&self) -> AUTOCMDERREN_R {
        AUTOCMDERREN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - ADMA Error Status Enable"]
    #[inline(always)]
    pub fn admaerren(&self) -> ADMAERREN_R {
        ADMAERREN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Tuning Error Status Enable"]
    #[inline(always)]
    pub fn tuningerren(&self) -> TUNINGERREN_R {
        TUNINGERREN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Target Response/Host Error Status Enable"]
    #[inline(always)]
    pub fn targetrespen(&self) -> TARGETRESPEN_R {
        TARGETRESPEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Signal Enable"]
    #[inline(always)]
    pub fn cmdcomen(&mut self) -> CMDCOMEN_W {
        CMDCOMEN_W { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable"]
    #[inline(always)]
    pub fn trancomen(&mut self) -> TRANCOMEN_W {
        TRANCOMEN_W { w: self }
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable"]
    #[inline(always)]
    pub fn blkgapevten(&mut self) -> BLKGAPEVTEN_W {
        BLKGAPEVTEN_W { w: self }
    }
    #[doc = "Bit 3 - DMA Interrupt Signal Enable"]
    #[inline(always)]
    pub fn dmainten(&mut self) -> DMAINTEN_W {
        DMAINTEN_W { w: self }
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable"]
    #[inline(always)]
    pub fn bufwrrdyen(&mut self) -> BUFWRRDYEN_W {
        BUFWRRDYEN_W { w: self }
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable"]
    #[inline(always)]
    pub fn bufrdrdyen(&mut self) -> BUFRDRDYEN_W {
        BUFRDRDYEN_W { w: self }
    }
    #[doc = "Bit 6 - Card Insertion Signal Enable"]
    #[inline(always)]
    pub fn cardinsen(&mut self) -> CARDINSEN_W {
        CARDINSEN_W { w: self }
    }
    #[doc = "Bit 7 - Card Removal Signal Enable"]
    #[inline(always)]
    pub fn cardrmen(&mut self) -> CARDRMEN_W {
        CARDRMEN_W { w: self }
    }
    #[doc = "Bit 8 - Card Interrupt Signal Enable"]
    #[inline(always)]
    pub fn cardinten(&mut self) -> CARDINTEN_W {
        CARDINTEN_W { w: self }
    }
    #[doc = "Bit 12 - Re-Tunning Event Signal Enable"]
    #[inline(always)]
    pub fn retuningevten(&mut self) -> RETUNINGEVTEN_W {
        RETUNINGEVTEN_W { w: self }
    }
    #[doc = "Bit 13 - Boot Ack Received Signal Enable"]
    #[inline(always)]
    pub fn bootackrcven(&mut self) -> BOOTACKRCVEN_W {
        BOOTACKRCVEN_W { w: self }
    }
    #[doc = "Bit 14 - Boot Terminate Interrupt Signal Enable"]
    #[inline(always)]
    pub fn bootterminateen(&mut self) -> BOOTTERMINATEEN_W {
        BOOTTERMINATEEN_W { w: self }
    }
    #[doc = "Bit 16 - Command Time-out Error Status Enable"]
    #[inline(always)]
    pub fn cmdtouterren(&mut self) -> CMDTOUTERREN_W {
        CMDTOUTERREN_W { w: self }
    }
    #[doc = "Bit 17 - Command CRC Error Status Enable"]
    #[inline(always)]
    pub fn cmdcrcerren(&mut self) -> CMDCRCERREN_W {
        CMDCRCERREN_W { w: self }
    }
    #[doc = "Bit 18 - Command End Bit Error Status Enable"]
    #[inline(always)]
    pub fn cmdendbiterren(&mut self) -> CMDENDBITERREN_W {
        CMDENDBITERREN_W { w: self }
    }
    #[doc = "Bit 19 - Command Index Error Status Enable"]
    #[inline(always)]
    pub fn cmdindexerren(&mut self) -> CMDINDEXERREN_W {
        CMDINDEXERREN_W { w: self }
    }
    #[doc = "Bit 20 - Data Timeout Error Status Enable"]
    #[inline(always)]
    pub fn dattouterren(&mut self) -> DATTOUTERREN_W {
        DATTOUTERREN_W { w: self }
    }
    #[doc = "Bit 21 - Data CRC Error Status Enable"]
    #[inline(always)]
    pub fn datcrcerren(&mut self) -> DATCRCERREN_W {
        DATCRCERREN_W { w: self }
    }
    #[doc = "Bit 22 - Data End Bit Error Status Enable"]
    #[inline(always)]
    pub fn datendbiterren(&mut self) -> DATENDBITERREN_W {
        DATENDBITERREN_W { w: self }
    }
    #[doc = "Bit 23 - Current Limit Error Status Enable"]
    #[inline(always)]
    pub fn currentlimiterren(&mut self) -> CURRENTLIMITERREN_W {
        CURRENTLIMITERREN_W { w: self }
    }
    #[doc = "Bit 24 - Auto CMD12 Error Status Enable"]
    #[inline(always)]
    pub fn autocmderren(&mut self) -> AUTOCMDERREN_W {
        AUTOCMDERREN_W { w: self }
    }
    #[doc = "Bit 25 - ADMA Error Status Enable"]
    #[inline(always)]
    pub fn admaerren(&mut self) -> ADMAERREN_W {
        ADMAERREN_W { w: self }
    }
    #[doc = "Bit 26 - Tuning Error Status Enable"]
    #[inline(always)]
    pub fn tuningerren(&mut self) -> TUNINGERREN_W {
        TUNINGERREN_W { w: self }
    }
    #[doc = "Bit 28 - Target Response/Host Error Status Enable"]
    #[inline(always)]
    pub fn targetrespen(&mut self) -> TARGETRESPEN_W {
        TARGETRESPEN_W { w: self }
    }
}
