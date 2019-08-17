#[doc = "Reader of register WRPROTCTRL"]
pub type R = crate::R<u32, super::WRPROTCTRL>;
#[doc = "Writer for register WRPROTCTRL"]
pub type W = crate::W<u32, super::WRPROTCTRL>;
#[doc = "Register WRPROTCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::WRPROTCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INV`"]
pub type INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INV`"]
pub struct INV_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_W<'a> {
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
#[doc = "Reader of field `ENB`"]
pub type ENB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENB`"]
pub struct ENB_W<'a> {
    w: &'a mut W,
}
impl<'a> ENB_W<'a> {
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
    #[doc = "Bit 0 - Write Protection Inversion Bit"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write Protection Enable Bit"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection Inversion Bit"]
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W {
        INV_W { w: self }
    }
    #[doc = "Bit 1 - Write Protection Enable Bit"]
    #[inline(always)]
    pub fn enb(&mut self) -> ENB_W {
        ENB_W { w: self }
    }
}
