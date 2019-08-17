#[doc = "Reader of register PPUPATD1"]
pub type R = crate::R<u32, super::PPUPATD1>;
#[doc = "Writer for register PPUPATD1"]
pub type W = crate::W<u32, super::PPUPATD1>;
#[doc = "Register PPUPATD1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PPUPATD1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCNT0`"]
pub type PCNT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT0`"]
pub struct PCNT0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT0_W<'a> {
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
#[doc = "Reader of field `PCNT1`"]
pub type PCNT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT1`"]
pub struct PCNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT1_W<'a> {
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
#[doc = "Reader of field `PCNT2`"]
pub type PCNT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCNT2`"]
pub struct PCNT2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT2_W<'a> {
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
#[doc = "Reader of field `QSPI0`"]
pub type QSPI0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QSPI0`"]
pub struct QSPI0_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPI0_W<'a> {
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
#[doc = "Reader of field `RMU`"]
pub type RMU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMU`"]
pub struct RMU_W<'a> {
    w: &'a mut W,
}
impl<'a> RMU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RTCC`"]
pub type RTCC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCC`"]
pub struct RTCC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCC_W<'a> {
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
#[doc = "Reader of field `SDIO`"]
pub type SDIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIO`"]
pub struct SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_W<'a> {
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
#[doc = "Reader of field `SMU`"]
pub type SMU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMU`"]
pub struct SMU_W<'a> {
    w: &'a mut W,
}
impl<'a> SMU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `UART0`"]
pub type UART0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART0`"]
pub struct UART0_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_W<'a> {
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
#[doc = "Reader of field `UART1`"]
pub type UART1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART1`"]
pub struct UART1_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `USB`"]
pub type USB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB`"]
pub struct USB_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `WDOG0`"]
pub type WDOG0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDOG0`"]
pub struct WDOG0_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `WDOG1`"]
pub type WDOG1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDOG1`"]
pub struct WDOG1_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `WTIMER0`"]
pub type WTIMER0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTIMER0`"]
pub struct WTIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> WTIMER0_W<'a> {
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
#[doc = "Reader of field `WTIMER1`"]
pub type WTIMER1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTIMER1`"]
pub struct WTIMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> WTIMER1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `WTIMER2`"]
pub type WTIMER2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTIMER2`"]
pub struct WTIMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> WTIMER2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `WTIMER3`"]
pub type WTIMER3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTIMER3`"]
pub struct WTIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> WTIMER3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Pulse Counter 0 access control bit"]
    #[inline(always)]
    pub fn pcnt0(&self) -> PCNT0_R {
        PCNT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pulse Counter 1 access control bit"]
    #[inline(always)]
    pub fn pcnt1(&self) -> PCNT1_R {
        PCNT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pulse Counter 2 access control bit"]
    #[inline(always)]
    pub fn pcnt2(&self) -> PCNT2_R {
        PCNT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Quad-SPI access control bit"]
    #[inline(always)]
    pub fn qspi0(&self) -> QSPI0_R {
        QSPI0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reset Management Unit access control bit"]
    #[inline(always)]
    pub fn rmu(&self) -> RMU_R {
        RMU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Real-Time Counter access control bit"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Real-Time Counter and Calendar access control bit"]
    #[inline(always)]
    pub fn rtcc(&self) -> RTCC_R {
        RTCC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SDIO Controller access control bit"]
    #[inline(always)]
    pub fn sdio(&self) -> SDIO_R {
        SDIO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Security Management Unit access control bit"]
    #[inline(always)]
    pub fn smu(&self) -> SMU_R {
        SMU_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Timer 0 access control bit"]
    #[inline(always)]
    pub fn timer0(&self) -> TIMER0_R {
        TIMER0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Timer 1 access control bit"]
    #[inline(always)]
    pub fn timer1(&self) -> TIMER1_R {
        TIMER1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Timer 2 access control bit"]
    #[inline(always)]
    pub fn timer2(&self) -> TIMER2_R {
        TIMER2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Timer 3 access control bit"]
    #[inline(always)]
    pub fn timer3(&self) -> TIMER3_R {
        TIMER3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Timer 4 access control bit"]
    #[inline(always)]
    pub fn timer4(&self) -> TIMER4_R {
        TIMER4_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Timer 5 access control bit"]
    #[inline(always)]
    pub fn timer5(&self) -> TIMER5_R {
        TIMER5_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Timer 6 access control bit"]
    #[inline(always)]
    pub fn timer6(&self) -> TIMER6_R {
        TIMER6_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - True Random Number Generator 0 access control bit"]
    #[inline(always)]
    pub fn trng0(&self) -> TRNG0_R {
        TRNG0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Universal Asynchronous Receiver/Transmitter 0 access control bit"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Universal Asynchronous Receiver/Transmitter 1 access control bit"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 access control bit"]
    #[inline(always)]
    pub fn usart0(&self) -> USART0_R {
        USART0_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 access control bit"]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Universal Synchronous/Asynchronous Receiver/Transmitter 2 access control bit"]
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit"]
    #[inline(always)]
    pub fn usart3(&self) -> USART3_R {
        USART3_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Universal Synchronous/Asynchronous Receiver/Transmitter 4 access control bit"]
    #[inline(always)]
    pub fn usart4(&self) -> USART4_R {
        USART4_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Universal Synchronous/Asynchronous Receiver/Transmitter 5 access control bit"]
    #[inline(always)]
    pub fn usart5(&self) -> USART5_R {
        USART5_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Universal Serial Bus Interface access control bit"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Watchdog access control bit"]
    #[inline(always)]
    pub fn wdog0(&self) -> WDOG0_R {
        WDOG0_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Watchdog access control bit"]
    #[inline(always)]
    pub fn wdog1(&self) -> WDOG1_R {
        WDOG1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Wide Timer 0 access control bit"]
    #[inline(always)]
    pub fn wtimer0(&self) -> WTIMER0_R {
        WTIMER0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Wide Timer 0 access control bit"]
    #[inline(always)]
    pub fn wtimer1(&self) -> WTIMER1_R {
        WTIMER1_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Wide Timer 2 access control bit"]
    #[inline(always)]
    pub fn wtimer2(&self) -> WTIMER2_R {
        WTIMER2_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Wide Timer 3 access control bit"]
    #[inline(always)]
    pub fn wtimer3(&self) -> WTIMER3_R {
        WTIMER3_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pulse Counter 0 access control bit"]
    #[inline(always)]
    pub fn pcnt0(&mut self) -> PCNT0_W {
        PCNT0_W { w: self }
    }
    #[doc = "Bit 1 - Pulse Counter 1 access control bit"]
    #[inline(always)]
    pub fn pcnt1(&mut self) -> PCNT1_W {
        PCNT1_W { w: self }
    }
    #[doc = "Bit 2 - Pulse Counter 2 access control bit"]
    #[inline(always)]
    pub fn pcnt2(&mut self) -> PCNT2_W {
        PCNT2_W { w: self }
    }
    #[doc = "Bit 3 - Quad-SPI access control bit"]
    #[inline(always)]
    pub fn qspi0(&mut self) -> QSPI0_W {
        QSPI0_W { w: self }
    }
    #[doc = "Bit 4 - Reset Management Unit access control bit"]
    #[inline(always)]
    pub fn rmu(&mut self) -> RMU_W {
        RMU_W { w: self }
    }
    #[doc = "Bit 5 - Real-Time Counter access control bit"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
    #[doc = "Bit 6 - Real-Time Counter and Calendar access control bit"]
    #[inline(always)]
    pub fn rtcc(&mut self) -> RTCC_W {
        RTCC_W { w: self }
    }
    #[doc = "Bit 7 - SDIO Controller access control bit"]
    #[inline(always)]
    pub fn sdio(&mut self) -> SDIO_W {
        SDIO_W { w: self }
    }
    #[doc = "Bit 8 - Security Management Unit access control bit"]
    #[inline(always)]
    pub fn smu(&mut self) -> SMU_W {
        SMU_W { w: self }
    }
    #[doc = "Bit 9 - Timer 0 access control bit"]
    #[inline(always)]
    pub fn timer0(&mut self) -> TIMER0_W {
        TIMER0_W { w: self }
    }
    #[doc = "Bit 10 - Timer 1 access control bit"]
    #[inline(always)]
    pub fn timer1(&mut self) -> TIMER1_W {
        TIMER1_W { w: self }
    }
    #[doc = "Bit 11 - Timer 2 access control bit"]
    #[inline(always)]
    pub fn timer2(&mut self) -> TIMER2_W {
        TIMER2_W { w: self }
    }
    #[doc = "Bit 12 - Timer 3 access control bit"]
    #[inline(always)]
    pub fn timer3(&mut self) -> TIMER3_W {
        TIMER3_W { w: self }
    }
    #[doc = "Bit 13 - Timer 4 access control bit"]
    #[inline(always)]
    pub fn timer4(&mut self) -> TIMER4_W {
        TIMER4_W { w: self }
    }
    #[doc = "Bit 14 - Timer 5 access control bit"]
    #[inline(always)]
    pub fn timer5(&mut self) -> TIMER5_W {
        TIMER5_W { w: self }
    }
    #[doc = "Bit 15 - Timer 6 access control bit"]
    #[inline(always)]
    pub fn timer6(&mut self) -> TIMER6_W {
        TIMER6_W { w: self }
    }
    #[doc = "Bit 16 - True Random Number Generator 0 access control bit"]
    #[inline(always)]
    pub fn trng0(&mut self) -> TRNG0_W {
        TRNG0_W { w: self }
    }
    #[doc = "Bit 17 - Universal Asynchronous Receiver/Transmitter 0 access control bit"]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART0_W {
        UART0_W { w: self }
    }
    #[doc = "Bit 18 - Universal Asynchronous Receiver/Transmitter 1 access control bit"]
    #[inline(always)]
    pub fn uart1(&mut self) -> UART1_W {
        UART1_W { w: self }
    }
    #[doc = "Bit 19 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 access control bit"]
    #[inline(always)]
    pub fn usart0(&mut self) -> USART0_W {
        USART0_W { w: self }
    }
    #[doc = "Bit 20 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 access control bit"]
    #[inline(always)]
    pub fn usart1(&mut self) -> USART1_W {
        USART1_W { w: self }
    }
    #[doc = "Bit 21 - Universal Synchronous/Asynchronous Receiver/Transmitter 2 access control bit"]
    #[inline(always)]
    pub fn usart2(&mut self) -> USART2_W {
        USART2_W { w: self }
    }
    #[doc = "Bit 22 - Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit"]
    #[inline(always)]
    pub fn usart3(&mut self) -> USART3_W {
        USART3_W { w: self }
    }
    #[doc = "Bit 23 - Universal Synchronous/Asynchronous Receiver/Transmitter 4 access control bit"]
    #[inline(always)]
    pub fn usart4(&mut self) -> USART4_W {
        USART4_W { w: self }
    }
    #[doc = "Bit 24 - Universal Synchronous/Asynchronous Receiver/Transmitter 5 access control bit"]
    #[inline(always)]
    pub fn usart5(&mut self) -> USART5_W {
        USART5_W { w: self }
    }
    #[doc = "Bit 25 - Universal Serial Bus Interface access control bit"]
    #[inline(always)]
    pub fn usb(&mut self) -> USB_W {
        USB_W { w: self }
    }
    #[doc = "Bit 26 - Watchdog access control bit"]
    #[inline(always)]
    pub fn wdog0(&mut self) -> WDOG0_W {
        WDOG0_W { w: self }
    }
    #[doc = "Bit 27 - Watchdog access control bit"]
    #[inline(always)]
    pub fn wdog1(&mut self) -> WDOG1_W {
        WDOG1_W { w: self }
    }
    #[doc = "Bit 28 - Wide Timer 0 access control bit"]
    #[inline(always)]
    pub fn wtimer0(&mut self) -> WTIMER0_W {
        WTIMER0_W { w: self }
    }
    #[doc = "Bit 29 - Wide Timer 0 access control bit"]
    #[inline(always)]
    pub fn wtimer1(&mut self) -> WTIMER1_W {
        WTIMER1_W { w: self }
    }
    #[doc = "Bit 30 - Wide Timer 2 access control bit"]
    #[inline(always)]
    pub fn wtimer2(&mut self) -> WTIMER2_W {
        WTIMER2_W { w: self }
    }
    #[doc = "Bit 31 - Wide Timer 3 access control bit"]
    #[inline(always)]
    pub fn wtimer3(&mut self) -> WTIMER3_W {
        WTIMER3_W { w: self }
    }
}
