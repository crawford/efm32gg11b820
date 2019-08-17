#[doc = "Reader of register HFPERCLKEN0"]
pub type R = crate::R<u32, super::HFPERCLKEN0>;
#[doc = "Writer for register HFPERCLKEN0"]
pub type W = crate::W<u32, super::HFPERCLKEN0>;
#[doc = "Register HFPERCLKEN0 `reset()`'s with value 0"]
impl crate::ResetValue for super::HFPERCLKEN0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMER0`"]
pub type TIMER0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER0`"]
pub struct TIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_W<'a> {
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
#[doc = "Reader of field `TIMER1`"]
pub type TIMER1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER1`"]
pub struct TIMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_W<'a> {
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
#[doc = "Reader of field `TIMER2`"]
pub type TIMER2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER2`"]
pub struct TIMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_W<'a> {
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
#[doc = "Reader of field `TIMER3`"]
pub type TIMER3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER3`"]
pub struct TIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER3_W<'a> {
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
#[doc = "Reader of field `TIMER4`"]
pub type TIMER4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER4`"]
pub struct TIMER4_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER4_W<'a> {
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
#[doc = "Reader of field `TIMER5`"]
pub type TIMER5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER5`"]
pub struct TIMER5_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER5_W<'a> {
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
#[doc = "Reader of field `TIMER6`"]
pub type TIMER6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER6`"]
pub struct TIMER6_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER6_W<'a> {
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
#[doc = "Reader of field `USART0`"]
pub type USART0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART0`"]
pub struct USART0_W<'a> {
    w: &'a mut W,
}
impl<'a> USART0_W<'a> {
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
#[doc = "Reader of field `USART1`"]
pub type USART1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART1`"]
pub struct USART1_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1_W<'a> {
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
#[doc = "Reader of field `USART2`"]
pub type USART2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART2`"]
pub struct USART2_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2_W<'a> {
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
#[doc = "Reader of field `USART3`"]
pub type USART3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART3`"]
pub struct USART3_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3_W<'a> {
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
#[doc = "Reader of field `USART4`"]
pub type USART4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART4`"]
pub struct USART4_W<'a> {
    w: &'a mut W,
}
impl<'a> USART4_W<'a> {
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
#[doc = "Reader of field `USART5`"]
pub type USART5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART5`"]
pub struct USART5_W<'a> {
    w: &'a mut W,
}
impl<'a> USART5_W<'a> {
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
#[doc = "Reader of field `ACMP0`"]
pub type ACMP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACMP0`"]
pub struct ACMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP0_W<'a> {
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
#[doc = "Reader of field `ACMP1`"]
pub type ACMP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACMP1`"]
pub struct ACMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP1_W<'a> {
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
#[doc = "Reader of field `ACMP2`"]
pub type ACMP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACMP2`"]
pub struct ACMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP2_W<'a> {
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
#[doc = "Reader of field `ACMP3`"]
pub type ACMP3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACMP3`"]
pub struct ACMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP3_W<'a> {
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
#[doc = "Reader of field `I2C0`"]
pub type I2C0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C0`"]
pub struct I2C0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_W<'a> {
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
#[doc = "Reader of field `I2C1`"]
pub type I2C1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1`"]
pub struct I2C1_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_W<'a> {
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
#[doc = "Reader of field `I2C2`"]
pub type I2C2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C2`"]
pub struct I2C2_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_W<'a> {
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
#[doc = "Reader of field `ADC0`"]
pub type ADC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC0`"]
pub struct ADC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_W<'a> {
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
#[doc = "Reader of field `ADC1`"]
pub type ADC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC1`"]
pub struct ADC1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1_W<'a> {
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
#[doc = "Reader of field `CRYOTIMER`"]
pub type CRYOTIMER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRYOTIMER`"]
pub struct CRYOTIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYOTIMER_W<'a> {
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
#[doc = "Reader of field `IDAC0`"]
pub type IDAC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDAC0`"]
pub struct IDAC0_W<'a> {
    w: &'a mut W,
}
impl<'a> IDAC0_W<'a> {
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
#[doc = "Reader of field `TRNG0`"]
pub type TRNG0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRNG0`"]
pub struct TRNG0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRNG0_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn timer0(&self) -> TIMER0_R {
        TIMER0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer 1 Clock Enable"]
    #[inline(always)]
    pub fn timer1(&self) -> TIMER1_R {
        TIMER1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer 2 Clock Enable"]
    #[inline(always)]
    pub fn timer2(&self) -> TIMER2_R {
        TIMER2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timer 3 Clock Enable"]
    #[inline(always)]
    pub fn timer3(&self) -> TIMER3_R {
        TIMER3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timer 4 Clock Enable"]
    #[inline(always)]
    pub fn timer4(&self) -> TIMER4_R {
        TIMER4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer 5 Clock Enable"]
    #[inline(always)]
    pub fn timer5(&self) -> TIMER5_R {
        TIMER5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Timer 6 Clock Enable"]
    #[inline(always)]
    pub fn timer6(&self) -> TIMER6_R {
        TIMER6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
    #[inline(always)]
    pub fn usart0(&self) -> USART0_R {
        USART0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Universal Synchronous/Asynchronous Receiver/Transmitter 2 Clock Enable"]
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Universal Synchronous/Asynchronous Receiver/Transmitter 3 Clock Enable"]
    #[inline(always)]
    pub fn usart3(&self) -> USART3_R {
        USART3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Universal Synchronous/Asynchronous Receiver/Transmitter 4 Clock Enable"]
    #[inline(always)]
    pub fn usart4(&self) -> USART4_R {
        USART4_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Universal Synchronous/Asynchronous Receiver/Transmitter 5 Clock Enable"]
    #[inline(always)]
    pub fn usart5(&self) -> USART5_R {
        USART5_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Analog Comparator 0 Clock Enable"]
    #[inline(always)]
    pub fn acmp0(&self) -> ACMP0_R {
        ACMP0_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Analog Comparator 1 Clock Enable"]
    #[inline(always)]
    pub fn acmp1(&self) -> ACMP1_R {
        ACMP1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Analog Comparator 1 Clock Enable"]
    #[inline(always)]
    pub fn acmp2(&self) -> ACMP2_R {
        ACMP2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Analog Comparator 3 Clock Enable"]
    #[inline(always)]
    pub fn acmp3(&self) -> ACMP3_R {
        ACMP3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - I2C 0 Clock Enable"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - I2C 1 Clock Enable"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - I2C 2 Clock Enable"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Analog to Digital Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn adc0(&self) -> ADC0_R {
        ADC0_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Analog to Digital Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn adc1(&self) -> ADC1_R {
        ADC1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - CryoTimer Clock Enable"]
    #[inline(always)]
    pub fn cryotimer(&self) -> CRYOTIMER_R {
        CRYOTIMER_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Current Digital to Analog Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn idac0(&self) -> IDAC0_R {
        IDAC0_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - True Random Number Generator 0 Clock Enable"]
    #[inline(always)]
    pub fn trng0(&self) -> TRNG0_R {
        TRNG0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn timer0(&mut self) -> TIMER0_W {
        TIMER0_W { w: self }
    }
    #[doc = "Bit 1 - Timer 1 Clock Enable"]
    #[inline(always)]
    pub fn timer1(&mut self) -> TIMER1_W {
        TIMER1_W { w: self }
    }
    #[doc = "Bit 2 - Timer 2 Clock Enable"]
    #[inline(always)]
    pub fn timer2(&mut self) -> TIMER2_W {
        TIMER2_W { w: self }
    }
    #[doc = "Bit 3 - Timer 3 Clock Enable"]
    #[inline(always)]
    pub fn timer3(&mut self) -> TIMER3_W {
        TIMER3_W { w: self }
    }
    #[doc = "Bit 4 - Timer 4 Clock Enable"]
    #[inline(always)]
    pub fn timer4(&mut self) -> TIMER4_W {
        TIMER4_W { w: self }
    }
    #[doc = "Bit 5 - Timer 5 Clock Enable"]
    #[inline(always)]
    pub fn timer5(&mut self) -> TIMER5_W {
        TIMER5_W { w: self }
    }
    #[doc = "Bit 6 - Timer 6 Clock Enable"]
    #[inline(always)]
    pub fn timer6(&mut self) -> TIMER6_W {
        TIMER6_W { w: self }
    }
    #[doc = "Bit 7 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
    #[inline(always)]
    pub fn usart0(&mut self) -> USART0_W {
        USART0_W { w: self }
    }
    #[doc = "Bit 8 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
    #[inline(always)]
    pub fn usart1(&mut self) -> USART1_W {
        USART1_W { w: self }
    }
    #[doc = "Bit 9 - Universal Synchronous/Asynchronous Receiver/Transmitter 2 Clock Enable"]
    #[inline(always)]
    pub fn usart2(&mut self) -> USART2_W {
        USART2_W { w: self }
    }
    #[doc = "Bit 10 - Universal Synchronous/Asynchronous Receiver/Transmitter 3 Clock Enable"]
    #[inline(always)]
    pub fn usart3(&mut self) -> USART3_W {
        USART3_W { w: self }
    }
    #[doc = "Bit 11 - Universal Synchronous/Asynchronous Receiver/Transmitter 4 Clock Enable"]
    #[inline(always)]
    pub fn usart4(&mut self) -> USART4_W {
        USART4_W { w: self }
    }
    #[doc = "Bit 12 - Universal Synchronous/Asynchronous Receiver/Transmitter 5 Clock Enable"]
    #[inline(always)]
    pub fn usart5(&mut self) -> USART5_W {
        USART5_W { w: self }
    }
    #[doc = "Bit 13 - Analog Comparator 0 Clock Enable"]
    #[inline(always)]
    pub fn acmp0(&mut self) -> ACMP0_W {
        ACMP0_W { w: self }
    }
    #[doc = "Bit 14 - Analog Comparator 1 Clock Enable"]
    #[inline(always)]
    pub fn acmp1(&mut self) -> ACMP1_W {
        ACMP1_W { w: self }
    }
    #[doc = "Bit 15 - Analog Comparator 1 Clock Enable"]
    #[inline(always)]
    pub fn acmp2(&mut self) -> ACMP2_W {
        ACMP2_W { w: self }
    }
    #[doc = "Bit 16 - Analog Comparator 3 Clock Enable"]
    #[inline(always)]
    pub fn acmp3(&mut self) -> ACMP3_W {
        ACMP3_W { w: self }
    }
    #[doc = "Bit 17 - I2C 0 Clock Enable"]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2C0_W {
        I2C0_W { w: self }
    }
    #[doc = "Bit 18 - I2C 1 Clock Enable"]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2C1_W {
        I2C1_W { w: self }
    }
    #[doc = "Bit 19 - I2C 2 Clock Enable"]
    #[inline(always)]
    pub fn i2c2(&mut self) -> I2C2_W {
        I2C2_W { w: self }
    }
    #[doc = "Bit 20 - Analog to Digital Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn adc0(&mut self) -> ADC0_W {
        ADC0_W { w: self }
    }
    #[doc = "Bit 21 - Analog to Digital Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn adc1(&mut self) -> ADC1_W {
        ADC1_W { w: self }
    }
    #[doc = "Bit 22 - CryoTimer Clock Enable"]
    #[inline(always)]
    pub fn cryotimer(&mut self) -> CRYOTIMER_W {
        CRYOTIMER_W { w: self }
    }
    #[doc = "Bit 23 - Current Digital to Analog Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn idac0(&mut self) -> IDAC0_W {
        IDAC0_W { w: self }
    }
    #[doc = "Bit 24 - True Random Number Generator 0 Clock Enable"]
    #[inline(always)]
    pub fn trng0(&mut self) -> TRNG0_W {
        TRNG0_W { w: self }
    }
}
