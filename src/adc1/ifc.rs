#[doc = "Writer for register IFC"]
pub type W = crate::W<u32, super::IFC>;
#[doc = "Register IFC `reset()`'s with value 0"]
impl crate::ResetValue for super::IFC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
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
impl W {
    #[doc = "Bit 8 - Clear SINGLEOF Interrupt Flag"]
    #[inline(always)]
    pub fn singleof(&mut self) -> SINGLEOF_W {
        SINGLEOF_W { w: self }
    }
    #[doc = "Bit 9 - Clear SCANOF Interrupt Flag"]
    #[inline(always)]
    pub fn scanof(&mut self) -> SCANOF_W {
        SCANOF_W { w: self }
    }
    #[doc = "Bit 10 - Clear SINGLEUF Interrupt Flag"]
    #[inline(always)]
    pub fn singleuf(&mut self) -> SINGLEUF_W {
        SINGLEUF_W { w: self }
    }
    #[doc = "Bit 11 - Clear SCANUF Interrupt Flag"]
    #[inline(always)]
    pub fn scanuf(&mut self) -> SCANUF_W {
        SCANUF_W { w: self }
    }
    #[doc = "Bit 16 - Clear SINGLECMP Interrupt Flag"]
    #[inline(always)]
    pub fn singlecmp(&mut self) -> SINGLECMP_W {
        SINGLECMP_W { w: self }
    }
    #[doc = "Bit 17 - Clear SCANCMP Interrupt Flag"]
    #[inline(always)]
    pub fn scancmp(&mut self) -> SCANCMP_W {
        SCANCMP_W { w: self }
    }
    #[doc = "Bit 24 - Clear VREFOV Interrupt Flag"]
    #[inline(always)]
    pub fn vrefov(&mut self) -> VREFOV_W {
        VREFOV_W { w: self }
    }
    #[doc = "Bit 25 - Clear PROGERR Interrupt Flag"]
    #[inline(always)]
    pub fn progerr(&mut self) -> PROGERR_W {
        PROGERR_W { w: self }
    }
    #[doc = "Bit 26 - Clear SCANEXTPEND Interrupt Flag"]
    #[inline(always)]
    pub fn scanextpend(&mut self) -> SCANEXTPEND_W {
        SCANEXTPEND_W { w: self }
    }
    #[doc = "Bit 27 - Clear SCANPEND Interrupt Flag"]
    #[inline(always)]
    pub fn scanpend(&mut self) -> SCANPEND_W {
        SCANPEND_W { w: self }
    }
    #[doc = "Bit 28 - Clear PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    pub fn prstimederr(&mut self) -> PRSTIMEDERR_W {
        PRSTIMEDERR_W { w: self }
    }
    #[doc = "Bit 29 - Clear EM23ERR Interrupt Flag"]
    #[inline(always)]
    pub fn em23err(&mut self) -> EM23ERR_W {
        EM23ERR_W { w: self }
    }
}
