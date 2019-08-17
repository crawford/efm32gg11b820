#[doc = "Reader of register WRITECTRL"]
pub type R = crate::R<u32, super::WRITECTRL>;
#[doc = "Writer for register WRITECTRL"]
pub type W = crate::W<u32, super::WRITECTRL>;
#[doc = "Register WRITECTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::WRITECTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WREN`"]
pub type WREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WREN`"]
pub struct WREN_W<'a> {
    w: &'a mut W,
}
impl<'a> WREN_W<'a> {
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
#[doc = "Reader of field `IRQERASEABORT`"]
pub type IRQERASEABORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRQERASEABORT`"]
pub struct IRQERASEABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQERASEABORT_W<'a> {
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
#[doc = "Reader of field `RWWEN`"]
pub type RWWEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RWWEN`"]
pub struct RWWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RWWEN_W<'a> {
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
    #[doc = "Bit 0 - Enable Write/Erase Controller"]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Abort Page Erase on Interrupt"]
    #[inline(always)]
    pub fn irqeraseabort(&self) -> IRQERASEABORT_R {
        IRQERASEABORT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Read-While-Write Enable"]
    #[inline(always)]
    pub fn rwwen(&self) -> RWWEN_R {
        RWWEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write/Erase Controller"]
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W {
        WREN_W { w: self }
    }
    #[doc = "Bit 1 - Abort Page Erase on Interrupt"]
    #[inline(always)]
    pub fn irqeraseabort(&mut self) -> IRQERASEABORT_W {
        IRQERASEABORT_W { w: self }
    }
    #[doc = "Bit 5 - Read-While-Write Enable"]
    #[inline(always)]
    pub fn rwwen(&mut self) -> RWWEN_W {
        RWWEN_W { w: self }
    }
}
