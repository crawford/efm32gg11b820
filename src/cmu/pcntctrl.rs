#[doc = "Reader of register PCNTCTRL"]
pub type R = crate::R<u32, super::PCNTCTRL>;
#[doc = "Writer for register PCNTCTRL"]
pub type W = crate::W<u32, super::PCNTCTRL>;
#[doc = "Register PCNTCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::PCNTCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCNT0CLKEN`"]
pub type PCNT0CLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT0CLKEN`"]
pub struct PCNT0CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT0CLKEN_W<'a> {
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
#[doc = "Reader of field `PCNT0CLKSEL`"]
pub type PCNT0CLKSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT0CLKSEL`"]
pub struct PCNT0CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT0CLKSEL_W<'a> {
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
#[doc = "Reader of field `PCNT1CLKEN`"]
pub type PCNT1CLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT1CLKEN`"]
pub struct PCNT1CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT1CLKEN_W<'a> {
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
#[doc = "Reader of field `PCNT1CLKSEL`"]
pub type PCNT1CLKSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT1CLKSEL`"]
pub struct PCNT1CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT1CLKSEL_W<'a> {
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
#[doc = "Reader of field `PCNT2CLKEN`"]
pub type PCNT2CLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT2CLKEN`"]
pub struct PCNT2CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT2CLKEN_W<'a> {
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
#[doc = "Reader of field `PCNT2CLKSEL`"]
pub type PCNT2CLKSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT2CLKSEL`"]
pub struct PCNT2CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT2CLKSEL_W<'a> {
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
    #[doc = "Bit 0 - PCNT0 Clock Enable"]
    #[inline(always)]
    pub fn pcnt0clken(&self) -> PCNT0CLKEN_R {
        PCNT0CLKEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PCNT0 Clock Select"]
    #[inline(always)]
    pub fn pcnt0clksel(&self) -> PCNT0CLKSEL_R {
        PCNT0CLKSEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PCNT1 Clock Enable"]
    #[inline(always)]
    pub fn pcnt1clken(&self) -> PCNT1CLKEN_R {
        PCNT1CLKEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PCNT1 Clock Select"]
    #[inline(always)]
    pub fn pcnt1clksel(&self) -> PCNT1CLKSEL_R {
        PCNT1CLKSEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PCNT2 Clock Enable"]
    #[inline(always)]
    pub fn pcnt2clken(&self) -> PCNT2CLKEN_R {
        PCNT2CLKEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PCNT2 Clock Select"]
    #[inline(always)]
    pub fn pcnt2clksel(&self) -> PCNT2CLKSEL_R {
        PCNT2CLKSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCNT0 Clock Enable"]
    #[inline(always)]
    pub fn pcnt0clken(&mut self) -> PCNT0CLKEN_W {
        PCNT0CLKEN_W { w: self }
    }
    #[doc = "Bit 1 - PCNT0 Clock Select"]
    #[inline(always)]
    pub fn pcnt0clksel(&mut self) -> PCNT0CLKSEL_W {
        PCNT0CLKSEL_W { w: self }
    }
    #[doc = "Bit 2 - PCNT1 Clock Enable"]
    #[inline(always)]
    pub fn pcnt1clken(&mut self) -> PCNT1CLKEN_W {
        PCNT1CLKEN_W { w: self }
    }
    #[doc = "Bit 3 - PCNT1 Clock Select"]
    #[inline(always)]
    pub fn pcnt1clksel(&mut self) -> PCNT1CLKSEL_W {
        PCNT1CLKSEL_W { w: self }
    }
    #[doc = "Bit 4 - PCNT2 Clock Enable"]
    #[inline(always)]
    pub fn pcnt2clken(&mut self) -> PCNT2CLKEN_W {
        PCNT2CLKEN_W { w: self }
    }
    #[doc = "Bit 5 - PCNT2 Clock Select"]
    #[inline(always)]
    pub fn pcnt2clksel(&mut self) -> PCNT2CLKSEL_W {
        PCNT2CLKSEL_W { w: self }
    }
}
