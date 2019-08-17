#[doc = "Reader of register CLKDIV"]
pub type R = crate::R<u32, super::CLKDIV>;
#[doc = "Writer for register CLKDIV"]
pub type W = crate::W<u32, super::CLKDIV>;
#[doc = "Register CLKDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIV`"]
pub type DIV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DIV`"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 3)) | (((value as u32) & 0x000f_ffff) << 3);
        self.w
    }
}
#[doc = "Reader of field `AUTOBAUDEN`"]
pub type AUTOBAUDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOBAUDEN`"]
pub struct AUTOBAUDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOBAUDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:22 - Fractional Clock Divider"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 3) & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 31 - AUTOBAUD Detection Enable"]
    #[inline(always)]
    pub fn autobauden(&self) -> AUTOBAUDEN_R {
        AUTOBAUDEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:22 - Fractional Clock Divider"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Bit 31 - AUTOBAUD Detection Enable"]
    #[inline(always)]
    pub fn autobauden(&mut self) -> AUTOBAUDEN_W {
        AUTOBAUDEN_W { w: self }
    }
}
