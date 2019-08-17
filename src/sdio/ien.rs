#[doc = "Reader of register IEN"]
pub type R = crate::R<u32, super::IEN>;
#[doc = "Writer for register IEN"]
pub type W = crate::W<u32, super::IEN>;
#[doc = "Register IEN `reset()`'s with value 0"]
impl crate::ResetValue for super::IEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMDCOMSEN`"]
pub type CMDCOMSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDCOMSEN`"]
pub struct CMDCOMSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDCOMSEN_W<'a> {
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
#[doc = "Reader of field `TRANCOMSEN`"]
pub type TRANCOMSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRANCOMSEN`"]
pub struct TRANCOMSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANCOMSEN_W<'a> {
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
#[doc = "Reader of field `BLKGAPEVTSEN`"]
pub type BLKGAPEVTSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLKGAPEVTSEN`"]
pub struct BLKGAPEVTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLKGAPEVTSEN_W<'a> {
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
#[doc = "Reader of field `DMAINTSEN`"]
pub type DMAINTSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAINTSEN`"]
pub struct DMAINTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAINTSEN_W<'a> {
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
#[doc = "Reader of field `BUFWRRDYSEN`"]
pub type BUFWRRDYSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUFWRRDYSEN`"]
pub struct BUFWRRDYSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFWRRDYSEN_W<'a> {
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
#[doc = "Reader of field `BUFRDRDYSEN`"]
pub type BUFRDRDYSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUFRDRDYSEN`"]
pub struct BUFRDRDYSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFRDRDYSEN_W<'a> {
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
#[doc = "Reader of field `CARDINSSEN`"]
pub type CARDINSSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARDINSSEN`"]
pub struct CARDINSSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARDINSSEN_W<'a> {
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
#[doc = "Reader of field `CARDREMSEN`"]
pub type CARDREMSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARDREMSEN`"]
pub struct CARDREMSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARDREMSEN_W<'a> {
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
#[doc = "Reader of field `CARDINTSEN`"]
pub type CARDINTSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARDINTSEN`"]
pub struct CARDINTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARDINTSEN_W<'a> {
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
#[doc = "Reader of field `RETUNINGEVTSEN`"]
pub type RETUNINGEVTSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RETUNINGEVTSEN`"]
pub struct RETUNINGEVTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RETUNINGEVTSEN_W<'a> {
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
#[doc = "Reader of field `BOOTACKRCVSEN`"]
pub type BOOTACKRCVSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOTACKRCVSEN`"]
pub struct BOOTACKRCVSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTACKRCVSEN_W<'a> {
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
#[doc = "Reader of field `BOOTTERMINATESEN`"]
pub type BOOTTERMINATESEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOTTERMINATESEN`"]
pub struct BOOTTERMINATESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTTERMINATESEN_W<'a> {
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
#[doc = "Reader of field `CMDTOUTERRSEN`"]
pub type CMDTOUTERRSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDTOUTERRSEN`"]
pub struct CMDTOUTERRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDTOUTERRSEN_W<'a> {
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
#[doc = "Reader of field `CMDCRCERRSEN`"]
pub type CMDCRCERRSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDCRCERRSEN`"]
pub struct CMDCRCERRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDCRCERRSEN_W<'a> {
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
#[doc = "Reader of field `CMDENDBITERRSEN`"]
pub type CMDENDBITERRSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDENDBITERRSEN`"]
pub struct CMDENDBITERRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDENDBITERRSEN_W<'a> {
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
#[doc = "Reader of field `CMDINDEXERRSEN`"]
pub type CMDINDEXERRSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDINDEXERRSEN`"]
pub struct CMDINDEXERRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDINDEXERRSEN_W<'a> {
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
#[doc = "Reader of field `DATTOUTERRSEN`"]
pub type DATTOUTERRSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATTOUTERRSEN`"]
pub struct DATTOUTERRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATTOUTERRSEN_W<'a> {
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
#[doc = "Reader of field `DATCRCERRSEN`"]
pub type DATCRCERRSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATCRCERRSEN`"]
pub struct DATCRCERRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATCRCERRSEN_W<'a> {
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
#[doc = "Reader of field `DATENDBITERRSEN`"]
pub type DATENDBITERRSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATENDBITERRSEN`"]
pub struct DATENDBITERRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATENDBITERRSEN_W<'a> {
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
#[doc = "Reader of field `CURRENTLIMITERRSEN`"]
pub type CURRENTLIMITERRSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CURRENTLIMITERRSEN`"]
pub struct CURRENTLIMITERRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CURRENTLIMITERRSEN_W<'a> {
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
#[doc = "Reader of field `AUTOCMDERRSEN`"]
pub type AUTOCMDERRSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOCMDERRSEN`"]
pub struct AUTOCMDERRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOCMDERRSEN_W<'a> {
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
#[doc = "Reader of field `ADMAERRSEN`"]
pub type ADMAERRSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADMAERRSEN`"]
pub struct ADMAERRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADMAERRSEN_W<'a> {
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
#[doc = "Reader of field `TUNINGERRSIGNALENABLE`"]
pub type TUNINGERRSIGNALENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TUNINGERRSIGNALENABLE`"]
pub struct TUNINGERRSIGNALENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNINGERRSIGNALENABLE_W<'a> {
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
#[doc = "Reader of field `TARGETRESPERRSEN`"]
pub type TARGETRESPERRSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TARGETRESPERRSEN`"]
pub struct TARGETRESPERRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGETRESPERRSEN_W<'a> {
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
    pub fn cmdcomsen(&self) -> CMDCOMSEN_R {
        CMDCOMSEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable"]
    #[inline(always)]
    pub fn trancomsen(&self) -> TRANCOMSEN_R {
        TRANCOMSEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable"]
    #[inline(always)]
    pub fn blkgapevtsen(&self) -> BLKGAPEVTSEN_R {
        BLKGAPEVTSEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Signal Enable"]
    #[inline(always)]
    pub fn dmaintsen(&self) -> DMAINTSEN_R {
        DMAINTSEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable"]
    #[inline(always)]
    pub fn bufwrrdysen(&self) -> BUFWRRDYSEN_R {
        BUFWRRDYSEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable"]
    #[inline(always)]
    pub fn bufrdrdysen(&self) -> BUFRDRDYSEN_R {
        BUFRDRDYSEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Signal Enable"]
    #[inline(always)]
    pub fn cardinssen(&self) -> CARDINSSEN_R {
        CARDINSSEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Card Removal Signal Enable"]
    #[inline(always)]
    pub fn cardremsen(&self) -> CARDREMSEN_R {
        CARDREMSEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Signal Enable"]
    #[inline(always)]
    pub fn cardintsen(&self) -> CARDINTSEN_R {
        CARDINTSEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Re-Tuning Event Signal Enable"]
    #[inline(always)]
    pub fn retuningevtsen(&self) -> RETUNINGEVTSEN_R {
        RETUNINGEVTSEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Boot Ack Received Signal Enable"]
    #[inline(always)]
    pub fn bootackrcvsen(&self) -> BOOTACKRCVSEN_R {
        BOOTACKRCVSEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Boot Terminate Interrupt Signal Enable"]
    #[inline(always)]
    pub fn bootterminatesen(&self) -> BOOTTERMINATESEN_R {
        BOOTTERMINATESEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Command Timeout Error Signal Enable"]
    #[inline(always)]
    pub fn cmdtouterrsen(&self) -> CMDTOUTERRSEN_R {
        CMDTOUTERRSEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Command CRC Error Signal Enable"]
    #[inline(always)]
    pub fn cmdcrcerrsen(&self) -> CMDCRCERRSEN_R {
        CMDCRCERRSEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Command End Bit Error Signal Enable"]
    #[inline(always)]
    pub fn cmdendbiterrsen(&self) -> CMDENDBITERRSEN_R {
        CMDENDBITERRSEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Command Index Error Signal Enable"]
    #[inline(always)]
    pub fn cmdindexerrsen(&self) -> CMDINDEXERRSEN_R {
        CMDINDEXERRSEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Data Timeout Error Signal Enable"]
    #[inline(always)]
    pub fn dattouterrsen(&self) -> DATTOUTERRSEN_R {
        DATTOUTERRSEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error Signal Enable"]
    #[inline(always)]
    pub fn datcrcerrsen(&self) -> DATCRCERRSEN_R {
        DATCRCERRSEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Data End Bit Error Signal Enable"]
    #[inline(always)]
    pub fn datendbiterrsen(&self) -> DATENDBITERRSEN_R {
        DATENDBITERRSEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Current Limit Error Signal Enable"]
    #[inline(always)]
    pub fn currentlimiterrsen(&self) -> CURRENTLIMITERRSEN_R {
        CURRENTLIMITERRSEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Auto CMD12 Error Signal Enable"]
    #[inline(always)]
    pub fn autocmderrsen(&self) -> AUTOCMDERRSEN_R {
        AUTOCMDERRSEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - ADMA Error Signal Enable"]
    #[inline(always)]
    pub fn admaerrsen(&self) -> ADMAERRSEN_R {
        ADMAERRSEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Tuning Error Signal Enable"]
    #[inline(always)]
    pub fn tuningerrsignalenable(&self) -> TUNINGERRSIGNALENABLE_R {
        TUNINGERRSIGNALENABLE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Target Response Error Signal Enable"]
    #[inline(always)]
    pub fn targetresperrsen(&self) -> TARGETRESPERRSEN_R {
        TARGETRESPERRSEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Signal Enable"]
    #[inline(always)]
    pub fn cmdcomsen(&mut self) -> CMDCOMSEN_W {
        CMDCOMSEN_W { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable"]
    #[inline(always)]
    pub fn trancomsen(&mut self) -> TRANCOMSEN_W {
        TRANCOMSEN_W { w: self }
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable"]
    #[inline(always)]
    pub fn blkgapevtsen(&mut self) -> BLKGAPEVTSEN_W {
        BLKGAPEVTSEN_W { w: self }
    }
    #[doc = "Bit 3 - DMA Interrupt Signal Enable"]
    #[inline(always)]
    pub fn dmaintsen(&mut self) -> DMAINTSEN_W {
        DMAINTSEN_W { w: self }
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable"]
    #[inline(always)]
    pub fn bufwrrdysen(&mut self) -> BUFWRRDYSEN_W {
        BUFWRRDYSEN_W { w: self }
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable"]
    #[inline(always)]
    pub fn bufrdrdysen(&mut self) -> BUFRDRDYSEN_W {
        BUFRDRDYSEN_W { w: self }
    }
    #[doc = "Bit 6 - Card Insertion Signal Enable"]
    #[inline(always)]
    pub fn cardinssen(&mut self) -> CARDINSSEN_W {
        CARDINSSEN_W { w: self }
    }
    #[doc = "Bit 7 - Card Removal Signal Enable"]
    #[inline(always)]
    pub fn cardremsen(&mut self) -> CARDREMSEN_W {
        CARDREMSEN_W { w: self }
    }
    #[doc = "Bit 8 - Card Interrupt Signal Enable"]
    #[inline(always)]
    pub fn cardintsen(&mut self) -> CARDINTSEN_W {
        CARDINTSEN_W { w: self }
    }
    #[doc = "Bit 12 - Re-Tuning Event Signal Enable"]
    #[inline(always)]
    pub fn retuningevtsen(&mut self) -> RETUNINGEVTSEN_W {
        RETUNINGEVTSEN_W { w: self }
    }
    #[doc = "Bit 13 - Boot Ack Received Signal Enable"]
    #[inline(always)]
    pub fn bootackrcvsen(&mut self) -> BOOTACKRCVSEN_W {
        BOOTACKRCVSEN_W { w: self }
    }
    #[doc = "Bit 14 - Boot Terminate Interrupt Signal Enable"]
    #[inline(always)]
    pub fn bootterminatesen(&mut self) -> BOOTTERMINATESEN_W {
        BOOTTERMINATESEN_W { w: self }
    }
    #[doc = "Bit 16 - Command Timeout Error Signal Enable"]
    #[inline(always)]
    pub fn cmdtouterrsen(&mut self) -> CMDTOUTERRSEN_W {
        CMDTOUTERRSEN_W { w: self }
    }
    #[doc = "Bit 17 - Command CRC Error Signal Enable"]
    #[inline(always)]
    pub fn cmdcrcerrsen(&mut self) -> CMDCRCERRSEN_W {
        CMDCRCERRSEN_W { w: self }
    }
    #[doc = "Bit 18 - Command End Bit Error Signal Enable"]
    #[inline(always)]
    pub fn cmdendbiterrsen(&mut self) -> CMDENDBITERRSEN_W {
        CMDENDBITERRSEN_W { w: self }
    }
    #[doc = "Bit 19 - Command Index Error Signal Enable"]
    #[inline(always)]
    pub fn cmdindexerrsen(&mut self) -> CMDINDEXERRSEN_W {
        CMDINDEXERRSEN_W { w: self }
    }
    #[doc = "Bit 20 - Data Timeout Error Signal Enable"]
    #[inline(always)]
    pub fn dattouterrsen(&mut self) -> DATTOUTERRSEN_W {
        DATTOUTERRSEN_W { w: self }
    }
    #[doc = "Bit 21 - Data CRC Error Signal Enable"]
    #[inline(always)]
    pub fn datcrcerrsen(&mut self) -> DATCRCERRSEN_W {
        DATCRCERRSEN_W { w: self }
    }
    #[doc = "Bit 22 - Data End Bit Error Signal Enable"]
    #[inline(always)]
    pub fn datendbiterrsen(&mut self) -> DATENDBITERRSEN_W {
        DATENDBITERRSEN_W { w: self }
    }
    #[doc = "Bit 23 - Current Limit Error Signal Enable"]
    #[inline(always)]
    pub fn currentlimiterrsen(&mut self) -> CURRENTLIMITERRSEN_W {
        CURRENTLIMITERRSEN_W { w: self }
    }
    #[doc = "Bit 24 - Auto CMD12 Error Signal Enable"]
    #[inline(always)]
    pub fn autocmderrsen(&mut self) -> AUTOCMDERRSEN_W {
        AUTOCMDERRSEN_W { w: self }
    }
    #[doc = "Bit 25 - ADMA Error Signal Enable"]
    #[inline(always)]
    pub fn admaerrsen(&mut self) -> ADMAERRSEN_W {
        ADMAERRSEN_W { w: self }
    }
    #[doc = "Bit 26 - Tuning Error Signal Enable"]
    #[inline(always)]
    pub fn tuningerrsignalenable(&mut self) -> TUNINGERRSIGNALENABLE_W {
        TUNINGERRSIGNALENABLE_W { w: self }
    }
    #[doc = "Bit 28 - Target Response Error Signal Enable"]
    #[inline(always)]
    pub fn targetresperrsen(&mut self) -> TARGETRESPERRSEN_W {
        TARGETRESPERRSEN_W { w: self }
    }
}
