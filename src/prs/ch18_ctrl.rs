#[doc = "Reader of register CH18_CTRL"]
pub type R = crate::R<u32, super::CH18_CTRL>;
#[doc = "Writer for register CH18_CTRL"]
pub type W = crate::W<u32, super::CH18_CTRL>;
#[doc = "Register CH18_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CH18_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SIGSEL`"]
pub type SIGSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIGSEL`"]
pub struct SIGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOURCESEL_A {
    #[doc = "0: No source selected"]
    NONE,
    #[doc = "1: Peripheral Reflex System"]
    PRSL,
    #[doc = "2: Peripheral Reflex System"]
    PRS,
    #[doc = "3: Peripheral Reflex System"]
    PRSH,
    #[doc = "4: Analog Comparator 0"]
    ACMP0,
    #[doc = "5: Analog Comparator 1"]
    ACMP1,
    #[doc = "6: Analog to Digital Converter 0"]
    ADC0,
    #[doc = "7: Real-Time Counter"]
    RTC,
    #[doc = "8: Real-Time Counter and Calendar"]
    RTCC,
    #[doc = "9: General purpose Input/Output"]
    GPIOL,
    #[doc = "10: General purpose Input/Output"]
    GPIOH,
    #[doc = "11: Low Energy Timer 0"]
    LETIMER0,
    #[doc = "12: Low Energy Timer 1"]
    LETIMER1,
    #[doc = "13: Pulse Counter 0"]
    PCNT0,
    #[doc = "14: Pulse Counter 1"]
    PCNT1,
    #[doc = "15: Pulse Counter 2"]
    PCNT2,
    #[doc = "16: CryoTimer"]
    CRYOTIMER,
    #[doc = "17: Clock Management Unit"]
    CMU,
    #[doc = "23: Digital to Analog Converter 0"]
    VDAC0,
    #[doc = "24: Low Energy Sensor Interface"]
    LESENSEL,
    #[doc = "25: Low Energy Sensor Interface"]
    LESENSEH,
    #[doc = "26: Low Energy Sensor Interface"]
    LESENSED,
    #[doc = "27: Low Energy Sensor Interface"]
    LESENSE,
    #[doc = "28: Analog Comparator 1"]
    ACMP2,
    #[doc = "29: Analog Comparator 3"]
    ACMP3,
    #[doc = "30: Analog to Digital Converter 0"]
    ADC1,
    #[doc = "48: Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    USART0,
    #[doc = "49: Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    USART1,
    #[doc = "50: Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    USART2,
    #[doc = "51: Universal Synchronous/Asynchronous Receiver/Transmitter 3"]
    USART3,
    #[doc = "52: Universal Synchronous/Asynchronous Receiver/Transmitter 4"]
    USART4,
    #[doc = "53: Universal Synchronous/Asynchronous Receiver/Transmitter 5"]
    USART5,
    #[doc = "54: Universal Asynchronous Receiver/Transmitter 0"]
    UART0,
    #[doc = "55: Universal Asynchronous Receiver/Transmitter 1"]
    UART1,
    #[doc = "60: Timer 0"]
    TIMER0,
    #[doc = "61: Timer 1"]
    TIMER1,
    #[doc = "62: Timer 2"]
    TIMER2,
    #[doc = "64: Universal Serial Bus Interface"]
    USB,
    #[doc = "67: `1000011`"]
    CM4,
    #[doc = "80: Timer 3"]
    TIMER3,
    #[doc = "82: Wide Timer 0"]
    WTIMER0,
    #[doc = "83: Wide Timer 0"]
    WTIMER1,
    #[doc = "84: Wide Timer 2"]
    WTIMER2,
    #[doc = "85: Wide Timer 3"]
    WTIMER3,
    #[doc = "98: Timer 4"]
    TIMER4,
    #[doc = "99: Timer 5"]
    TIMER5,
    #[doc = "100: Timer 6"]
    TIMER6,
}
impl From<SOURCESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SOURCESEL_A) -> Self {
        match variant {
            SOURCESEL_A::NONE => 0,
            SOURCESEL_A::PRSL => 1,
            SOURCESEL_A::PRS => 2,
            SOURCESEL_A::PRSH => 3,
            SOURCESEL_A::ACMP0 => 4,
            SOURCESEL_A::ACMP1 => 5,
            SOURCESEL_A::ADC0 => 6,
            SOURCESEL_A::RTC => 7,
            SOURCESEL_A::RTCC => 8,
            SOURCESEL_A::GPIOL => 9,
            SOURCESEL_A::GPIOH => 10,
            SOURCESEL_A::LETIMER0 => 11,
            SOURCESEL_A::LETIMER1 => 12,
            SOURCESEL_A::PCNT0 => 13,
            SOURCESEL_A::PCNT1 => 14,
            SOURCESEL_A::PCNT2 => 15,
            SOURCESEL_A::CRYOTIMER => 16,
            SOURCESEL_A::CMU => 17,
            SOURCESEL_A::VDAC0 => 23,
            SOURCESEL_A::LESENSEL => 24,
            SOURCESEL_A::LESENSEH => 25,
            SOURCESEL_A::LESENSED => 26,
            SOURCESEL_A::LESENSE => 27,
            SOURCESEL_A::ACMP2 => 28,
            SOURCESEL_A::ACMP3 => 29,
            SOURCESEL_A::ADC1 => 30,
            SOURCESEL_A::USART0 => 48,
            SOURCESEL_A::USART1 => 49,
            SOURCESEL_A::USART2 => 50,
            SOURCESEL_A::USART3 => 51,
            SOURCESEL_A::USART4 => 52,
            SOURCESEL_A::USART5 => 53,
            SOURCESEL_A::UART0 => 54,
            SOURCESEL_A::UART1 => 55,
            SOURCESEL_A::TIMER0 => 60,
            SOURCESEL_A::TIMER1 => 61,
            SOURCESEL_A::TIMER2 => 62,
            SOURCESEL_A::USB => 64,
            SOURCESEL_A::CM4 => 67,
            SOURCESEL_A::TIMER3 => 80,
            SOURCESEL_A::WTIMER0 => 82,
            SOURCESEL_A::WTIMER1 => 83,
            SOURCESEL_A::WTIMER2 => 84,
            SOURCESEL_A::WTIMER3 => 85,
            SOURCESEL_A::TIMER4 => 98,
            SOURCESEL_A::TIMER5 => 99,
            SOURCESEL_A::TIMER6 => 100,
        }
    }
}
#[doc = "Reader of field `SOURCESEL`"]
pub type SOURCESEL_R = crate::R<u8, SOURCESEL_A>;
impl SOURCESEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SOURCESEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SOURCESEL_A::NONE),
            1 => Val(SOURCESEL_A::PRSL),
            2 => Val(SOURCESEL_A::PRS),
            3 => Val(SOURCESEL_A::PRSH),
            4 => Val(SOURCESEL_A::ACMP0),
            5 => Val(SOURCESEL_A::ACMP1),
            6 => Val(SOURCESEL_A::ADC0),
            7 => Val(SOURCESEL_A::RTC),
            8 => Val(SOURCESEL_A::RTCC),
            9 => Val(SOURCESEL_A::GPIOL),
            10 => Val(SOURCESEL_A::GPIOH),
            11 => Val(SOURCESEL_A::LETIMER0),
            12 => Val(SOURCESEL_A::LETIMER1),
            13 => Val(SOURCESEL_A::PCNT0),
            14 => Val(SOURCESEL_A::PCNT1),
            15 => Val(SOURCESEL_A::PCNT2),
            16 => Val(SOURCESEL_A::CRYOTIMER),
            17 => Val(SOURCESEL_A::CMU),
            23 => Val(SOURCESEL_A::VDAC0),
            24 => Val(SOURCESEL_A::LESENSEL),
            25 => Val(SOURCESEL_A::LESENSEH),
            26 => Val(SOURCESEL_A::LESENSED),
            27 => Val(SOURCESEL_A::LESENSE),
            28 => Val(SOURCESEL_A::ACMP2),
            29 => Val(SOURCESEL_A::ACMP3),
            30 => Val(SOURCESEL_A::ADC1),
            48 => Val(SOURCESEL_A::USART0),
            49 => Val(SOURCESEL_A::USART1),
            50 => Val(SOURCESEL_A::USART2),
            51 => Val(SOURCESEL_A::USART3),
            52 => Val(SOURCESEL_A::USART4),
            53 => Val(SOURCESEL_A::USART5),
            54 => Val(SOURCESEL_A::UART0),
            55 => Val(SOURCESEL_A::UART1),
            60 => Val(SOURCESEL_A::TIMER0),
            61 => Val(SOURCESEL_A::TIMER1),
            62 => Val(SOURCESEL_A::TIMER2),
            64 => Val(SOURCESEL_A::USB),
            67 => Val(SOURCESEL_A::CM4),
            80 => Val(SOURCESEL_A::TIMER3),
            82 => Val(SOURCESEL_A::WTIMER0),
            83 => Val(SOURCESEL_A::WTIMER1),
            84 => Val(SOURCESEL_A::WTIMER2),
            85 => Val(SOURCESEL_A::WTIMER3),
            98 => Val(SOURCESEL_A::TIMER4),
            99 => Val(SOURCESEL_A::TIMER5),
            100 => Val(SOURCESEL_A::TIMER6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SOURCESEL_A::NONE
    }
    #[doc = "Checks if the value of the field is `PRSL`"]
    #[inline(always)]
    pub fn is_prsl(&self) -> bool {
        *self == SOURCESEL_A::PRSL
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == SOURCESEL_A::PRS
    }
    #[doc = "Checks if the value of the field is `PRSH`"]
    #[inline(always)]
    pub fn is_prsh(&self) -> bool {
        *self == SOURCESEL_A::PRSH
    }
    #[doc = "Checks if the value of the field is `ACMP0`"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == SOURCESEL_A::ACMP0
    }
    #[doc = "Checks if the value of the field is `ACMP1`"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == SOURCESEL_A::ACMP1
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == SOURCESEL_A::ADC0
    }
    #[doc = "Checks if the value of the field is `RTC`"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == SOURCESEL_A::RTC
    }
    #[doc = "Checks if the value of the field is `RTCC`"]
    #[inline(always)]
    pub fn is_rtcc(&self) -> bool {
        *self == SOURCESEL_A::RTCC
    }
    #[doc = "Checks if the value of the field is `GPIOL`"]
    #[inline(always)]
    pub fn is_gpiol(&self) -> bool {
        *self == SOURCESEL_A::GPIOL
    }
    #[doc = "Checks if the value of the field is `GPIOH`"]
    #[inline(always)]
    pub fn is_gpioh(&self) -> bool {
        *self == SOURCESEL_A::GPIOH
    }
    #[doc = "Checks if the value of the field is `LETIMER0`"]
    #[inline(always)]
    pub fn is_letimer0(&self) -> bool {
        *self == SOURCESEL_A::LETIMER0
    }
    #[doc = "Checks if the value of the field is `LETIMER1`"]
    #[inline(always)]
    pub fn is_letimer1(&self) -> bool {
        *self == SOURCESEL_A::LETIMER1
    }
    #[doc = "Checks if the value of the field is `PCNT0`"]
    #[inline(always)]
    pub fn is_pcnt0(&self) -> bool {
        *self == SOURCESEL_A::PCNT0
    }
    #[doc = "Checks if the value of the field is `PCNT1`"]
    #[inline(always)]
    pub fn is_pcnt1(&self) -> bool {
        *self == SOURCESEL_A::PCNT1
    }
    #[doc = "Checks if the value of the field is `PCNT2`"]
    #[inline(always)]
    pub fn is_pcnt2(&self) -> bool {
        *self == SOURCESEL_A::PCNT2
    }
    #[doc = "Checks if the value of the field is `CRYOTIMER`"]
    #[inline(always)]
    pub fn is_cryotimer(&self) -> bool {
        *self == SOURCESEL_A::CRYOTIMER
    }
    #[doc = "Checks if the value of the field is `CMU`"]
    #[inline(always)]
    pub fn is_cmu(&self) -> bool {
        *self == SOURCESEL_A::CMU
    }
    #[doc = "Checks if the value of the field is `VDAC0`"]
    #[inline(always)]
    pub fn is_vdac0(&self) -> bool {
        *self == SOURCESEL_A::VDAC0
    }
    #[doc = "Checks if the value of the field is `LESENSEL`"]
    #[inline(always)]
    pub fn is_lesensel(&self) -> bool {
        *self == SOURCESEL_A::LESENSEL
    }
    #[doc = "Checks if the value of the field is `LESENSEH`"]
    #[inline(always)]
    pub fn is_lesenseh(&self) -> bool {
        *self == SOURCESEL_A::LESENSEH
    }
    #[doc = "Checks if the value of the field is `LESENSED`"]
    #[inline(always)]
    pub fn is_lesensed(&self) -> bool {
        *self == SOURCESEL_A::LESENSED
    }
    #[doc = "Checks if the value of the field is `LESENSE`"]
    #[inline(always)]
    pub fn is_lesense(&self) -> bool {
        *self == SOURCESEL_A::LESENSE
    }
    #[doc = "Checks if the value of the field is `ACMP2`"]
    #[inline(always)]
    pub fn is_acmp2(&self) -> bool {
        *self == SOURCESEL_A::ACMP2
    }
    #[doc = "Checks if the value of the field is `ACMP3`"]
    #[inline(always)]
    pub fn is_acmp3(&self) -> bool {
        *self == SOURCESEL_A::ACMP3
    }
    #[doc = "Checks if the value of the field is `ADC1`"]
    #[inline(always)]
    pub fn is_adc1(&self) -> bool {
        *self == SOURCESEL_A::ADC1
    }
    #[doc = "Checks if the value of the field is `USART0`"]
    #[inline(always)]
    pub fn is_usart0(&self) -> bool {
        *self == SOURCESEL_A::USART0
    }
    #[doc = "Checks if the value of the field is `USART1`"]
    #[inline(always)]
    pub fn is_usart1(&self) -> bool {
        *self == SOURCESEL_A::USART1
    }
    #[doc = "Checks if the value of the field is `USART2`"]
    #[inline(always)]
    pub fn is_usart2(&self) -> bool {
        *self == SOURCESEL_A::USART2
    }
    #[doc = "Checks if the value of the field is `USART3`"]
    #[inline(always)]
    pub fn is_usart3(&self) -> bool {
        *self == SOURCESEL_A::USART3
    }
    #[doc = "Checks if the value of the field is `USART4`"]
    #[inline(always)]
    pub fn is_usart4(&self) -> bool {
        *self == SOURCESEL_A::USART4
    }
    #[doc = "Checks if the value of the field is `USART5`"]
    #[inline(always)]
    pub fn is_usart5(&self) -> bool {
        *self == SOURCESEL_A::USART5
    }
    #[doc = "Checks if the value of the field is `UART0`"]
    #[inline(always)]
    pub fn is_uart0(&self) -> bool {
        *self == SOURCESEL_A::UART0
    }
    #[doc = "Checks if the value of the field is `UART1`"]
    #[inline(always)]
    pub fn is_uart1(&self) -> bool {
        *self == SOURCESEL_A::UART1
    }
    #[doc = "Checks if the value of the field is `TIMER0`"]
    #[inline(always)]
    pub fn is_timer0(&self) -> bool {
        *self == SOURCESEL_A::TIMER0
    }
    #[doc = "Checks if the value of the field is `TIMER1`"]
    #[inline(always)]
    pub fn is_timer1(&self) -> bool {
        *self == SOURCESEL_A::TIMER1
    }
    #[doc = "Checks if the value of the field is `TIMER2`"]
    #[inline(always)]
    pub fn is_timer2(&self) -> bool {
        *self == SOURCESEL_A::TIMER2
    }
    #[doc = "Checks if the value of the field is `USB`"]
    #[inline(always)]
    pub fn is_usb(&self) -> bool {
        *self == SOURCESEL_A::USB
    }
    #[doc = "Checks if the value of the field is `CM4`"]
    #[inline(always)]
    pub fn is_cm4(&self) -> bool {
        *self == SOURCESEL_A::CM4
    }
    #[doc = "Checks if the value of the field is `TIMER3`"]
    #[inline(always)]
    pub fn is_timer3(&self) -> bool {
        *self == SOURCESEL_A::TIMER3
    }
    #[doc = "Checks if the value of the field is `WTIMER0`"]
    #[inline(always)]
    pub fn is_wtimer0(&self) -> bool {
        *self == SOURCESEL_A::WTIMER0
    }
    #[doc = "Checks if the value of the field is `WTIMER1`"]
    #[inline(always)]
    pub fn is_wtimer1(&self) -> bool {
        *self == SOURCESEL_A::WTIMER1
    }
    #[doc = "Checks if the value of the field is `WTIMER2`"]
    #[inline(always)]
    pub fn is_wtimer2(&self) -> bool {
        *self == SOURCESEL_A::WTIMER2
    }
    #[doc = "Checks if the value of the field is `WTIMER3`"]
    #[inline(always)]
    pub fn is_wtimer3(&self) -> bool {
        *self == SOURCESEL_A::WTIMER3
    }
    #[doc = "Checks if the value of the field is `TIMER4`"]
    #[inline(always)]
    pub fn is_timer4(&self) -> bool {
        *self == SOURCESEL_A::TIMER4
    }
    #[doc = "Checks if the value of the field is `TIMER5`"]
    #[inline(always)]
    pub fn is_timer5(&self) -> bool {
        *self == SOURCESEL_A::TIMER5
    }
    #[doc = "Checks if the value of the field is `TIMER6`"]
    #[inline(always)]
    pub fn is_timer6(&self) -> bool {
        *self == SOURCESEL_A::TIMER6
    }
}
#[doc = "Write proxy for field `SOURCESEL`"]
pub struct SOURCESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SOURCESEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOURCESEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No source selected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SOURCESEL_A::NONE)
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn prsl(self) -> &'a mut W {
        self.variant(SOURCESEL_A::PRSL)
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut W {
        self.variant(SOURCESEL_A::PRS)
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn prsh(self) -> &'a mut W {
        self.variant(SOURCESEL_A::PRSH)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::ACMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn acmp1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::ACMP1)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::ADC0)
    }
    #[doc = "Real-Time Counter"]
    #[inline(always)]
    pub fn rtc(self) -> &'a mut W {
        self.variant(SOURCESEL_A::RTC)
    }
    #[doc = "Real-Time Counter and Calendar"]
    #[inline(always)]
    pub fn rtcc(self) -> &'a mut W {
        self.variant(SOURCESEL_A::RTCC)
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn gpiol(self) -> &'a mut W {
        self.variant(SOURCESEL_A::GPIOL)
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn gpioh(self) -> &'a mut W {
        self.variant(SOURCESEL_A::GPIOH)
    }
    #[doc = "Low Energy Timer 0"]
    #[inline(always)]
    pub fn letimer0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::LETIMER0)
    }
    #[doc = "Low Energy Timer 1"]
    #[inline(always)]
    pub fn letimer1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::LETIMER1)
    }
    #[doc = "Pulse Counter 0"]
    #[inline(always)]
    pub fn pcnt0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::PCNT0)
    }
    #[doc = "Pulse Counter 1"]
    #[inline(always)]
    pub fn pcnt1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::PCNT1)
    }
    #[doc = "Pulse Counter 2"]
    #[inline(always)]
    pub fn pcnt2(self) -> &'a mut W {
        self.variant(SOURCESEL_A::PCNT2)
    }
    #[doc = "CryoTimer"]
    #[inline(always)]
    pub fn cryotimer(self) -> &'a mut W {
        self.variant(SOURCESEL_A::CRYOTIMER)
    }
    #[doc = "Clock Management Unit"]
    #[inline(always)]
    pub fn cmu(self) -> &'a mut W {
        self.variant(SOURCESEL_A::CMU)
    }
    #[doc = "Digital to Analog Converter 0"]
    #[inline(always)]
    pub fn vdac0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::VDAC0)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn lesensel(self) -> &'a mut W {
        self.variant(SOURCESEL_A::LESENSEL)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn lesenseh(self) -> &'a mut W {
        self.variant(SOURCESEL_A::LESENSEH)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn lesensed(self) -> &'a mut W {
        self.variant(SOURCESEL_A::LESENSED)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn lesense(self) -> &'a mut W {
        self.variant(SOURCESEL_A::LESENSE)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn acmp2(self) -> &'a mut W {
        self.variant(SOURCESEL_A::ACMP2)
    }
    #[doc = "Analog Comparator 3"]
    #[inline(always)]
    pub fn acmp3(self) -> &'a mut W {
        self.variant(SOURCESEL_A::ACMP3)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn adc1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::ADC1)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn usart0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USART0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn usart1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USART1)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    #[inline(always)]
    pub fn usart2(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USART2)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3"]
    #[inline(always)]
    pub fn usart3(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USART3)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4"]
    #[inline(always)]
    pub fn usart4(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USART4)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5"]
    #[inline(always)]
    pub fn usart5(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USART5)
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn uart0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::UART0)
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn uart1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::UART1)
    }
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn timer0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::TIMER0)
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn timer1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::TIMER1)
    }
    #[doc = "Timer 2"]
    #[inline(always)]
    pub fn timer2(self) -> &'a mut W {
        self.variant(SOURCESEL_A::TIMER2)
    }
    #[doc = "Universal Serial Bus Interface"]
    #[inline(always)]
    pub fn usb(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USB)
    }
    #[doc = "`1000011`"]
    #[inline(always)]
    pub fn cm4(self) -> &'a mut W {
        self.variant(SOURCESEL_A::CM4)
    }
    #[doc = "Timer 3"]
    #[inline(always)]
    pub fn timer3(self) -> &'a mut W {
        self.variant(SOURCESEL_A::TIMER3)
    }
    #[doc = "Wide Timer 0"]
    #[inline(always)]
    pub fn wtimer0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::WTIMER0)
    }
    #[doc = "Wide Timer 0"]
    #[inline(always)]
    pub fn wtimer1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::WTIMER1)
    }
    #[doc = "Wide Timer 2"]
    #[inline(always)]
    pub fn wtimer2(self) -> &'a mut W {
        self.variant(SOURCESEL_A::WTIMER2)
    }
    #[doc = "Wide Timer 3"]
    #[inline(always)]
    pub fn wtimer3(self) -> &'a mut W {
        self.variant(SOURCESEL_A::WTIMER3)
    }
    #[doc = "Timer 4"]
    #[inline(always)]
    pub fn timer4(self) -> &'a mut W {
        self.variant(SOURCESEL_A::TIMER4)
    }
    #[doc = "Timer 5"]
    #[inline(always)]
    pub fn timer5(self) -> &'a mut W {
        self.variant(SOURCESEL_A::TIMER5)
    }
    #[doc = "Timer 6"]
    #[inline(always)]
    pub fn timer6(self) -> &'a mut W {
        self.variant(SOURCESEL_A::TIMER6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Edge Detect Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDSEL_A {
    #[doc = "0: Signal is left as it is"]
    OFF,
    #[doc = "1: A one HFCLK cycle pulse is generated for every positive edge of the incoming signal"]
    POSEDGE,
    #[doc = "2: A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    NEGEDGE,
    #[doc = "3: A one HFCLK clock cycle pulse is generated for every edge of the incoming signal"]
    BOTHEDGES,
}
impl From<EDSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EDSEL_A) -> Self {
        match variant {
            EDSEL_A::OFF => 0,
            EDSEL_A::POSEDGE => 1,
            EDSEL_A::NEGEDGE => 2,
            EDSEL_A::BOTHEDGES => 3,
        }
    }
}
#[doc = "Reader of field `EDSEL`"]
pub type EDSEL_R = crate::R<u8, EDSEL_A>;
impl EDSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDSEL_A {
        match self.bits {
            0 => EDSEL_A::OFF,
            1 => EDSEL_A::POSEDGE,
            2 => EDSEL_A::NEGEDGE,
            3 => EDSEL_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == EDSEL_A::OFF
    }
    #[doc = "Checks if the value of the field is `POSEDGE`"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        *self == EDSEL_A::POSEDGE
    }
    #[doc = "Checks if the value of the field is `NEGEDGE`"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == EDSEL_A::NEGEDGE
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline(always)]
    pub fn is_bothedges(&self) -> bool {
        *self == EDSEL_A::BOTHEDGES
    }
}
#[doc = "Write proxy for field `EDSEL`"]
pub struct EDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Signal is left as it is"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(EDSEL_A::OFF)
    }
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut W {
        self.variant(EDSEL_A::POSEDGE)
    }
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut W {
        self.variant(EDSEL_A::NEGEDGE)
    }
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal"]
    #[inline(always)]
    pub fn bothedges(self) -> &'a mut W {
        self.variant(EDSEL_A::BOTHEDGES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `STRETCH`"]
pub type STRETCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STRETCH`"]
pub struct STRETCH_W<'a> {
    w: &'a mut W,
}
impl<'a> STRETCH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `ORPREV`"]
pub type ORPREV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ORPREV`"]
pub struct ORPREV_W<'a> {
    w: &'a mut W,
}
impl<'a> ORPREV_W<'a> {
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
#[doc = "Reader of field `ANDNEXT`"]
pub type ANDNEXT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANDNEXT`"]
pub struct ANDNEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> ANDNEXT_W<'a> {
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
#[doc = "Reader of field `ASYNC`"]
pub type ASYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASYNC`"]
pub struct ASYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> ASYNC_W<'a> {
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
impl R {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SIGSEL_R {
        SIGSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&self) -> SOURCESEL_R {
        SOURCESEL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 20:21 - Edge Detect Select"]
    #[inline(always)]
    pub fn edsel(&self) -> EDSEL_R {
        EDSEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 25 - Stretch Channel Output"]
    #[inline(always)]
    pub fn stretch(&self) -> STRETCH_R {
        STRETCH_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Invert Channel"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Or Previous"]
    #[inline(always)]
    pub fn orprev(&self) -> ORPREV_R {
        ORPREV_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - And Next"]
    #[inline(always)]
    pub fn andnext(&self) -> ANDNEXT_R {
        ANDNEXT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Asynchronous Reflex"]
    #[inline(always)]
    pub fn async(&self) -> ASYNC_R {
        ASYNC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&mut self) -> SIGSEL_W {
        SIGSEL_W { w: self }
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&mut self) -> SOURCESEL_W {
        SOURCESEL_W { w: self }
    }
    #[doc = "Bits 20:21 - Edge Detect Select"]
    #[inline(always)]
    pub fn edsel(&mut self) -> EDSEL_W {
        EDSEL_W { w: self }
    }
    #[doc = "Bit 25 - Stretch Channel Output"]
    #[inline(always)]
    pub fn stretch(&mut self) -> STRETCH_W {
        STRETCH_W { w: self }
    }
    #[doc = "Bit 26 - Invert Channel"]
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W {
        INV_W { w: self }
    }
    #[doc = "Bit 27 - Or Previous"]
    #[inline(always)]
    pub fn orprev(&mut self) -> ORPREV_W {
        ORPREV_W { w: self }
    }
    #[doc = "Bit 28 - And Next"]
    #[inline(always)]
    pub fn andnext(&mut self) -> ANDNEXT_W {
        ANDNEXT_W { w: self }
    }
    #[doc = "Bit 30 - Asynchronous Reflex"]
    #[inline(always)]
    pub fn async(&mut self) -> ASYNC_W {
        ASYNC_W { w: self }
    }
}
