#[doc = "Reader of register INSENSE"]
pub type R = crate::R<u32, super::INSENSE>;
#[doc = "Writer for register INSENSE"]
pub type W = crate::W<u32, super::INSENSE>;
#[doc = "Register INSENSE `reset()`'s with value 0x03"]
impl crate::ResetValue for super::INSENSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Reader of field `INT`"]
pub type INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT`"]
pub struct INT_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_W<'a> {
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
#[doc = "Reader of field `EM4WU`"]
pub type EM4WU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM4WU`"]
pub struct EM4WU_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4WU_W<'a> {
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
    #[doc = "Bit 0 - Interrupt Sense Enable"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EM4WU Interrupt Sense Enable"]
    #[inline(always)]
    pub fn em4wu(&self) -> EM4WU_R {
        EM4WU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Sense Enable"]
    #[inline(always)]
    pub fn int(&mut self) -> INT_W {
        INT_W { w: self }
    }
    #[doc = "Bit 1 - EM4WU Interrupt Sense Enable"]
    #[inline(always)]
    pub fn em4wu(&mut self) -> EM4WU_W {
        EM4WU_W { w: self }
    }
}
