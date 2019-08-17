#[doc = "Reader of register LFBCLKEN0"]
pub type R = crate::R<u32, super::LFBCLKEN0>;
#[doc = "Writer for register LFBCLKEN0"]
pub type W = crate::W<u32, super::LFBCLKEN0>;
#[doc = "Register LFBCLKEN0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LFBCLKEN0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEUART0`"]
pub type LEUART0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEUART0`"]
pub struct LEUART0_W<'a> {
    w: &'a mut W,
}
impl<'a> LEUART0_W<'a> {
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
#[doc = "Reader of field `LEUART1`"]
pub type LEUART1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEUART1`"]
pub struct LEUART1_W<'a> {
    w: &'a mut W,
}
impl<'a> LEUART1_W<'a> {
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
#[doc = "Reader of field `SYSTICK`"]
pub type SYSTICK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTICK`"]
pub struct SYSTICK_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTICK_W<'a> {
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
#[doc = "Reader of field `CSEN`"]
pub type CSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSEN`"]
pub struct CSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Low Energy UART 0 Clock Enable"]
    #[inline(always)]
    pub fn leuart0(&self) -> LEUART0_R {
        LEUART0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Low Energy UART 1 Clock Enable"]
    #[inline(always)]
    pub fn leuart1(&self) -> LEUART1_R {
        LEUART1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clock Enable"]
    #[inline(always)]
    pub fn systick(&self) -> SYSTICK_R {
        SYSTICK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Capacitive touch sense module Clock Enable"]
    #[inline(always)]
    pub fn csen(&self) -> CSEN_R {
        CSEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Energy UART 0 Clock Enable"]
    #[inline(always)]
    pub fn leuart0(&mut self) -> LEUART0_W {
        LEUART0_W { w: self }
    }
    #[doc = "Bit 1 - Low Energy UART 1 Clock Enable"]
    #[inline(always)]
    pub fn leuart1(&mut self) -> LEUART1_W {
        LEUART1_W { w: self }
    }
    #[doc = "Bit 2 - Clock Enable"]
    #[inline(always)]
    pub fn systick(&mut self) -> SYSTICK_W {
        SYSTICK_W { w: self }
    }
    #[doc = "Bit 3 - Capacitive touch sense module Clock Enable"]
    #[inline(always)]
    pub fn csen(&mut self) -> CSEN_W {
        CSEN_W { w: self }
    }
}
