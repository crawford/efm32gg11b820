#[doc = "Reader of register POLARITY3"]
pub type R = crate::R<u32, super::POLARITY3>;
#[doc = "Writer for register POLARITY3"]
pub type W = crate::W<u32, super::POLARITY3>;
#[doc = "Register POLARITY3 `reset()`'s with value 0"]
impl crate::ResetValue for super::POLARITY3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSPOL`"]
pub type CSPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSPOL`"]
pub struct CSPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSPOL_W<'a> {
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
#[doc = "Reader of field `REPOL`"]
pub type REPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REPOL`"]
pub struct REPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> REPOL_W<'a> {
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
#[doc = "Reader of field `WEPOL`"]
pub type WEPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WEPOL`"]
pub struct WEPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> WEPOL_W<'a> {
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
#[doc = "Reader of field `ALEPOL`"]
pub type ALEPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALEPOL`"]
pub struct ALEPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> ALEPOL_W<'a> {
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
#[doc = "Reader of field `ARDYPOL`"]
pub type ARDYPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARDYPOL`"]
pub struct ARDYPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> ARDYPOL_W<'a> {
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
#[doc = "Reader of field `BLPOL`"]
pub type BLPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLPOL`"]
pub struct BLPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> BLPOL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Chip Select Polarity"]
    #[inline(always)]
    pub fn cspol(&self) -> CSPOL_R {
        CSPOL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read Enable Polarity"]
    #[inline(always)]
    pub fn repol(&self) -> REPOL_R {
        REPOL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write Enable Polarity"]
    #[inline(always)]
    pub fn wepol(&self) -> WEPOL_R {
        WEPOL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Address Latch Polarity"]
    #[inline(always)]
    pub fn alepol(&self) -> ALEPOL_R {
        ALEPOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ARDY Polarity"]
    #[inline(always)]
    pub fn ardypol(&self) -> ARDYPOL_R {
        ARDYPOL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - BL Polarity"]
    #[inline(always)]
    pub fn blpol(&self) -> BLPOL_R {
        BLPOL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Chip Select Polarity"]
    #[inline(always)]
    pub fn cspol(&mut self) -> CSPOL_W {
        CSPOL_W { w: self }
    }
    #[doc = "Bit 1 - Read Enable Polarity"]
    #[inline(always)]
    pub fn repol(&mut self) -> REPOL_W {
        REPOL_W { w: self }
    }
    #[doc = "Bit 2 - Write Enable Polarity"]
    #[inline(always)]
    pub fn wepol(&mut self) -> WEPOL_W {
        WEPOL_W { w: self }
    }
    #[doc = "Bit 3 - Address Latch Polarity"]
    #[inline(always)]
    pub fn alepol(&mut self) -> ALEPOL_W {
        ALEPOL_W { w: self }
    }
    #[doc = "Bit 4 - ARDY Polarity"]
    #[inline(always)]
    pub fn ardypol(&mut self) -> ARDYPOL_W {
        ARDYPOL_W { w: self }
    }
    #[doc = "Bit 5 - BL Polarity"]
    #[inline(always)]
    pub fn blpol(&mut self) -> BLPOL_W {
        BLPOL_W { w: self }
    }
}
