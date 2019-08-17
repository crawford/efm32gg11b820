#[doc = "Reader of register CTRLX"]
pub type R = crate::R<u32, super::CTRLX>;
#[doc = "Writer for register CTRLX"]
pub type W = crate::W<u32, super::CTRLX>;
#[doc = "Register CTRLX `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DBGHALT`"]
pub type DBGHALT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGHALT`"]
pub struct DBGHALT_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGHALT_W<'a> {
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
#[doc = "Reader of field `CTSINV`"]
pub type CTSINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTSINV`"]
pub struct CTSINV_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSINV_W<'a> {
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
#[doc = "Reader of field `CTSEN`"]
pub type CTSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTSEN`"]
pub struct CTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSEN_W<'a> {
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
#[doc = "Reader of field `RTSINV`"]
pub type RTSINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTSINV`"]
pub struct RTSINV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSINV_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Debug Halt"]
    #[inline(always)]
    pub fn dbghalt(&self) -> DBGHALT_R {
        DBGHALT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTS Pin Inversion"]
    #[inline(always)]
    pub fn ctsinv(&self) -> CTSINV_R {
        CTSINV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CTS Function Enabled"]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTS Pin Inversion"]
    #[inline(always)]
    pub fn rtsinv(&self) -> RTSINV_R {
        RTSINV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Halt"]
    #[inline(always)]
    pub fn dbghalt(&mut self) -> DBGHALT_W {
        DBGHALT_W { w: self }
    }
    #[doc = "Bit 1 - CTS Pin Inversion"]
    #[inline(always)]
    pub fn ctsinv(&mut self) -> CTSINV_W {
        CTSINV_W { w: self }
    }
    #[doc = "Bit 2 - CTS Function Enabled"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CTSEN_W {
        CTSEN_W { w: self }
    }
    #[doc = "Bit 3 - RTS Pin Inversion"]
    #[inline(always)]
    pub fn rtsinv(&mut self) -> RTSINV_W {
        RTSINV_W { w: self }
    }
}
