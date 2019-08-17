#[doc = "Reader of register ROUTEPEN"]
pub type R = crate::R<u32, super::ROUTEPEN>;
#[doc = "Writer for register ROUTEPEN"]
pub type W = crate::W<u32, super::ROUTEPEN>;
#[doc = "Register ROUTEPEN `reset()`'s with value 0"]
impl crate::ResetValue for super::ROUTEPEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLKOUT0PEN`"]
pub type CLKOUT0PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKOUT0PEN`"]
pub struct CLKOUT0PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUT0PEN_W<'a> {
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
#[doc = "Reader of field `CLKOUT1PEN`"]
pub type CLKOUT1PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKOUT1PEN`"]
pub struct CLKOUT1PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUT1PEN_W<'a> {
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
#[doc = "Reader of field `CLKOUT2PEN`"]
pub type CLKOUT2PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKOUT2PEN`"]
pub struct CLKOUT2PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUT2PEN_W<'a> {
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
#[doc = "Reader of field `CLKIN0PEN`"]
pub type CLKIN0PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKIN0PEN`"]
pub struct CLKIN0PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKIN0PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CLKOUT0 Pin Enable"]
    #[inline(always)]
    pub fn clkout0pen(&self) -> CLKOUT0PEN_R {
        CLKOUT0PEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CLKOUT1 Pin Enable"]
    #[inline(always)]
    pub fn clkout1pen(&self) -> CLKOUT1PEN_R {
        CLKOUT1PEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CLKOUT2 Pin Enable"]
    #[inline(always)]
    pub fn clkout2pen(&self) -> CLKOUT2PEN_R {
        CLKOUT2PEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 28 - CLKIN0 Pin Enable"]
    #[inline(always)]
    pub fn clkin0pen(&self) -> CLKIN0PEN_R {
        CLKIN0PEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CLKOUT0 Pin Enable"]
    #[inline(always)]
    pub fn clkout0pen(&mut self) -> CLKOUT0PEN_W {
        CLKOUT0PEN_W { w: self }
    }
    #[doc = "Bit 1 - CLKOUT1 Pin Enable"]
    #[inline(always)]
    pub fn clkout1pen(&mut self) -> CLKOUT1PEN_W {
        CLKOUT1PEN_W { w: self }
    }
    #[doc = "Bit 2 - CLKOUT2 Pin Enable"]
    #[inline(always)]
    pub fn clkout2pen(&mut self) -> CLKOUT2PEN_W {
        CLKOUT2PEN_W { w: self }
    }
    #[doc = "Bit 28 - CLKIN0 Pin Enable"]
    #[inline(always)]
    pub fn clkin0pen(&mut self) -> CLKIN0PEN_W {
        CLKIN0PEN_W { w: self }
    }
}
