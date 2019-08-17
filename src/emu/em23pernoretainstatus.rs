#[doc = "Reader of register EM23PERNORETAINSTATUS"]
pub type R = crate::R<u32, super::EM23PERNORETAINSTATUS>;
#[doc = "Reader of field `ACMP0LOCKED`"]
pub type ACMP0LOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACMP1LOCKED`"]
pub type ACMP1LOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `PCNT0LOCKED`"]
pub type PCNT0LOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `PCNT1LOCKED`"]
pub type PCNT1LOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `PCNT2LOCKED`"]
pub type PCNT2LOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C0LOCKED`"]
pub type I2C0LOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C1LOCKED`"]
pub type I2C1LOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `DAC0LOCKED`"]
pub type DAC0LOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `IDAC0LOCKED`"]
pub type IDAC0LOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0LOCKED`"]
pub type ADC0LOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `LETIMER0LOCKED`"]
pub type LETIMER0LOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `WDOG0LOCKED`"]
pub type WDOG0LOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `WDOG1LOCKED`"]
pub type WDOG1LOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `LESENSE0LOCKED`"]
pub type LESENSE0LOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `CSENLOCKED`"]
pub type CSENLOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `LEUART0LOCKED`"]
pub type LEUART0LOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `LEUART1LOCKED`"]
pub type LEUART1LOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `LCDLOCKED`"]
pub type LCDLOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `LETIMER1LOCKED`"]
pub type LETIMER1LOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C2LOCKED`"]
pub type I2C2LOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1LOCKED`"]
pub type ADC1LOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACMP2LOCKED`"]
pub type ACMP2LOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACMP3LOCKED`"]
pub type ACMP3LOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTCLOCKED`"]
pub type RTCLOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `USBLOCKED`"]
pub type USBLOCKED_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Indicates If ACMP0 Powered Down During EM23"]
    #[inline(always)]
    pub fn acmp0locked(&self) -> ACMP0LOCKED_R {
        ACMP0LOCKED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates If ACMP1 Powered Down During EM23"]
    #[inline(always)]
    pub fn acmp1locked(&self) -> ACMP1LOCKED_R {
        ACMP1LOCKED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicates If PCNT0 Powered Down During EM23"]
    #[inline(always)]
    pub fn pcnt0locked(&self) -> PCNT0LOCKED_R {
        PCNT0LOCKED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Indicates If PCNT1 Powered Down During EM23"]
    #[inline(always)]
    pub fn pcnt1locked(&self) -> PCNT1LOCKED_R {
        PCNT1LOCKED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Indicates If PCNT2 Powered Down During EM23"]
    #[inline(always)]
    pub fn pcnt2locked(&self) -> PCNT2LOCKED_R {
        PCNT2LOCKED_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates If I2C0 Powered Down During EM23"]
    #[inline(always)]
    pub fn i2c0locked(&self) -> I2C0LOCKED_R {
        I2C0LOCKED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Indicates If I2C1 Powered Down During EM23"]
    #[inline(always)]
    pub fn i2c1locked(&self) -> I2C1LOCKED_R {
        I2C1LOCKED_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Indicates If DAC0 Powered Down During EM23"]
    #[inline(always)]
    pub fn dac0locked(&self) -> DAC0LOCKED_R {
        DAC0LOCKED_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Indicates If IDAC0 Powered Down During EM23"]
    #[inline(always)]
    pub fn idac0locked(&self) -> IDAC0LOCKED_R {
        IDAC0LOCKED_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Indicates If ADC0 Powered Down During EM23"]
    #[inline(always)]
    pub fn adc0locked(&self) -> ADC0LOCKED_R {
        ADC0LOCKED_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Indicates If LETIMER0 Powered Down During EM23"]
    #[inline(always)]
    pub fn letimer0locked(&self) -> LETIMER0LOCKED_R {
        LETIMER0LOCKED_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Indicates If WDOG0 Powered Down During EM23"]
    #[inline(always)]
    pub fn wdog0locked(&self) -> WDOG0LOCKED_R {
        WDOG0LOCKED_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Indicates If WDOG1 Powered Down During EM23"]
    #[inline(always)]
    pub fn wdog1locked(&self) -> WDOG1LOCKED_R {
        WDOG1LOCKED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Indicates If LESENSE0 Powered Down During EM23"]
    #[inline(always)]
    pub fn lesense0locked(&self) -> LESENSE0LOCKED_R {
        LESENSE0LOCKED_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Indicates If CSEN Powered Down During EM23"]
    #[inline(always)]
    pub fn csenlocked(&self) -> CSENLOCKED_R {
        CSENLOCKED_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Indicates If LEUART0 Powered Down During EM23"]
    #[inline(always)]
    pub fn leuart0locked(&self) -> LEUART0LOCKED_R {
        LEUART0LOCKED_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Indicates If LEUART1 Powered Down During EM23"]
    #[inline(always)]
    pub fn leuart1locked(&self) -> LEUART1LOCKED_R {
        LEUART1LOCKED_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Indicates If LCD Powered Down During EM23"]
    #[inline(always)]
    pub fn lcdlocked(&self) -> LCDLOCKED_R {
        LCDLOCKED_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Indicates If LETIMER1 Powered Down During EM23"]
    #[inline(always)]
    pub fn letimer1locked(&self) -> LETIMER1LOCKED_R {
        LETIMER1LOCKED_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Indicates If I2C2 Powered Down During EM23"]
    #[inline(always)]
    pub fn i2c2locked(&self) -> I2C2LOCKED_R {
        I2C2LOCKED_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Indicates If ADC1 Powered Down During EM23"]
    #[inline(always)]
    pub fn adc1locked(&self) -> ADC1LOCKED_R {
        ADC1LOCKED_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Indicates If ACMP2 Powered Down During EM23"]
    #[inline(always)]
    pub fn acmp2locked(&self) -> ACMP2LOCKED_R {
        ACMP2LOCKED_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Indicates If ACMP3 Powered Down During EM23"]
    #[inline(always)]
    pub fn acmp3locked(&self) -> ACMP3LOCKED_R {
        ACMP3LOCKED_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Indicates If RTC Powered Down During EM23"]
    #[inline(always)]
    pub fn rtclocked(&self) -> RTCLOCKED_R {
        RTCLOCKED_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Indicates If USB Powered Down During EM23"]
    #[inline(always)]
    pub fn usblocked(&self) -> USBLOCKED_R {
        USBLOCKED_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
