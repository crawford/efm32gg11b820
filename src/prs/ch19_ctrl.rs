#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CH19_CTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
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
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct SIGSELR {
    bits: u8,
}
impl SIGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SOURCESEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOURCESELR {
    #[doc = "No source selected"]
    NONE,
    #[doc = "Peripheral Reflex System"]
    PRSL,
    #[doc = "Peripheral Reflex System"]
    PRS,
    #[doc = "Peripheral Reflex System"]
    PRSH,
    #[doc = "Analog Comparator 0"]
    ACMP0,
    #[doc = "Analog Comparator 1"]
    ACMP1,
    #[doc = "Analog to Digital Converter 0"]
    ADC0,
    #[doc = "Real-Time Counter"]
    RTC,
    #[doc = "Real-Time Counter and Calendar"]
    RTCC,
    #[doc = "General purpose Input/Output"]
    GPIOL,
    #[doc = "General purpose Input/Output"]
    GPIOH,
    #[doc = "Low Energy Timer 0"]
    LETIMER0,
    #[doc = "Low Energy Timer 1"]
    LETIMER1,
    #[doc = "Pulse Counter 0"]
    PCNT0,
    #[doc = "Pulse Counter 1"]
    PCNT1,
    #[doc = "Pulse Counter 2"]
    PCNT2,
    #[doc = "CryoTimer"]
    CRYOTIMER,
    #[doc = "Clock Management Unit"]
    CMU,
    #[doc = "Digital to Analog Converter 0"]
    VDAC0,
    #[doc = "Low Energy Sensor Interface"]
    LESENSEL,
    #[doc = "Low Energy Sensor Interface"]
    LESENSEH,
    #[doc = "Low Energy Sensor Interface"]
    LESENSED,
    #[doc = "Low Energy Sensor Interface"]
    LESENSE,
    #[doc = "Analog Comparator 1"]
    ACMP2,
    #[doc = "Analog Comparator 3"]
    ACMP3,
    #[doc = "Analog to Digital Converter 0"]
    ADC1,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    USART0,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    USART1,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    USART2,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3"]
    USART3,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4"]
    USART4,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5"]
    USART5,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0"]
    UART0,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1"]
    UART1,
    #[doc = "Timer 0"]
    TIMER0,
    #[doc = "Timer 1"]
    TIMER1,
    #[doc = "Timer 2"]
    TIMER2,
    #[doc = "Universal Serial Bus Interface"]
    USB,
    #[doc = "undocumented"]
    CM4,
    #[doc = "Timer 3"]
    TIMER3,
    #[doc = "Wide Timer 0"]
    WTIMER0,
    #[doc = "Wide Timer 0"]
    WTIMER1,
    #[doc = "Wide Timer 2"]
    WTIMER2,
    #[doc = "Wide Timer 3"]
    WTIMER3,
    #[doc = "Timer 4"]
    TIMER4,
    #[doc = "Timer 5"]
    TIMER5,
    #[doc = "Timer 6"]
    TIMER6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SOURCESELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SOURCESELR::NONE => 0,
            SOURCESELR::PRSL => 1,
            SOURCESELR::PRS => 2,
            SOURCESELR::PRSH => 3,
            SOURCESELR::ACMP0 => 4,
            SOURCESELR::ACMP1 => 5,
            SOURCESELR::ADC0 => 6,
            SOURCESELR::RTC => 7,
            SOURCESELR::RTCC => 8,
            SOURCESELR::GPIOL => 9,
            SOURCESELR::GPIOH => 10,
            SOURCESELR::LETIMER0 => 11,
            SOURCESELR::LETIMER1 => 12,
            SOURCESELR::PCNT0 => 13,
            SOURCESELR::PCNT1 => 14,
            SOURCESELR::PCNT2 => 15,
            SOURCESELR::CRYOTIMER => 16,
            SOURCESELR::CMU => 17,
            SOURCESELR::VDAC0 => 23,
            SOURCESELR::LESENSEL => 24,
            SOURCESELR::LESENSEH => 25,
            SOURCESELR::LESENSED => 26,
            SOURCESELR::LESENSE => 27,
            SOURCESELR::ACMP2 => 28,
            SOURCESELR::ACMP3 => 29,
            SOURCESELR::ADC1 => 30,
            SOURCESELR::USART0 => 48,
            SOURCESELR::USART1 => 49,
            SOURCESELR::USART2 => 50,
            SOURCESELR::USART3 => 51,
            SOURCESELR::USART4 => 52,
            SOURCESELR::USART5 => 53,
            SOURCESELR::UART0 => 54,
            SOURCESELR::UART1 => 55,
            SOURCESELR::TIMER0 => 60,
            SOURCESELR::TIMER1 => 61,
            SOURCESELR::TIMER2 => 62,
            SOURCESELR::USB => 64,
            SOURCESELR::CM4 => 67,
            SOURCESELR::TIMER3 => 80,
            SOURCESELR::WTIMER0 => 82,
            SOURCESELR::WTIMER1 => 83,
            SOURCESELR::WTIMER2 => 84,
            SOURCESELR::WTIMER3 => 85,
            SOURCESELR::TIMER4 => 98,
            SOURCESELR::TIMER5 => 99,
            SOURCESELR::TIMER6 => 100,
            SOURCESELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SOURCESELR {
        match value {
            0 => SOURCESELR::NONE,
            1 => SOURCESELR::PRSL,
            2 => SOURCESELR::PRS,
            3 => SOURCESELR::PRSH,
            4 => SOURCESELR::ACMP0,
            5 => SOURCESELR::ACMP1,
            6 => SOURCESELR::ADC0,
            7 => SOURCESELR::RTC,
            8 => SOURCESELR::RTCC,
            9 => SOURCESELR::GPIOL,
            10 => SOURCESELR::GPIOH,
            11 => SOURCESELR::LETIMER0,
            12 => SOURCESELR::LETIMER1,
            13 => SOURCESELR::PCNT0,
            14 => SOURCESELR::PCNT1,
            15 => SOURCESELR::PCNT2,
            16 => SOURCESELR::CRYOTIMER,
            17 => SOURCESELR::CMU,
            23 => SOURCESELR::VDAC0,
            24 => SOURCESELR::LESENSEL,
            25 => SOURCESELR::LESENSEH,
            26 => SOURCESELR::LESENSED,
            27 => SOURCESELR::LESENSE,
            28 => SOURCESELR::ACMP2,
            29 => SOURCESELR::ACMP3,
            30 => SOURCESELR::ADC1,
            48 => SOURCESELR::USART0,
            49 => SOURCESELR::USART1,
            50 => SOURCESELR::USART2,
            51 => SOURCESELR::USART3,
            52 => SOURCESELR::USART4,
            53 => SOURCESELR::USART5,
            54 => SOURCESELR::UART0,
            55 => SOURCESELR::UART1,
            60 => SOURCESELR::TIMER0,
            61 => SOURCESELR::TIMER1,
            62 => SOURCESELR::TIMER2,
            64 => SOURCESELR::USB,
            67 => SOURCESELR::CM4,
            80 => SOURCESELR::TIMER3,
            82 => SOURCESELR::WTIMER0,
            83 => SOURCESELR::WTIMER1,
            84 => SOURCESELR::WTIMER2,
            85 => SOURCESELR::WTIMER3,
            98 => SOURCESELR::TIMER4,
            99 => SOURCESELR::TIMER5,
            100 => SOURCESELR::TIMER6,
            i => SOURCESELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SOURCESELR::NONE
    }
    #[doc = "Checks if the value of the field is `PRSL`"]
    #[inline]
    pub fn is_prsl(&self) -> bool {
        *self == SOURCESELR::PRSL
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline]
    pub fn is_prs(&self) -> bool {
        *self == SOURCESELR::PRS
    }
    #[doc = "Checks if the value of the field is `PRSH`"]
    #[inline]
    pub fn is_prsh(&self) -> bool {
        *self == SOURCESELR::PRSH
    }
    #[doc = "Checks if the value of the field is `ACMP0`"]
    #[inline]
    pub fn is_acmp0(&self) -> bool {
        *self == SOURCESELR::ACMP0
    }
    #[doc = "Checks if the value of the field is `ACMP1`"]
    #[inline]
    pub fn is_acmp1(&self) -> bool {
        *self == SOURCESELR::ACMP1
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline]
    pub fn is_adc0(&self) -> bool {
        *self == SOURCESELR::ADC0
    }
    #[doc = "Checks if the value of the field is `RTC`"]
    #[inline]
    pub fn is_rtc(&self) -> bool {
        *self == SOURCESELR::RTC
    }
    #[doc = "Checks if the value of the field is `RTCC`"]
    #[inline]
    pub fn is_rtcc(&self) -> bool {
        *self == SOURCESELR::RTCC
    }
    #[doc = "Checks if the value of the field is `GPIOL`"]
    #[inline]
    pub fn is_gpiol(&self) -> bool {
        *self == SOURCESELR::GPIOL
    }
    #[doc = "Checks if the value of the field is `GPIOH`"]
    #[inline]
    pub fn is_gpioh(&self) -> bool {
        *self == SOURCESELR::GPIOH
    }
    #[doc = "Checks if the value of the field is `LETIMER0`"]
    #[inline]
    pub fn is_letimer0(&self) -> bool {
        *self == SOURCESELR::LETIMER0
    }
    #[doc = "Checks if the value of the field is `LETIMER1`"]
    #[inline]
    pub fn is_letimer1(&self) -> bool {
        *self == SOURCESELR::LETIMER1
    }
    #[doc = "Checks if the value of the field is `PCNT0`"]
    #[inline]
    pub fn is_pcnt0(&self) -> bool {
        *self == SOURCESELR::PCNT0
    }
    #[doc = "Checks if the value of the field is `PCNT1`"]
    #[inline]
    pub fn is_pcnt1(&self) -> bool {
        *self == SOURCESELR::PCNT1
    }
    #[doc = "Checks if the value of the field is `PCNT2`"]
    #[inline]
    pub fn is_pcnt2(&self) -> bool {
        *self == SOURCESELR::PCNT2
    }
    #[doc = "Checks if the value of the field is `CRYOTIMER`"]
    #[inline]
    pub fn is_cryotimer(&self) -> bool {
        *self == SOURCESELR::CRYOTIMER
    }
    #[doc = "Checks if the value of the field is `CMU`"]
    #[inline]
    pub fn is_cmu(&self) -> bool {
        *self == SOURCESELR::CMU
    }
    #[doc = "Checks if the value of the field is `VDAC0`"]
    #[inline]
    pub fn is_vdac0(&self) -> bool {
        *self == SOURCESELR::VDAC0
    }
    #[doc = "Checks if the value of the field is `LESENSEL`"]
    #[inline]
    pub fn is_lesensel(&self) -> bool {
        *self == SOURCESELR::LESENSEL
    }
    #[doc = "Checks if the value of the field is `LESENSEH`"]
    #[inline]
    pub fn is_lesenseh(&self) -> bool {
        *self == SOURCESELR::LESENSEH
    }
    #[doc = "Checks if the value of the field is `LESENSED`"]
    #[inline]
    pub fn is_lesensed(&self) -> bool {
        *self == SOURCESELR::LESENSED
    }
    #[doc = "Checks if the value of the field is `LESENSE`"]
    #[inline]
    pub fn is_lesense(&self) -> bool {
        *self == SOURCESELR::LESENSE
    }
    #[doc = "Checks if the value of the field is `ACMP2`"]
    #[inline]
    pub fn is_acmp2(&self) -> bool {
        *self == SOURCESELR::ACMP2
    }
    #[doc = "Checks if the value of the field is `ACMP3`"]
    #[inline]
    pub fn is_acmp3(&self) -> bool {
        *self == SOURCESELR::ACMP3
    }
    #[doc = "Checks if the value of the field is `ADC1`"]
    #[inline]
    pub fn is_adc1(&self) -> bool {
        *self == SOURCESELR::ADC1
    }
    #[doc = "Checks if the value of the field is `USART0`"]
    #[inline]
    pub fn is_usart0(&self) -> bool {
        *self == SOURCESELR::USART0
    }
    #[doc = "Checks if the value of the field is `USART1`"]
    #[inline]
    pub fn is_usart1(&self) -> bool {
        *self == SOURCESELR::USART1
    }
    #[doc = "Checks if the value of the field is `USART2`"]
    #[inline]
    pub fn is_usart2(&self) -> bool {
        *self == SOURCESELR::USART2
    }
    #[doc = "Checks if the value of the field is `USART3`"]
    #[inline]
    pub fn is_usart3(&self) -> bool {
        *self == SOURCESELR::USART3
    }
    #[doc = "Checks if the value of the field is `USART4`"]
    #[inline]
    pub fn is_usart4(&self) -> bool {
        *self == SOURCESELR::USART4
    }
    #[doc = "Checks if the value of the field is `USART5`"]
    #[inline]
    pub fn is_usart5(&self) -> bool {
        *self == SOURCESELR::USART5
    }
    #[doc = "Checks if the value of the field is `UART0`"]
    #[inline]
    pub fn is_uart0(&self) -> bool {
        *self == SOURCESELR::UART0
    }
    #[doc = "Checks if the value of the field is `UART1`"]
    #[inline]
    pub fn is_uart1(&self) -> bool {
        *self == SOURCESELR::UART1
    }
    #[doc = "Checks if the value of the field is `TIMER0`"]
    #[inline]
    pub fn is_timer0(&self) -> bool {
        *self == SOURCESELR::TIMER0
    }
    #[doc = "Checks if the value of the field is `TIMER1`"]
    #[inline]
    pub fn is_timer1(&self) -> bool {
        *self == SOURCESELR::TIMER1
    }
    #[doc = "Checks if the value of the field is `TIMER2`"]
    #[inline]
    pub fn is_timer2(&self) -> bool {
        *self == SOURCESELR::TIMER2
    }
    #[doc = "Checks if the value of the field is `USB`"]
    #[inline]
    pub fn is_usb(&self) -> bool {
        *self == SOURCESELR::USB
    }
    #[doc = "Checks if the value of the field is `CM4`"]
    #[inline]
    pub fn is_cm4(&self) -> bool {
        *self == SOURCESELR::CM4
    }
    #[doc = "Checks if the value of the field is `TIMER3`"]
    #[inline]
    pub fn is_timer3(&self) -> bool {
        *self == SOURCESELR::TIMER3
    }
    #[doc = "Checks if the value of the field is `WTIMER0`"]
    #[inline]
    pub fn is_wtimer0(&self) -> bool {
        *self == SOURCESELR::WTIMER0
    }
    #[doc = "Checks if the value of the field is `WTIMER1`"]
    #[inline]
    pub fn is_wtimer1(&self) -> bool {
        *self == SOURCESELR::WTIMER1
    }
    #[doc = "Checks if the value of the field is `WTIMER2`"]
    #[inline]
    pub fn is_wtimer2(&self) -> bool {
        *self == SOURCESELR::WTIMER2
    }
    #[doc = "Checks if the value of the field is `WTIMER3`"]
    #[inline]
    pub fn is_wtimer3(&self) -> bool {
        *self == SOURCESELR::WTIMER3
    }
    #[doc = "Checks if the value of the field is `TIMER4`"]
    #[inline]
    pub fn is_timer4(&self) -> bool {
        *self == SOURCESELR::TIMER4
    }
    #[doc = "Checks if the value of the field is `TIMER5`"]
    #[inline]
    pub fn is_timer5(&self) -> bool {
        *self == SOURCESELR::TIMER5
    }
    #[doc = "Checks if the value of the field is `TIMER6`"]
    #[inline]
    pub fn is_timer6(&self) -> bool {
        *self == SOURCESELR::TIMER6
    }
}
#[doc = "Possible values of the field `EDSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDSELR {
    #[doc = "Signal is left as it is"]
    OFF,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal"]
    POSEDGE,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    NEGEDGE,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal"]
    BOTHEDGES,
}
impl EDSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EDSELR::OFF => 0,
            EDSELR::POSEDGE => 1,
            EDSELR::NEGEDGE => 2,
            EDSELR::BOTHEDGES => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EDSELR {
        match value {
            0 => EDSELR::OFF,
            1 => EDSELR::POSEDGE,
            2 => EDSELR::NEGEDGE,
            3 => EDSELR::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == EDSELR::OFF
    }
    #[doc = "Checks if the value of the field is `POSEDGE`"]
    #[inline]
    pub fn is_posedge(&self) -> bool {
        *self == EDSELR::POSEDGE
    }
    #[doc = "Checks if the value of the field is `NEGEDGE`"]
    #[inline]
    pub fn is_negedge(&self) -> bool {
        *self == EDSELR::NEGEDGE
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline]
    pub fn is_bothedges(&self) -> bool {
        *self == EDSELR::BOTHEDGES
    }
}
#[doc = r" Value of the field"]
pub struct STRETCHR {
    bits: bool,
}
impl STRETCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct INVR {
    bits: bool,
}
impl INVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ORPREVR {
    bits: bool,
}
impl ORPREVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ANDNEXTR {
    bits: bool,
}
impl ANDNEXTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ASYNCR {
    bits: bool,
}
impl ASYNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Proxy"]
pub struct _SIGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SIGSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SOURCESEL`"]
pub enum SOURCESELW {
    #[doc = "No source selected"]
    NONE,
    #[doc = "Peripheral Reflex System"]
    PRSL,
    #[doc = "Peripheral Reflex System"]
    PRS,
    #[doc = "Peripheral Reflex System"]
    PRSH,
    #[doc = "Analog Comparator 0"]
    ACMP0,
    #[doc = "Analog Comparator 1"]
    ACMP1,
    #[doc = "Analog to Digital Converter 0"]
    ADC0,
    #[doc = "Real-Time Counter"]
    RTC,
    #[doc = "Real-Time Counter and Calendar"]
    RTCC,
    #[doc = "General purpose Input/Output"]
    GPIOL,
    #[doc = "General purpose Input/Output"]
    GPIOH,
    #[doc = "Low Energy Timer 0"]
    LETIMER0,
    #[doc = "Low Energy Timer 1"]
    LETIMER1,
    #[doc = "Pulse Counter 0"]
    PCNT0,
    #[doc = "Pulse Counter 1"]
    PCNT1,
    #[doc = "Pulse Counter 2"]
    PCNT2,
    #[doc = "CryoTimer"]
    CRYOTIMER,
    #[doc = "Clock Management Unit"]
    CMU,
    #[doc = "Digital to Analog Converter 0"]
    VDAC0,
    #[doc = "Low Energy Sensor Interface"]
    LESENSEL,
    #[doc = "Low Energy Sensor Interface"]
    LESENSEH,
    #[doc = "Low Energy Sensor Interface"]
    LESENSED,
    #[doc = "Low Energy Sensor Interface"]
    LESENSE,
    #[doc = "Analog Comparator 1"]
    ACMP2,
    #[doc = "Analog Comparator 3"]
    ACMP3,
    #[doc = "Analog to Digital Converter 0"]
    ADC1,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    USART0,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    USART1,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    USART2,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3"]
    USART3,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4"]
    USART4,
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5"]
    USART5,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0"]
    UART0,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1"]
    UART1,
    #[doc = "Timer 0"]
    TIMER0,
    #[doc = "Timer 1"]
    TIMER1,
    #[doc = "Timer 2"]
    TIMER2,
    #[doc = "Universal Serial Bus Interface"]
    USB,
    #[doc = "`1000011`"]
    CM4,
    #[doc = "Timer 3"]
    TIMER3,
    #[doc = "Wide Timer 0"]
    WTIMER0,
    #[doc = "Wide Timer 0"]
    WTIMER1,
    #[doc = "Wide Timer 2"]
    WTIMER2,
    #[doc = "Wide Timer 3"]
    WTIMER3,
    #[doc = "Timer 4"]
    TIMER4,
    #[doc = "Timer 5"]
    TIMER5,
    #[doc = "Timer 6"]
    TIMER6,
}
impl SOURCESELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SOURCESELW::NONE => 0,
            SOURCESELW::PRSL => 1,
            SOURCESELW::PRS => 2,
            SOURCESELW::PRSH => 3,
            SOURCESELW::ACMP0 => 4,
            SOURCESELW::ACMP1 => 5,
            SOURCESELW::ADC0 => 6,
            SOURCESELW::RTC => 7,
            SOURCESELW::RTCC => 8,
            SOURCESELW::GPIOL => 9,
            SOURCESELW::GPIOH => 10,
            SOURCESELW::LETIMER0 => 11,
            SOURCESELW::LETIMER1 => 12,
            SOURCESELW::PCNT0 => 13,
            SOURCESELW::PCNT1 => 14,
            SOURCESELW::PCNT2 => 15,
            SOURCESELW::CRYOTIMER => 16,
            SOURCESELW::CMU => 17,
            SOURCESELW::VDAC0 => 23,
            SOURCESELW::LESENSEL => 24,
            SOURCESELW::LESENSEH => 25,
            SOURCESELW::LESENSED => 26,
            SOURCESELW::LESENSE => 27,
            SOURCESELW::ACMP2 => 28,
            SOURCESELW::ACMP3 => 29,
            SOURCESELW::ADC1 => 30,
            SOURCESELW::USART0 => 48,
            SOURCESELW::USART1 => 49,
            SOURCESELW::USART2 => 50,
            SOURCESELW::USART3 => 51,
            SOURCESELW::USART4 => 52,
            SOURCESELW::USART5 => 53,
            SOURCESELW::UART0 => 54,
            SOURCESELW::UART1 => 55,
            SOURCESELW::TIMER0 => 60,
            SOURCESELW::TIMER1 => 61,
            SOURCESELW::TIMER2 => 62,
            SOURCESELW::USB => 64,
            SOURCESELW::CM4 => 67,
            SOURCESELW::TIMER3 => 80,
            SOURCESELW::WTIMER0 => 82,
            SOURCESELW::WTIMER1 => 83,
            SOURCESELW::WTIMER2 => 84,
            SOURCESELW::WTIMER3 => 85,
            SOURCESELW::TIMER4 => 98,
            SOURCESELW::TIMER5 => 99,
            SOURCESELW::TIMER6 => 100,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOURCESELW<'a> {
    w: &'a mut W,
}
impl<'a> _SOURCESELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOURCESELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No source selected"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(SOURCESELW::NONE)
    }
    #[doc = "Peripheral Reflex System"]
    #[inline]
    pub fn prsl(self) -> &'a mut W {
        self.variant(SOURCESELW::PRSL)
    }
    #[doc = "Peripheral Reflex System"]
    #[inline]
    pub fn prs(self) -> &'a mut W {
        self.variant(SOURCESELW::PRS)
    }
    #[doc = "Peripheral Reflex System"]
    #[inline]
    pub fn prsh(self) -> &'a mut W {
        self.variant(SOURCESELW::PRSH)
    }
    #[doc = "Analog Comparator 0"]
    #[inline]
    pub fn acmp0(self) -> &'a mut W {
        self.variant(SOURCESELW::ACMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline]
    pub fn acmp1(self) -> &'a mut W {
        self.variant(SOURCESELW::ACMP1)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline]
    pub fn adc0(self) -> &'a mut W {
        self.variant(SOURCESELW::ADC0)
    }
    #[doc = "Real-Time Counter"]
    #[inline]
    pub fn rtc(self) -> &'a mut W {
        self.variant(SOURCESELW::RTC)
    }
    #[doc = "Real-Time Counter and Calendar"]
    #[inline]
    pub fn rtcc(self) -> &'a mut W {
        self.variant(SOURCESELW::RTCC)
    }
    #[doc = "General purpose Input/Output"]
    #[inline]
    pub fn gpiol(self) -> &'a mut W {
        self.variant(SOURCESELW::GPIOL)
    }
    #[doc = "General purpose Input/Output"]
    #[inline]
    pub fn gpioh(self) -> &'a mut W {
        self.variant(SOURCESELW::GPIOH)
    }
    #[doc = "Low Energy Timer 0"]
    #[inline]
    pub fn letimer0(self) -> &'a mut W {
        self.variant(SOURCESELW::LETIMER0)
    }
    #[doc = "Low Energy Timer 1"]
    #[inline]
    pub fn letimer1(self) -> &'a mut W {
        self.variant(SOURCESELW::LETIMER1)
    }
    #[doc = "Pulse Counter 0"]
    #[inline]
    pub fn pcnt0(self) -> &'a mut W {
        self.variant(SOURCESELW::PCNT0)
    }
    #[doc = "Pulse Counter 1"]
    #[inline]
    pub fn pcnt1(self) -> &'a mut W {
        self.variant(SOURCESELW::PCNT1)
    }
    #[doc = "Pulse Counter 2"]
    #[inline]
    pub fn pcnt2(self) -> &'a mut W {
        self.variant(SOURCESELW::PCNT2)
    }
    #[doc = "CryoTimer"]
    #[inline]
    pub fn cryotimer(self) -> &'a mut W {
        self.variant(SOURCESELW::CRYOTIMER)
    }
    #[doc = "Clock Management Unit"]
    #[inline]
    pub fn cmu(self) -> &'a mut W {
        self.variant(SOURCESELW::CMU)
    }
    #[doc = "Digital to Analog Converter 0"]
    #[inline]
    pub fn vdac0(self) -> &'a mut W {
        self.variant(SOURCESELW::VDAC0)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline]
    pub fn lesensel(self) -> &'a mut W {
        self.variant(SOURCESELW::LESENSEL)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline]
    pub fn lesenseh(self) -> &'a mut W {
        self.variant(SOURCESELW::LESENSEH)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline]
    pub fn lesensed(self) -> &'a mut W {
        self.variant(SOURCESELW::LESENSED)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline]
    pub fn lesense(self) -> &'a mut W {
        self.variant(SOURCESELW::LESENSE)
    }
    #[doc = "Analog Comparator 1"]
    #[inline]
    pub fn acmp2(self) -> &'a mut W {
        self.variant(SOURCESELW::ACMP2)
    }
    #[doc = "Analog Comparator 3"]
    #[inline]
    pub fn acmp3(self) -> &'a mut W {
        self.variant(SOURCESELW::ACMP3)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline]
    pub fn adc1(self) -> &'a mut W {
        self.variant(SOURCESELW::ADC1)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    #[inline]
    pub fn usart0(self) -> &'a mut W {
        self.variant(SOURCESELW::USART0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    #[inline]
    pub fn usart1(self) -> &'a mut W {
        self.variant(SOURCESELW::USART1)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    #[inline]
    pub fn usart2(self) -> &'a mut W {
        self.variant(SOURCESELW::USART2)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3"]
    #[inline]
    pub fn usart3(self) -> &'a mut W {
        self.variant(SOURCESELW::USART3)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 4"]
    #[inline]
    pub fn usart4(self) -> &'a mut W {
        self.variant(SOURCESELW::USART4)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 5"]
    #[inline]
    pub fn usart5(self) -> &'a mut W {
        self.variant(SOURCESELW::USART5)
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 0"]
    #[inline]
    pub fn uart0(self) -> &'a mut W {
        self.variant(SOURCESELW::UART0)
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 1"]
    #[inline]
    pub fn uart1(self) -> &'a mut W {
        self.variant(SOURCESELW::UART1)
    }
    #[doc = "Timer 0"]
    #[inline]
    pub fn timer0(self) -> &'a mut W {
        self.variant(SOURCESELW::TIMER0)
    }
    #[doc = "Timer 1"]
    #[inline]
    pub fn timer1(self) -> &'a mut W {
        self.variant(SOURCESELW::TIMER1)
    }
    #[doc = "Timer 2"]
    #[inline]
    pub fn timer2(self) -> &'a mut W {
        self.variant(SOURCESELW::TIMER2)
    }
    #[doc = "Universal Serial Bus Interface"]
    #[inline]
    pub fn usb(self) -> &'a mut W {
        self.variant(SOURCESELW::USB)
    }
    #[doc = "`1000011`"]
    #[inline]
    pub fn cm4(self) -> &'a mut W {
        self.variant(SOURCESELW::CM4)
    }
    #[doc = "Timer 3"]
    #[inline]
    pub fn timer3(self) -> &'a mut W {
        self.variant(SOURCESELW::TIMER3)
    }
    #[doc = "Wide Timer 0"]
    #[inline]
    pub fn wtimer0(self) -> &'a mut W {
        self.variant(SOURCESELW::WTIMER0)
    }
    #[doc = "Wide Timer 0"]
    #[inline]
    pub fn wtimer1(self) -> &'a mut W {
        self.variant(SOURCESELW::WTIMER1)
    }
    #[doc = "Wide Timer 2"]
    #[inline]
    pub fn wtimer2(self) -> &'a mut W {
        self.variant(SOURCESELW::WTIMER2)
    }
    #[doc = "Wide Timer 3"]
    #[inline]
    pub fn wtimer3(self) -> &'a mut W {
        self.variant(SOURCESELW::WTIMER3)
    }
    #[doc = "Timer 4"]
    #[inline]
    pub fn timer4(self) -> &'a mut W {
        self.variant(SOURCESELW::TIMER4)
    }
    #[doc = "Timer 5"]
    #[inline]
    pub fn timer5(self) -> &'a mut W {
        self.variant(SOURCESELW::TIMER5)
    }
    #[doc = "Timer 6"]
    #[inline]
    pub fn timer6(self) -> &'a mut W {
        self.variant(SOURCESELW::TIMER6)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EDSEL`"]
