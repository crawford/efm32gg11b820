#[doc = "Writer for register EM23PERNORETAINCMD"]
pub type W = crate::W<u32, super::EM23PERNORETAINCMD>;
#[doc = "Register EM23PERNORETAINCMD `reset()`'s with value 0"]
impl crate::ResetValue for super::EM23PERNORETAINCMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ACMP0UNLOCK`"]
pub struct ACMP0UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP0UNLOCK_W<'a> {
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
#[doc = "Write proxy for field `ACMP1UNLOCK`"]
pub struct ACMP1UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP1UNLOCK_W<'a> {
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
#[doc = "Write proxy for field `PCNT0UNLOCK`"]
pub struct PCNT0UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT0UNLOCK_W<'a> {
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
#[doc = "Write proxy for field `PCNT1UNLOCK`"]
pub struct PCNT1UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT1UNLOCK_W<'a> {
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
#[doc = "Write proxy for field `PCNT2UNLOCK`"]
pub struct PCNT2UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT2UNLOCK_W<'a> {
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
#[doc = "Write proxy for field `I2C0UNLOCK`"]
pub struct I2C0UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0UNLOCK_W<'a> {
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
#[doc = "Write proxy for field `I2C1UNLOCK`"]
pub struct I2C1UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1UNLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Write proxy for field `DAC0UNLOCK`"]
pub struct DAC0UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC0UNLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Write proxy for field `IDAC0UNLOCK`"]
pub struct IDAC0UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> IDAC0UNLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `ADC0UNLOCK`"]
pub struct ADC0UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0UNLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Write proxy for field `LETIMER0UNLOCK`"]
pub struct LETIMER0UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LETIMER0UNLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Write proxy for field `WDOG0UNLOCK`"]
pub struct WDOG0UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG0UNLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Write proxy for field `WDOG1UNLOCK`"]
pub struct WDOG1UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG1UNLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Write proxy for field `LESENSE0UNLOCK`"]
pub struct LESENSE0UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LESENSE0UNLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Write proxy for field `CSENUNLOCK`"]
pub struct CSENUNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> CSENUNLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Write proxy for field `LEUART0UNLOCK`"]
pub struct LEUART0UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LEUART0UNLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Write proxy for field `LEUART1UNLOCK`"]
pub struct LEUART1UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LEUART1UNLOCK_W<'a> {
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
#[doc = "Write proxy for field `LCDUNLOCK`"]
pub struct LCDUNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDUNLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Write proxy for field `LETIMER1UNLOCK`"]
pub struct LETIMER1UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LETIMER1UNLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Write proxy for field `I2C2UNLOCK`"]
pub struct I2C2UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2UNLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Write proxy for field `ADC1UNLOCK`"]
pub struct ADC1UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1UNLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Write proxy for field `ACMP2UNLOCK`"]
pub struct ACMP2UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP2UNLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Write proxy for field `ACMP3UNLOCK`"]
pub struct ACMP3UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP3UNLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Write proxy for field `RTCUNLOCK`"]
pub struct RTCUNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCUNLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Write proxy for field `USBUNLOCK`"]
pub struct USBUNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> USBUNLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Clears Status Bit of ACMP0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn acmp0unlock(&mut self) -> ACMP0UNLOCK_W {
        ACMP0UNLOCK_W { w: self }
    }
    #[doc = "Bit 1 - Clears Status Bit of ACMP1 and Unlocks Access to It"]
    #[inline(always)]
    pub fn acmp1unlock(&mut self) -> ACMP1UNLOCK_W {
        ACMP1UNLOCK_W { w: self }
    }
    #[doc = "Bit 2 - Clears Status Bit of PCNT0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn pcnt0unlock(&mut self) -> PCNT0UNLOCK_W {
        PCNT0UNLOCK_W { w: self }
    }
    #[doc = "Bit 3 - Clears Status Bit of PCNT1 and Unlocks Access to It"]
    #[inline(always)]
    pub fn pcnt1unlock(&mut self) -> PCNT1UNLOCK_W {
        PCNT1UNLOCK_W { w: self }
    }
    #[doc = "Bit 4 - Clears Status Bit of PCNT2 and Unlocks Access to It"]
    #[inline(always)]
    pub fn pcnt2unlock(&mut self) -> PCNT2UNLOCK_W {
        PCNT2UNLOCK_W { w: self }
    }
    #[doc = "Bit 5 - Clears Status Bit of I2C0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn i2c0unlock(&mut self) -> I2C0UNLOCK_W {
        I2C0UNLOCK_W { w: self }
    }
    #[doc = "Bit 6 - Clears Status Bit of I2C1 and Unlocks Access to It"]
    #[inline(always)]
    pub fn i2c1unlock(&mut self) -> I2C1UNLOCK_W {
        I2C1UNLOCK_W { w: self }
    }
    #[doc = "Bit 7 - Clears Status Bit of DAC0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn dac0unlock(&mut self) -> DAC0UNLOCK_W {
        DAC0UNLOCK_W { w: self }
    }
    #[doc = "Bit 8 - Clears Status Bit of IDAC0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn idac0unlock(&mut self) -> IDAC0UNLOCK_W {
        IDAC0UNLOCK_W { w: self }
    }
    #[doc = "Bit 9 - Clears Status Bit of ADC0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn adc0unlock(&mut self) -> ADC0UNLOCK_W {
        ADC0UNLOCK_W { w: self }
    }
    #[doc = "Bit 10 - Clears Status Bit of LETIMER0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn letimer0unlock(&mut self) -> LETIMER0UNLOCK_W {
        LETIMER0UNLOCK_W { w: self }
    }
    #[doc = "Bit 11 - Clears Status Bit of WDOG0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn wdog0unlock(&mut self) -> WDOG0UNLOCK_W {
        WDOG0UNLOCK_W { w: self }
    }
    #[doc = "Bit 12 - Clears Status Bit of WDOG1 and Unlocks Access to It"]
    #[inline(always)]
    pub fn wdog1unlock(&mut self) -> WDOG1UNLOCK_W {
        WDOG1UNLOCK_W { w: self }
    }
    #[doc = "Bit 13 - Clears Status Bit of LESENSE0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn lesense0unlock(&mut self) -> LESENSE0UNLOCK_W {
        LESENSE0UNLOCK_W { w: self }
    }
    #[doc = "Bit 14 - Clears Status Bit of CSEN and Unlocks Access to It"]
    #[inline(always)]
    pub fn csenunlock(&mut self) -> CSENUNLOCK_W {
        CSENUNLOCK_W { w: self }
    }
    #[doc = "Bit 15 - Clears Status Bit of LEUART0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn leuart0unlock(&mut self) -> LEUART0UNLOCK_W {
        LEUART0UNLOCK_W { w: self }
    }
    #[doc = "Bit 16 - Clears Status Bit of LEUART1 and Unlocks Access to It"]
    #[inline(always)]
    pub fn leuart1unlock(&mut self) -> LEUART1UNLOCK_W {
        LEUART1UNLOCK_W { w: self }
    }
    #[doc = "Bit 17 - Clears Status Bit of LCD and Unlocks Access to It"]
    #[inline(always)]
    pub fn lcdunlock(&mut self) -> LCDUNLOCK_W {
        LCDUNLOCK_W { w: self }
    }
    #[doc = "Bit 18 - Clears Status Bit of LETIMER1 and Unlocks Access to It"]
    #[inline(always)]
    pub fn letimer1unlock(&mut self) -> LETIMER1UNLOCK_W {
        LETIMER1UNLOCK_W { w: self }
    }
    #[doc = "Bit 19 - Clears Status Bit of I2C2 and Unlocks Access to It"]
    #[inline(always)]
    pub fn i2c2unlock(&mut self) -> I2C2UNLOCK_W {
        I2C2UNLOCK_W { w: self }
    }
    #[doc = "Bit 20 - Clears Status Bit of ADC1 and Unlocks Access to It"]
    #[inline(always)]
    pub fn adc1unlock(&mut self) -> ADC1UNLOCK_W {
        ADC1UNLOCK_W { w: self }
    }
    #[doc = "Bit 21 - Clears Status Bit of ACMP2 and Unlocks Access to It"]
    #[inline(always)]
    pub fn acmp2unlock(&mut self) -> ACMP2UNLOCK_W {
        ACMP2UNLOCK_W { w: self }
    }
    #[doc = "Bit 22 - Clears Status Bit of ACMP3 and Unlocks Access to It"]
    #[inline(always)]
    pub fn acmp3unlock(&mut self) -> ACMP3UNLOCK_W {
        ACMP3UNLOCK_W { w: self }
    }
    #[doc = "Bit 23 - Clears Status Bit of RTC and Unlocks Access to It"]
    #[inline(always)]
    pub fn rtcunlock(&mut self) -> RTCUNLOCK_W {
        RTCUNLOCK_W { w: self }
    }
    #[doc = "Bit 24 - Clears Status Bit of USB and Unlocks Access to It"]
    #[inline(always)]
    pub fn usbunlock(&mut self) -> USBUNLOCK_W {
        USBUNLOCK_W { w: self }
    }
}
