#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CH22_REQSEL {
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
    PRS,
    #[doc = "Analog to Digital Converter 0"]
    ADC0,
    #[doc = "Analog to Digital Converter 0"]
    ADC1,
    #[doc = "Digital to Analog Converter 0"]
    VDAC0,
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
    #[doc = "Low Energy UART 0"]
    LEUART0,
    #[doc = "Low Energy UART 1"]
    LEUART1,
    #[doc = "I2C 0"]
    I2C0,
    #[doc = "I2C 1"]
    I2C1,
    #[doc = "I2C 2"]
    I2C2,
    #[doc = "Timer 0"]
    TIMER0,
    #[doc = "Timer 1"]
    TIMER1,
    #[doc = "Timer 2"]
    TIMER2,
    #[doc = "Timer 3"]
    TIMER3,
    #[doc = "Timer 4"]
    TIMER4,
    #[doc = "Timer 5"]
    TIMER5,
    #[doc = "Timer 6"]
    TIMER6,
    #[doc = "Wide Timer 0"]
    WTIMER0,
    #[doc = "Wide Timer 0"]
    WTIMER1,
    #[doc = "Wide Timer 2"]
    WTIMER2,
    #[doc = "Wide Timer 3"]
    WTIMER3,
    #[doc = "Memory System Controller"]
    MSC,
    #[doc = "Advanced Encryption Standard Accelerator"]
    CRYPTO0,
    #[doc = "External Bus Interface"]
    EBI,
    #[doc = "Capacitive touch sense module"]
    CSEN,
    #[doc = "Low Energy Sensor Interface"]
    LESENSE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SOURCESELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SOURCESELR::NONE => 0,
            SOURCESELR::PRS => 1,
            SOURCESELR::ADC0 => 8,
            SOURCESELR::ADC1 => 9,
            SOURCESELR::VDAC0 => 10,
            SOURCESELR::USART0 => 12,
            SOURCESELR::USART1 => 13,
            SOURCESELR::USART2 => 14,
            SOURCESELR::USART3 => 15,
            SOURCESELR::USART4 => 16,
            SOURCESELR::USART5 => 17,
            SOURCESELR::UART0 => 18,
            SOURCESELR::UART1 => 19,
            SOURCESELR::LEUART0 => 20,
            SOURCESELR::LEUART1 => 21,
            SOURCESELR::I2C0 => 22,
            SOURCESELR::I2C1 => 23,
            SOURCESELR::I2C2 => 24,
            SOURCESELR::TIMER0 => 25,
            SOURCESELR::TIMER1 => 26,
            SOURCESELR::TIMER2 => 27,
            SOURCESELR::TIMER3 => 28,
            SOURCESELR::TIMER4 => 29,
            SOURCESELR::TIMER5 => 30,
            SOURCESELR::TIMER6 => 31,
            SOURCESELR::WTIMER0 => 32,
            SOURCESELR::WTIMER1 => 33,
            SOURCESELR::WTIMER2 => 34,
            SOURCESELR::WTIMER3 => 35,
            SOURCESELR::MSC => 48,
            SOURCESELR::CRYPTO0 => 49,
            SOURCESELR::EBI => 50,
            SOURCESELR::CSEN => 61,
            SOURCESELR::LESENSE => 62,
            SOURCESELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SOURCESELR {
        match value {
            0 => SOURCESELR::NONE,
            1 => SOURCESELR::PRS,
            8 => SOURCESELR::ADC0,
            9 => SOURCESELR::ADC1,
            10 => SOURCESELR::VDAC0,
            12 => SOURCESELR::USART0,
            13 => SOURCESELR::USART1,
            14 => SOURCESELR::USART2,
            15 => SOURCESELR::USART3,
            16 => SOURCESELR::USART4,
            17 => SOURCESELR::USART5,
            18 => SOURCESELR::UART0,
            19 => SOURCESELR::UART1,
            20 => SOURCESELR::LEUART0,
            21 => SOURCESELR::LEUART1,
            22 => SOURCESELR::I2C0,
            23 => SOURCESELR::I2C1,
            24 => SOURCESELR::I2C2,
            25 => SOURCESELR::TIMER0,
            26 => SOURCESELR::TIMER1,
            27 => SOURCESELR::TIMER2,
            28 => SOURCESELR::TIMER3,
            29 => SOURCESELR::TIMER4,
            30 => SOURCESELR::TIMER5,
            31 => SOURCESELR::TIMER6,
            32 => SOURCESELR::WTIMER0,
            33 => SOURCESELR::WTIMER1,
            34 => SOURCESELR::WTIMER2,
            35 => SOURCESELR::WTIMER3,
            48 => SOURCESELR::MSC,
            49 => SOURCESELR::CRYPTO0,
            50 => SOURCESELR::EBI,
            61 => SOURCESELR::CSEN,
            62 => SOURCESELR::LESENSE,
            i => SOURCESELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == SOURCESELR::NONE
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline]
    pub fn is_prs(&self) -> bool {
        *self == SOURCESELR::PRS
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline]
    pub fn is_adc0(&self) -> bool {
        *self == SOURCESELR::ADC0
    }
    #[doc = "Checks if the value of the field is `ADC1`"]
    #[inline]
    pub fn is_adc1(&self) -> bool {
        *self == SOURCESELR::ADC1
    }
    #[doc = "Checks if the value of the field is `VDAC0`"]
    #[inline]
    pub fn is_vdac0(&self) -> bool {
        *self == SOURCESELR::VDAC0
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
    #[doc = "Checks if the value of the field is `LEUART0`"]
    #[inline]
    pub fn is_leuart0(&self) -> bool {
        *self == SOURCESELR::LEUART0
    }
    #[doc = "Checks if the value of the field is `LEUART1`"]
    #[inline]
    pub fn is_leuart1(&self) -> bool {
        *self == SOURCESELR::LEUART1
    }
    #[doc = "Checks if the value of the field is `I2C0`"]
    #[inline]
    pub fn is_i2c0(&self) -> bool {
        *self == SOURCESELR::I2C0
    }
    #[doc = "Checks if the value of the field is `I2C1`"]
    #[inline]
    pub fn is_i2c1(&self) -> bool {
        *self == SOURCESELR::I2C1
    }
    #[doc = "Checks if the value of the field is `I2C2`"]
    #[inline]
    pub fn is_i2c2(&self) -> bool {
        *self == SOURCESELR::I2C2
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
    #[doc = "Checks if the value of the field is `TIMER3`"]
    #[inline]
    pub fn is_timer3(&self) -> bool {
        *self == SOURCESELR::TIMER3
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
    #[doc = "Checks if the value of the field is `MSC`"]
    #[inline]
    pub fn is_msc(&self) -> bool {
        *self == SOURCESELR::MSC
    }
    #[doc = "Checks if the value of the field is `CRYPTO0`"]
    #[inline]
    pub fn is_crypto0(&self) -> bool {
        *self == SOURCESELR::CRYPTO0
    }
    #[doc = "Checks if the value of the field is `EBI`"]
    #[inline]
    pub fn is_ebi(&self) -> bool {
        *self == SOURCESELR::EBI
    }
    #[doc = "Checks if the value of the field is `CSEN`"]
    #[inline]
    pub fn is_csen(&self) -> bool {
        *self == SOURCESELR::CSEN
    }
    #[doc = "Checks if the value of the field is `LESENSE`"]
    #[inline]
    pub fn is_lesense(&self) -> bool {
        *self == SOURCESELR::LESENSE
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
        const MASK: u8 = 15;
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
    PRS,
    #[doc = "Analog to Digital Converter 0"]
    ADC0,
    #[doc = "Analog to Digital Converter 0"]
    ADC1,
    #[doc = "Digital to Analog Converter 0"]
    VDAC0,
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
    #[doc = "Low Energy UART 0"]
    LEUART0,
    #[doc = "Low Energy UART 1"]
    LEUART1,
    #[doc = "I2C 0"]
    I2C0,
    #[doc = "I2C 1"]
    I2C1,
    #[doc = "I2C 2"]
    I2C2,
    #[doc = "Timer 0"]
    TIMER0,
    #[doc = "Timer 1"]
    TIMER1,
    #[doc = "Timer 2"]
    TIMER2,
    #[doc = "Timer 3"]
    TIMER3,
    #[doc = "Timer 4"]
    TIMER4,
    #[doc = "Timer 5"]
    TIMER5,
    #[doc = "Timer 6"]
    TIMER6,
    #[doc = "Wide Timer 0"]
    WTIMER0,
    #[doc = "Wide Timer 0"]
    WTIMER1,
    #[doc = "Wide Timer 2"]
    WTIMER2,
    #[doc = "Wide Timer 3"]
    WTIMER3,
    #[doc = "Memory System Controller"]
    MSC,
    #[doc = "Advanced Encryption Standard Accelerator"]
    CRYPTO0,
    #[doc = "External Bus Interface"]
    EBI,
    #[doc = "Capacitive touch sense module"]
    CSEN,
    #[doc = "Low Energy Sensor Interface"]
    LESENSE,
}
impl SOURCESELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SOURCESELW::NONE => 0,
            SOURCESELW::PRS => 1,
            SOURCESELW::ADC0 => 8,
            SOURCESELW::ADC1 => 9,
            SOURCESELW::VDAC0 => 10,
            SOURCESELW::USART0 => 12,
            SOURCESELW::USART1 => 13,
            SOURCESELW::USART2 => 14,
            SOURCESELW::USART3 => 15,
            SOURCESELW::USART4 => 16,
            SOURCESELW::USART5 => 17,
            SOURCESELW::UART0 => 18,
            SOURCESELW::UART1 => 19,
            SOURCESELW::LEUART0 => 20,
            SOURCESELW::LEUART1 => 21,
            SOURCESELW::I2C0 => 22,
            SOURCESELW::I2C1 => 23,
            SOURCESELW::I2C2 => 24,
            SOURCESELW::TIMER0 => 25,
            SOURCESELW::TIMER1 => 26,
            SOURCESELW::TIMER2 => 27,
            SOURCESELW::TIMER3 => 28,
            SOURCESELW::TIMER4 => 29,
            SOURCESELW::TIMER5 => 30,
            SOURCESELW::TIMER6 => 31,
            SOURCESELW::WTIMER0 => 32,
            SOURCESELW::WTIMER1 => 33,
            SOURCESELW::WTIMER2 => 34,
            SOURCESELW::WTIMER3 => 35,
            SOURCESELW::MSC => 48,
            SOURCESELW::CRYPTO0 => 49,
            SOURCESELW::EBI => 50,
            SOURCESELW::CSEN => 61,
            SOURCESELW::LESENSE => 62,
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
    pub fn prs(self) -> &'a mut W {
        self.variant(SOURCESELW::PRS)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline]
    pub fn adc0(self) -> &'a mut W {
        self.variant(SOURCESELW::ADC0)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline]
    pub fn adc1(self) -> &'a mut W {
        self.variant(SOURCESELW::ADC1)
    }
    #[doc = "Digital to Analog Converter 0"]
    #[inline]
    pub fn vdac0(self) -> &'a mut W {
        self.variant(SOURCESELW::VDAC0)
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
    #[doc = "Low Energy UART 0"]
    #[inline]
    pub fn leuart0(self) -> &'a mut W {
        self.variant(SOURCESELW::LEUART0)
    }
    #[doc = "Low Energy UART 1"]
    #[inline]
    pub fn leuart1(self) -> &'a mut W {
        self.variant(SOURCESELW::LEUART1)
    }
    #[doc = "I2C 0"]
    #[inline]
    pub fn i2c0(self) -> &'a mut W {
        self.variant(SOURCESELW::I2C0)
    }
    #[doc = "I2C 1"]
    #[inline]
    pub fn i2c1(self) -> &'a mut W {
        self.variant(SOURCESELW::I2C1)
    }
    #[doc = "I2C 2"]
    #[inline]
    pub fn i2c2(self) -> &'a mut W {
        self.variant(SOURCESELW::I2C2)
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
    #[doc = "Timer 3"]
    #[inline]
    pub fn timer3(self) -> &'a mut W {
        self.variant(SOURCESELW::TIMER3)
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
    #[doc = "Memory System Controller"]
    #[inline]
    pub fn msc(self) -> &'a mut W {
        self.variant(SOURCESELW::MSC)
    }
    #[doc = "Advanced Encryption Standard Accelerator"]
    #[inline]
    pub fn crypto0(self) -> &'a mut W {
        self.variant(SOURCESELW::CRYPTO0)
    }
    #[doc = "External Bus Interface"]
    #[inline]
    pub fn ebi(self) -> &'a mut W {
        self.variant(SOURCESELW::EBI)
    }
    #[doc = "Capacitive touch sense module"]
    #[inline]
    pub fn csen(self) -> &'a mut W {
        self.variant(SOURCESELW::CSEN)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline]
    pub fn lesense(self) -> &'a mut W {
        self.variant(SOURCESELW::LESENSE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:3 - Signal Select"]
    #[inline]
    pub fn sigsel(&self) -> SIGSELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SIGSELR { bits }
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline]
    pub fn sourcesel(&self) -> SOURCESELR {
        SOURCESELR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:3 - Signal Select"]
    #[inline]
    pub fn sigsel(&mut self) -> _SIGSELW {
        _SIGSELW { w: self }
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline]
    pub fn sourcesel(&mut self) -> _SOURCESELW {
        _SOURCESELW { w: self }
    }
}
