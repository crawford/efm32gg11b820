#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EM23PERNORETAINCMD {
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
}
#[doc = r" Proxy"]
pub struct _ACMP0UNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP0UNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACMP1UNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP1UNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PCNT0UNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _PCNT0UNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PCNT1UNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _PCNT1UNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PCNT2UNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _PCNT2UNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _I2C0UNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C0UNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _I2C1UNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C1UNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DAC0UNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _DAC0UNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IDAC0UNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _IDAC0UNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADC0UNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0UNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LETIMER0UNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _LETIMER0UNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WDOG0UNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _WDOG0UNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WDOG1UNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _WDOG1UNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LESENSE0UNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _LESENSE0UNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CSENUNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _CSENUNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LEUART0UNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _LEUART0UNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LEUART1UNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _LEUART1UNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LCDUNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _LCDUNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LETIMER1UNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _LETIMER1UNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _I2C2UNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C2UNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADC1UNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC1UNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACMP2UNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP2UNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACMP3UNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP3UNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RTCUNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCUNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USBUNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _USBUNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Clears Status Bit of ACMP0 and Unlocks Access to It"]
    #[inline]
    pub fn acmp0unlock(&mut self) -> _ACMP0UNLOCKW {
        _ACMP0UNLOCKW { w: self }
    }
    #[doc = "Bit 1 - Clears Status Bit of ACMP1 and Unlocks Access to It"]
    #[inline]
    pub fn acmp1unlock(&mut self) -> _ACMP1UNLOCKW {
        _ACMP1UNLOCKW { w: self }
    }
    #[doc = "Bit 2 - Clears Status Bit of PCNT0 and Unlocks Access to It"]
    #[inline]
    pub fn pcnt0unlock(&mut self) -> _PCNT0UNLOCKW {
        _PCNT0UNLOCKW { w: self }
    }
    #[doc = "Bit 3 - Clears Status Bit of PCNT1 and Unlocks Access to It"]
    #[inline]
    pub fn pcnt1unlock(&mut self) -> _PCNT1UNLOCKW {
        _PCNT1UNLOCKW { w: self }
    }
    #[doc = "Bit 4 - Clears Status Bit of PCNT2 and Unlocks Access to It"]
    #[inline]
    pub fn pcnt2unlock(&mut self) -> _PCNT2UNLOCKW {
        _PCNT2UNLOCKW { w: self }
    }
    #[doc = "Bit 5 - Clears Status Bit of I2C0 and Unlocks Access to It"]
    #[inline]
    pub fn i2c0unlock(&mut self) -> _I2C0UNLOCKW {
        _I2C0UNLOCKW { w: self }
    }
    #[doc = "Bit 6 - Clears Status Bit of I2C1 and Unlocks Access to It"]
    #[inline]
    pub fn i2c1unlock(&mut self) -> _I2C1UNLOCKW {
        _I2C1UNLOCKW { w: self }
    }
    #[doc = "Bit 7 - Clears Status Bit of DAC0 and Unlocks Access to It"]
    #[inline]
    pub fn dac0unlock(&mut self) -> _DAC0UNLOCKW {
        _DAC0UNLOCKW { w: self }
    }
    #[doc = "Bit 8 - Clears Status Bit of IDAC0 and Unlocks Access to It"]
    #[inline]
    pub fn idac0unlock(&mut self) -> _IDAC0UNLOCKW {
        _IDAC0UNLOCKW { w: self }
    }
    #[doc = "Bit 9 - Clears Status Bit of ADC0 and Unlocks Access to It"]
    #[inline]
    pub fn adc0unlock(&mut self) -> _ADC0UNLOCKW {
        _ADC0UNLOCKW { w: self }
    }
    #[doc = "Bit 10 - Clears Status Bit of LETIMER0 and Unlocks Access to It"]
    #[inline]
    pub fn letimer0unlock(&mut self) -> _LETIMER0UNLOCKW {
        _LETIMER0UNLOCKW { w: self }
    }
    #[doc = "Bit 11 - Clears Status Bit of WDOG0 and Unlocks Access to It"]
    #[inline]
    pub fn wdog0unlock(&mut self) -> _WDOG0UNLOCKW {
        _WDOG0UNLOCKW { w: self }
    }
    #[doc = "Bit 12 - Clears Status Bit of WDOG1 and Unlocks Access to It"]
    #[inline]
    pub fn wdog1unlock(&mut self) -> _WDOG1UNLOCKW {
        _WDOG1UNLOCKW { w: self }
    }
    #[doc = "Bit 13 - Clears Status Bit of LESENSE0 and Unlocks Access to It"]
    #[inline]
    pub fn lesense0unlock(&mut self) -> _LESENSE0UNLOCKW {
        _LESENSE0UNLOCKW { w: self }
    }
    #[doc = "Bit 14 - Clears Status Bit of CSEN and Unlocks Access to It"]
    #[inline]
    pub fn csenunlock(&mut self) -> _CSENUNLOCKW {
        _CSENUNLOCKW { w: self }
    }
    #[doc = "Bit 15 - Clears Status Bit of LEUART0 and Unlocks Access to It"]
    #[inline]
    pub fn leuart0unlock(&mut self) -> _LEUART0UNLOCKW {
        _LEUART0UNLOCKW { w: self }
    }
    #[doc = "Bit 16 - Clears Status Bit of LEUART1 and Unlocks Access to It"]
    #[inline]
    pub fn leuart1unlock(&mut self) -> _LEUART1UNLOCKW {
        _LEUART1UNLOCKW { w: self }
    }
    #[doc = "Bit 17 - Clears Status Bit of LCD and Unlocks Access to It"]
    #[inline]
    pub fn lcdunlock(&mut self) -> _LCDUNLOCKW {
        _LCDUNLOCKW { w: self }
    }
    #[doc = "Bit 18 - Clears Status Bit of LETIMER1 and Unlocks Access to It"]
    #[inline]
    pub fn letimer1unlock(&mut self) -> _LETIMER1UNLOCKW {
        _LETIMER1UNLOCKW { w: self }
    }
    #[doc = "Bit 19 - Clears Status Bit of I2C2 and Unlocks Access to It"]
    #[inline]
    pub fn i2c2unlock(&mut self) -> _I2C2UNLOCKW {
        _I2C2UNLOCKW { w: self }
    }
    #[doc = "Bit 20 - Clears Status Bit of ADC1 and Unlocks Access to It"]
    #[inline]
    pub fn adc1unlock(&mut self) -> _ADC1UNLOCKW {
        _ADC1UNLOCKW { w: self }
    }
    #[doc = "Bit 21 - Clears Status Bit of ACMP2 and Unlocks Access to It"]
    #[inline]
    pub fn acmp2unlock(&mut self) -> _ACMP2UNLOCKW {
        _ACMP2UNLOCKW { w: self }
    }
    #[doc = "Bit 22 - Clears Status Bit of ACMP3 and Unlocks Access to It"]
    #[inline]
    pub fn acmp3unlock(&mut self) -> _ACMP3UNLOCKW {
        _ACMP3UNLOCKW { w: self }
    }
    #[doc = "Bit 23 - Clears Status Bit of RTC and Unlocks Access to It"]
    #[inline]
    pub fn rtcunlock(&mut self) -> _RTCUNLOCKW {
        _RTCUNLOCKW { w: self }
    }
    #[doc = "Bit 24 - Clears Status Bit of USB and Unlocks Access to It"]
    #[inline]
    pub fn usbunlock(&mut self) -> _USBUNLOCKW {
        _USBUNLOCKW { w: self }
    }
}
