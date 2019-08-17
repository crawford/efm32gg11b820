#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Reader of field `POLYSEL`"]
pub type POLYSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POLYSEL`"]
pub struct POLYSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> POLYSEL_W<'a> {
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
#[doc = "Reader of field `BYTEMODE`"]
pub type BYTEMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYTEMODE`"]
pub struct BYTEMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTEMODE_W<'a> {
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
#[doc = "Reader of field `BITREVERSE`"]
pub type BITREVERSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BITREVERSE`"]
pub struct BITREVERSE_W<'a> {
    w: &'a mut W,
}
impl<'a> BITREVERSE_W<'a> {
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
#[doc = "Reader of field `BYTEREVERSE`"]
pub type BYTEREVERSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYTEREVERSE`"]
pub struct BYTEREVERSE_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTEREVERSE_W<'a> {
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
#[doc = "Reader of field `AUTOINIT`"]
pub type AUTOINIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOINIT`"]
pub struct AUTOINIT_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOINIT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - CRC Functionality Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Polynomial Select"]
    #[inline(always)]
    pub fn polysel(&self) -> POLYSEL_R {
        POLYSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Byte Mode Enable"]
    #[inline(always)]
    pub fn bytemode(&self) -> BYTEMODE_R {
        BYTEMODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Byte-level Bit Reverse Enable"]
    #[inline(always)]
    pub fn bitreverse(&self) -> BITREVERSE_R {
        BITREVERSE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Byte Reverse Mode"]
    #[inline(always)]
    pub fn bytereverse(&self) -> BYTEREVERSE_R {
        BYTEREVERSE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Auto Init Enable"]
    #[inline(always)]
    pub fn autoinit(&self) -> AUTOINIT_R {
        AUTOINIT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Functionality Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 4 - Polynomial Select"]
    #[inline(always)]
    pub fn polysel(&mut self) -> POLYSEL_W {
        POLYSEL_W { w: self }
    }
    #[doc = "Bit 8 - Byte Mode Enable"]
    #[inline(always)]
    pub fn bytemode(&mut self) -> BYTEMODE_W {
        BYTEMODE_W { w: self }
    }
    #[doc = "Bit 9 - Byte-level Bit Reverse Enable"]
    #[inline(always)]
    pub fn bitreverse(&mut self) -> BITREVERSE_W {
        BITREVERSE_W { w: self }
    }
    #[doc = "Bit 10 - Byte Reverse Mode"]
    #[inline(always)]
    pub fn bytereverse(&mut self) -> BYTEREVERSE_W {
        BYTEREVERSE_W { w: self }
    }
    #[doc = "Bit 13 - Auto Init Enable"]
    #[inline(always)]
    pub fn autoinit(&mut self) -> AUTOINIT_W {
        AUTOINIT_W { w: self }
    }
}
