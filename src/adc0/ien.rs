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
#[doc = "Reader of field `SINGLE`"]
pub type SINGLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SINGLE`"]
pub struct SINGLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLE_W<'a> {
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
#[doc = "Reader of field `SCAN`"]
pub type SCAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCAN`"]
pub struct SCAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_W<'a> {
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
#[doc = "Reader of field `SINGLEOF`"]
pub type SINGLEOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SINGLEOF`"]
pub struct SINGLEOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLEOF_W<'a> {
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
#[doc = "Reader of field `SCANOF`"]
pub type SCANOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCANOF`"]
pub struct SCANOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANOF_W<'a> {
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
#[doc = "Reader of field `SINGLEUF`"]
pub type SINGLEUF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SINGLEUF`"]
pub struct SINGLEUF_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLEUF_W<'a> {
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
#[doc = "Reader of field `SCANUF`"]
pub type SCANUF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCANUF`"]
pub struct SCANUF_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANUF_W<'a> {
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
#[doc = "Reader of field `SINGLECMP`"]
pub type SINGLECMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SINGLECMP`"]
pub struct SINGLECMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLECMP_W<'a> {
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
#[doc = "Reader of field `SCANCMP`"]
pub type SCANCMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCANCMP`"]
pub struct SCANCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANCMP_W<'a> {
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
#[doc = "Reader of field `VREFOV`"]
pub type VREFOV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREFOV`"]
pub struct VREFOV_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFOV_W<'a> {
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
#[doc = "Reader of field `PROGERR`"]
pub type PROGERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROGERR`"]
pub struct PROGERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PROGERR_W<'a> {
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
#[doc = "Reader of field `SCANEXTPEND`"]
pub type SCANEXTPEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCANEXTPEND`"]
pub struct SCANEXTPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANEXTPEND_W<'a> {
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
#[doc = "Reader of field `SCANPEND`"]
pub type SCANPEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCANPEND`"]
pub struct SCANPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANPEND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `PRSTIMEDERR`"]
pub type PRSTIMEDERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRSTIMEDERR`"]
pub struct PRSTIMEDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSTIMEDERR_W<'a> {
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
#[doc = "Reader of field `EM23ERR`"]
pub type EM23ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM23ERR`"]
pub struct EM23ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> EM23ERR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - SINGLE Interrupt Enable"]
    #[inline(always)]
    pub fn single(&self) -> SINGLE_R {
        SINGLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SCAN Interrupt Enable"]
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SINGLEOF Interrupt Enable"]
    #[inline(always)]
    pub fn singleof(&self) -> SINGLEOF_R {
        SINGLEOF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SCANOF Interrupt Enable"]
    #[inline(always)]
    pub fn scanof(&self) -> SCANOF_R {
        SCANOF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SINGLEUF Interrupt Enable"]
    #[inline(always)]
    pub fn singleuf(&self) -> SINGLEUF_R {
        SINGLEUF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SCANUF Interrupt Enable"]
    #[inline(always)]
    pub fn scanuf(&self) -> SCANUF_R {
        SCANUF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SINGLECMP Interrupt Enable"]
    #[inline(always)]
    pub fn singlecmp(&self) -> SINGLECMP_R {
        SINGLECMP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SCANCMP Interrupt Enable"]
    #[inline(always)]
    pub fn scancmp(&self) -> SCANCMP_R {
        SCANCMP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 24 - VREFOV Interrupt Enable"]
    #[inline(always)]
    pub fn vrefov(&self) -> VREFOV_R {
        VREFOV_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - PROGERR Interrupt Enable"]
    #[inline(always)]
    pub fn progerr(&self) -> PROGERR_R {
        PROGERR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - SCANEXTPEND Interrupt Enable"]
    #[inline(always)]
    pub fn scanextpend(&self) -> SCANEXTPEND_R {
        SCANEXTPEND_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - SCANPEND Interrupt Enable"]
    #[inline(always)]
    pub fn scanpend(&self) -> SCANPEND_R {
        SCANPEND_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn prstimederr(&self) -> PRSTIMEDERR_R {
        PRSTIMEDERR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - EM23ERR Interrupt Enable"]
    #[inline(always)]
    pub fn em23err(&self) -> EM23ERR_R {
        EM23ERR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SINGLE Interrupt Enable"]
    #[inline(always)]
    pub fn single(&mut self) -> SINGLE_W {
        SINGLE_W { w: self }
    }
    #[doc = "Bit 1 - SCAN Interrupt Enable"]
    #[inline(always)]
    pub fn scan(&mut self) -> SCAN_W {
        SCAN_W { w: self }
    }
    #[doc = "Bit 8 - SINGLEOF Interrupt Enable"]
    #[inline(always)]
    pub fn singleof(&mut self) -> SINGLEOF_W {
        SINGLEOF_W { w: self }
    }
    #[doc = "Bit 9 - SCANOF Interrupt Enable"]
    #[inline(always)]
    pub fn scanof(&mut self) -> SCANOF_W {
        SCANOF_W { w: self }
    }
    #[doc = "Bit 10 - SINGLEUF Interrupt Enable"]
    #[inline(always)]
    pub fn singleuf(&mut self) -> SINGLEUF_W {
        SINGLEUF_W { w: self }
    }
    #[doc = "Bit 11 - SCANUF Interrupt Enable"]
    #[inline(always)]
    pub fn scanuf(&mut self) -> SCANUF_W {
        SCANUF_W { w: self }
    }
    #[doc = "Bit 16 - SINGLECMP Interrupt Enable"]
    #[inline(always)]
    pub fn singlecmp(&mut self) -> SINGLECMP_W {
        SINGLECMP_W { w: self }
    }
    #[doc = "Bit 17 - SCANCMP Interrupt Enable"]
    #[inline(always)]
    pub fn scancmp(&mut self) -> SCANCMP_W {
        SCANCMP_W { w: self }
    }
    #[doc = "Bit 24 - VREFOV Interrupt Enable"]
    #[inline(always)]
    pub fn vrefov(&mut self) -> VREFOV_W {
        VREFOV_W { w: self }
    }
    #[doc = "Bit 25 - PROGERR Interrupt Enable"]
    #[inline(always)]
    pub fn progerr(&mut self) -> PROGERR_W {
        PROGERR_W { w: self }
    }
    #[doc = "Bit 26 - SCANEXTPEND Interrupt Enable"]
    #[inline(always)]
    pub fn scanextpend(&mut self) -> SCANEXTPEND_W {
        SCANEXTPEND_W { w: self }
    }
    #[doc = "Bit 27 - SCANPEND Interrupt Enable"]
    #[inline(always)]
    pub fn scanpend(&mut self) -> SCANPEND_W {
        SCANPEND_W { w: self }
    }
    #[doc = "Bit 28 - PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn prstimederr(&mut self) -> PRSTIMEDERR_W {
        PRSTIMEDERR_W { w: self }
    }
    #[doc = "Bit 29 - EM23ERR Interrupt Enable"]
    #[inline(always)]
    pub fn em23err(&mut self) -> EM23ERR_W {
        EM23ERR_W { w: self }
    }
}
