#[doc = "Reader of register LFACLKEN0"]
pub type R = crate::R<u32, super::LFACLKEN0>;
#[doc = "Writer for register LFACLKEN0"]
pub type W = crate::W<u32, super::LFACLKEN0>;
#[doc = "Register LFACLKEN0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LFACLKEN0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LETIMER0`"]
pub type LETIMER0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LETIMER0`"]
pub struct LETIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> LETIMER0_W<'a> {
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
#[doc = "Reader of field `LETIMER1`"]
pub type LETIMER1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LETIMER1`"]
pub struct LETIMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> LETIMER1_W<'a> {
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
#[doc = "Reader of field `LESENSE`"]
pub type LESENSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LESENSE`"]
pub struct LESENSE_W<'a> {
    w: &'a mut W,
}
impl<'a> LESENSE_W<'a> {
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
#[doc = "Reader of field `LCD`"]
pub type LCD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCD`"]
pub struct LCD_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_W<'a> {
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
#[doc = "Reader of field `RTC`"]
pub type RTC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC`"]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
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
    #[doc = "Bit 0 - Low Energy Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn letimer0(&self) -> LETIMER0_R {
        LETIMER0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Low Energy Timer 1 Clock Enable"]
    #[inline(always)]
    pub fn letimer1(&self) -> LETIMER1_R {
        LETIMER1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Low Energy Sensor Interface Clock Enable"]
    #[inline(always)]
    pub fn lesense(&self) -> LESENSE_R {
        LESENSE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Liquid Crystal Display Controller Clock Enable"]
    #[inline(always)]
    pub fn lcd(&self) -> LCD_R {
        LCD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Real-Time Counter Clock Enable"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Energy Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn letimer0(&mut self) -> LETIMER0_W {
        LETIMER0_W { w: self }
    }
    #[doc = "Bit 1 - Low Energy Timer 1 Clock Enable"]
    #[inline(always)]
    pub fn letimer1(&mut self) -> LETIMER1_W {
        LETIMER1_W { w: self }
    }
    #[doc = "Bit 2 - Low Energy Sensor Interface Clock Enable"]
    #[inline(always)]
    pub fn lesense(&mut self) -> LESENSE_W {
        LESENSE_W { w: self }
    }
    #[doc = "Bit 3 - Liquid Crystal Display Controller Clock Enable"]
    #[inline(always)]
    pub fn lcd(&mut self) -> LCD_W {
        LCD_W { w: self }
    }
    #[doc = "Bit 4 - Real-Time Counter Clock Enable"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
}
