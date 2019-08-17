#[doc = "Reader of register CONFIG"]
pub type R = crate::R<u32, super::CONFIG>;
#[doc = "Writer for register CONFIG"]
pub type W = crate::W<u32, super::CONFIG>;
#[doc = "Register CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::CONFIG {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Debug Halt"]
    #[inline(always)]
    pub fn dbghalt(&self) -> DBGHALT_R {
        DBGHALT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Debug Halt"]
    #[inline(always)]
    pub fn dbghalt(&mut self) -> DBGHALT_W {
        DBGHALT_W { w: self }
    }
}
