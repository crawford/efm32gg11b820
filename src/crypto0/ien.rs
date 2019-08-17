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
#[doc = "Reader of field `INSTRDONE`"]
pub type INSTRDONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INSTRDONE`"]
pub struct INSTRDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTRDONE_W<'a> {
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
#[doc = "Reader of field `SEQDONE`"]
pub type SEQDONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEQDONE`"]
pub struct SEQDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQDONE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - INSTRDONE Interrupt Enable"]
    #[inline(always)]
    pub fn instrdone(&self) -> INSTRDONE_R {
        INSTRDONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SEQDONE Interrupt Enable"]
    #[inline(always)]
    pub fn seqdone(&self) -> SEQDONE_R {
        SEQDONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INSTRDONE Interrupt Enable"]
    #[inline(always)]
    pub fn instrdone(&mut self) -> INSTRDONE_W {
        INSTRDONE_W { w: self }
    }
    #[doc = "Bit 1 - SEQDONE Interrupt Enable"]
    #[inline(always)]
    pub fn seqdone(&mut self) -> SEQDONE_W {
        SEQDONE_W { w: self }
    }
}
