#[doc = "Reader of register SPECADDR1TOP"]
pub type R = crate::R<u32, super::SPECADDR1TOP>;
#[doc = "Writer for register SPECADDR1TOP"]
pub type W = crate::W<u32, super::SPECADDR1TOP>;
#[doc = "Register SPECADDR1TOP `reset()`'s with value 0"]
impl crate::ResetValue for super::SPECADDR1TOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `FILTERTYPE`"]
pub type FILTERTYPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FILTERTYPE`"]
pub struct FILTERTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTERTYPE_W<'a> {
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
impl R {
    #[doc = "Bits 0:15 - Specific address 1 MSB"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - MAC SA or DA selection"]
    #[inline(always)]
    pub fn filtertype(&self) -> FILTERTYPE_R {
        FILTERTYPE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specific address 1 MSB"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Bit 16 - MAC SA or DA selection"]
    #[inline(always)]
    pub fn filtertype(&mut self) -> FILTERTYPE_W {
        FILTERTYPE_W { w: self }
    }
}