pub enum EDSELW {
    #[doc = "Signal is left as it is"]
    OFF,
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal"]
    POSEDGE,
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    NEGEDGE,
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal"]
    BOTHEDGES,
}
impl EDSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EDSELW::OFF => 0,
            EDSELW::POSEDGE => 1,
            EDSELW::NEGEDGE => 2,
            EDSELW::BOTHEDGES => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDSELW<'a> {
    w: &'a mut W,
}
impl<'a> _EDSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Signal is left as it is"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(EDSELW::OFF)
    }
    #[doc = "A one HFCLK cycle pulse is generated for every positive edge of the incoming signal"]
    #[inline]
    pub fn posedge(self) -> &'a mut W {
        self.variant(EDSELW::POSEDGE)
    }
    #[doc = "A one HFCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    #[inline]
    pub fn negedge(self) -> &'a mut W {
        self.variant(EDSELW::NEGEDGE)
    }
    #[doc = "A one HFCLK clock cycle pulse is generated for every edge of the incoming signal"]
    #[inline]
    pub fn bothedges(self) -> &'a mut W {
        self.variant(EDSELW::BOTHEDGES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STRETCHW<'a> {
    w: &'a mut W,
}
impl<'a> _STRETCHW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INVW<'a> {
    w: &'a mut W,
}
impl<'a> _INVW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ORPREVW<'a> {
    w: &'a mut W,
}
impl<'a> _ORPREVW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ANDNEXTW<'a> {
    w: &'a mut W,
}
impl<'a> _ANDNEXTW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ASYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _ASYNCW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline]
    pub fn sigsel(&self) -> SIGSELR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SIGSELR { bits }
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline]
    pub fn sourcesel(&self) -> SOURCESELR {
        SOURCESELR::_from({
            const MASK: u8 = 127;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Edge Detect Select"]
    #[inline]
    pub fn edsel(&self) -> EDSELR {
        EDSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 25 - Stretch Channel Output"]
    #[inline]
    pub fn stretch(&self) -> STRETCHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STRETCHR { bits }
    }
    #[doc = "Bit 26 - Invert Channel"]
    #[inline]
    pub fn inv(&self) -> INVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INVR { bits }
    }
    #[doc = "Bit 27 - Or Previous"]
    #[inline]
    pub fn orprev(&self) -> ORPREVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ORPREVR { bits }
    }
    #[doc = "Bit 28 - And Next"]
    #[inline]
    pub fn andnext(&self) -> ANDNEXTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ANDNEXTR { bits }
    }
    #[doc = "Bit 30 - Asynchronous Reflex"]
    #[inline]
    pub fn async_(&self) -> ASYNCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ASYNCR { bits }
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
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline]
    pub fn sigsel(&mut self) -> _SIGSELW {
        _SIGSELW { w: self }
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline]
    pub fn sourcesel(&mut self) -> _SOURCESELW {
        _SOURCESELW { w: self }
    }
    #[doc = "Bits 20:21 - Edge Detect Select"]
    #[inline]
    pub fn edsel(&mut self) -> _EDSELW {
        _EDSELW { w: self }
    }
    #[doc = "Bit 25 - Stretch Channel Output"]
    #[inline]
    pub fn stretch(&mut self) -> _STRETCHW {
        _STRETCHW { w: self }
    }
    #[doc = "Bit 26 - Invert Channel"]
    #[inline]
    pub fn inv(&mut self) -> _INVW {
        _INVW { w: self }
    }
    #[doc = "Bit 27 - Or Previous"]
    #[inline]
    pub fn orprev(&mut self) -> _ORPREVW {
        _ORPREVW { w: self }
    }
    #[doc = "Bit 28 - And Next"]
    #[inline]
    pub fn andnext(&mut self) -> _ANDNEXTW {
        _ANDNEXTW { w: self }
    }
    #[doc = "Bit 30 - Asynchronous Reflex"]
    #[inline]
    pub fn async_(&mut self) -> _ASYNCW {
        _ASYNCW { w: self }
    }
}
