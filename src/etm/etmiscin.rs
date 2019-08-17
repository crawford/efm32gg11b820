#[doc = "Reader of register ETMISCIN"]
pub type R = crate::R<u32, super::ETMISCIN>;
#[doc = "Writer for register ETMISCIN"]
pub type W = crate::W<u32, super::ETMISCIN>;
#[doc = "Register ETMISCIN `reset()`'s with value 0"]
impl crate::ResetValue for super::ETMISCIN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTIN`"]
pub type EXTIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTIN`"]
pub struct EXTIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `COREHALT`"]
pub type COREHALT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COREHALT`"]
pub struct COREHALT_W<'a> {
    w: &'a mut W,
}
impl<'a> COREHALT_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - EXTIN Value"]
    #[inline(always)]
    pub fn extin(&self) -> EXTIN_R {
        EXTIN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - Core Halt"]
    #[inline(always)]
    pub fn corehalt(&self) -> COREHALT_R {
        COREHALT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - EXTIN Value"]
    #[inline(always)]
    pub fn extin(&mut self) -> EXTIN_W {
        EXTIN_W { w: self }
    }
    #[doc = "Bit 4 - Core Halt"]
    #[inline(always)]
    pub fn corehalt(&mut self) -> COREHALT_W {
        COREHALT_W { w: self }
    }
}
