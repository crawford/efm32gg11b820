#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PPUFS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `PERIPHID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIPHIDR {
    #[doc = "Analog Comparator 0"]
    ACMP0,
    #[doc = "Analog Comparator 1"]
    ACMP1,
    #[doc = "Analog Comparator 1"]
    ACMP2,
    #[doc = "Analog Comparator 3"]
    ACMP3,
    #[doc = "Analog to Digital Converter 0"]
    ADC0,
    #[doc = "Analog to Digital Converter 0"]
    ADC1,
    #[doc = "CAN 0"]
    CAN0,
    #[doc = "CAN 1"]
    CAN1,
    #[doc = "Clock Management Unit"]
    CMU,
    #[doc = "CryoTimer"]
    CRYOTIMER,
    #[doc = "Advanced Encryption Standard Accelerator"]
    CRYPTO0,
    #[doc = "Capacitive touch sense module"]
    CSEN,
    #[doc = "Digital to Analog Converter 0"]
    VDAC0,
    #[doc = "Peripheral Reflex System"]
    PRS,
    #[doc = "External Bus Interface"]
    EBI,
    #[doc = "Energy Management Unit"]
    EMU,
    #[doc = "Ethernet Controller"]
    ETH,
    #[doc = "FPU Exception Handler"]
    FPUEH,
    #[doc = "General Purpose CRC"]
    GPCRC,
    #[doc = "General purpose Input/Output"]
    GPIO,
    #[doc = "I2C 0"]
    I2C0,
    #[doc = "I2C 1"]
    I2C1,
    #[doc = "I2C 2"]
    I2C2,
    #[doc = "Current Digital to Analog Converter 0"]
    IDAC0,
    #[doc = "Memory System Controller"]
    MSC,
    #[doc = "Liquid Crystal Display Controller"]
    LCD,
    #[doc = "Linked Direct Memory Access Controller"]
    LDMA,
    #[doc = "Low Energy Sensor Interface"]
    LESENSE,
    #[doc = "Low Energy Timer 0"]
    LETIMER0,
    #[doc = "Low Energy Timer 1"]
    LETIMER1,
    #[doc = "Low Energy UART 0"]
    LEUART0,
    #[doc = "Low Energy UART 1"]
    LEUART1,
    #[doc = "Pulse Counter 0"]
    PCNT0,
    #[doc = "Pulse Counter 1"]
    PCNT1,
    #[doc = "Pulse Counter 2"]
    PCNT2,
    #[doc = "Quad-SPI"]
    QSPI0,
    #[doc = "Reset Management Unit"]
    RMU,
    #[doc = "Real-Time Counter"]
    RTC,
    #[doc = "Real-Time Counter and Calendar"]
    RTCC,
    #[doc = "SDIO Controller"]
    SDIO,
    #[doc = "Security Management Unit"]
    SMU,
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
    #[doc = "True Random Number Generator 0"]
    TRNG0,
    #[doc = "Universal Asynchronous Receiver/Transmitter 0"]
    UART0,
    #[doc = "Universal Asynchronous Receiver/Transmitter 1"]
    UART1,
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
    #[doc = "Universal Serial Bus Interface"]
    USB,
    #[doc = "Watchdog"]
    WDOG0,
    #[doc = "Watchdog"]
    WDOG1,
    #[doc = "Wide Timer 0"]
    WTIMER0,
    #[doc = "Wide Timer 0"]
    WTIMER1,
    #[doc = "Wide Timer 2"]
    WTIMER2,
    #[doc = "Wide Timer 3"]
    WTIMER3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PERIPHIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PERIPHIDR::ACMP0 => 0,
            PERIPHIDR::ACMP1 => 1,
            PERIPHIDR::ACMP2 => 2,
            PERIPHIDR::ACMP3 => 3,
            PERIPHIDR::ADC0 => 4,
            PERIPHIDR::ADC1 => 5,
            PERIPHIDR::CAN0 => 6,
            PERIPHIDR::CAN1 => 7,
            PERIPHIDR::CMU => 8,
            PERIPHIDR::CRYOTIMER => 9,
            PERIPHIDR::CRYPTO0 => 10,
            PERIPHIDR::CSEN => 11,
            PERIPHIDR::VDAC0 => 12,
            PERIPHIDR::PRS => 13,
            PERIPHIDR::EBI => 14,
            PERIPHIDR::EMU => 15,
            PERIPHIDR::ETH => 16,
            PERIPHIDR::FPUEH => 17,
            PERIPHIDR::GPCRC => 18,
            PERIPHIDR::GPIO => 19,
            PERIPHIDR::I2C0 => 20,
            PERIPHIDR::I2C1 => 21,
            PERIPHIDR::I2C2 => 22,
            PERIPHIDR::IDAC0 => 23,
            PERIPHIDR::MSC => 24,
            PERIPHIDR::LCD => 25,
            PERIPHIDR::LDMA => 26,
            PERIPHIDR::LESENSE => 27,
            PERIPHIDR::LETIMER0 => 28,
            PERIPHIDR::LETIMER1 => 29,
            PERIPHIDR::LEUART0 => 30,
            PERIPHIDR::LEUART1 => 31,
            PERIPHIDR::PCNT0 => 32,
            PERIPHIDR::PCNT1 => 33,
            PERIPHIDR::PCNT2 => 34,
            PERIPHIDR::QSPI0 => 35,
            PERIPHIDR::RMU => 36,
            PERIPHIDR::RTC => 37,
            PERIPHIDR::RTCC => 38,
            PERIPHIDR::SDIO => 39,
            PERIPHIDR::SMU => 40,
            PERIPHIDR::TIMER0 => 41,
            PERIPHIDR::TIMER1 => 42,
            PERIPHIDR::TIMER2 => 43,
            PERIPHIDR::TIMER3 => 44,
            PERIPHIDR::TIMER4 => 45,
            PERIPHIDR::TIMER5 => 46,
            PERIPHIDR::TIMER6 => 47,
            PERIPHIDR::TRNG0 => 48,
            PERIPHIDR::UART0 => 49,
            PERIPHIDR::UART1 => 50,
            PERIPHIDR::USART0 => 51,
            PERIPHIDR::USART1 => 52,
            PERIPHIDR::USART2 => 53,
            PERIPHIDR::USART3 => 54,
            PERIPHIDR::USART4 => 55,
            PERIPHIDR::USART5 => 56,
            PERIPHIDR::USB => 57,
            PERIPHIDR::WDOG0 => 58,
            PERIPHIDR::WDOG1 => 59,
            PERIPHIDR::WTIMER0 => 60,
            PERIPHIDR::WTIMER1 => 61,
            PERIPHIDR::WTIMER2 => 62,
            PERIPHIDR::WTIMER3 => 63,
            PERIPHIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PERIPHIDR {
        match value {
            0 => PERIPHIDR::ACMP0,
            1 => PERIPHIDR::ACMP1,
            2 => PERIPHIDR::ACMP2,
            3 => PERIPHIDR::ACMP3,
            4 => PERIPHIDR::ADC0,
            5 => PERIPHIDR::ADC1,
            6 => PERIPHIDR::CAN0,
            7 => PERIPHIDR::CAN1,
            8 => PERIPHIDR::CMU,
            9 => PERIPHIDR::CRYOTIMER,
            10 => PERIPHIDR::CRYPTO0,
            11 => PERIPHIDR::CSEN,
            12 => PERIPHIDR::VDAC0,
            13 => PERIPHIDR::PRS,
            14 => PERIPHIDR::EBI,
            15 => PERIPHIDR::EMU,
            16 => PERIPHIDR::ETH,
            17 => PERIPHIDR::FPUEH,
            18 => PERIPHIDR::GPCRC,
            19 => PERIPHIDR::GPIO,
            20 => PERIPHIDR::I2C0,
            21 => PERIPHIDR::I2C1,
            22 => PERIPHIDR::I2C2,
            23 => PERIPHIDR::IDAC0,
            24 => PERIPHIDR::MSC,
            25 => PERIPHIDR::LCD,
            26 => PERIPHIDR::LDMA,
            27 => PERIPHIDR::LESENSE,
            28 => PERIPHIDR::LETIMER0,
            29 => PERIPHIDR::LETIMER1,
            30 => PERIPHIDR::LEUART0,
            31 => PERIPHIDR::LEUART1,
            32 => PERIPHIDR::PCNT0,
            33 => PERIPHIDR::PCNT1,
            34 => PERIPHIDR::PCNT2,
            35 => PERIPHIDR::QSPI0,
            36 => PERIPHIDR::RMU,
            37 => PERIPHIDR::RTC,
            38 => PERIPHIDR::RTCC,
            39 => PERIPHIDR::SDIO,
            40 => PERIPHIDR::SMU,
            41 => PERIPHIDR::TIMER0,
            42 => PERIPHIDR::TIMER1,
            43 => PERIPHIDR::TIMER2,
            44 => PERIPHIDR::TIMER3,
            45 => PERIPHIDR::TIMER4,
            46 => PERIPHIDR::TIMER5,
            47 => PERIPHIDR::TIMER6,
            48 => PERIPHIDR::TRNG0,
            49 => PERIPHIDR::UART0,
            50 => PERIPHIDR::UART1,
            51 => PERIPHIDR::USART0,
            52 => PERIPHIDR::USART1,
            53 => PERIPHIDR::USART2,
            54 => PERIPHIDR::USART3,
            55 => PERIPHIDR::USART4,
            56 => PERIPHIDR::USART5,
            57 => PERIPHIDR::USB,
            58 => PERIPHIDR::WDOG0,
            59 => PERIPHIDR::WDOG1,
            60 => PERIPHIDR::WTIMER0,
            61 => PERIPHIDR::WTIMER1,
            62 => PERIPHIDR::WTIMER2,
            63 => PERIPHIDR::WTIMER3,
            i => PERIPHIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ACMP0`"]
    #[inline]
    pub fn is_acmp0(&self) -> bool {
        *self == PERIPHIDR::ACMP0
    }
    #[doc = "Checks if the value of the field is `ACMP1`"]
    #[inline]
    pub fn is_acmp1(&self) -> bool {
        *self == PERIPHIDR::ACMP1
    }
    #[doc = "Checks if the value of the field is `ACMP2`"]
    #[inline]
    pub fn is_acmp2(&self) -> bool {
        *self == PERIPHIDR::ACMP2
    }
    #[doc = "Checks if the value of the field is `ACMP3`"]
    #[inline]
    pub fn is_acmp3(&self) -> bool {
        *self == PERIPHIDR::ACMP3
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline]
    pub fn is_adc0(&self) -> bool {
        *self == PERIPHIDR::ADC0
    }
    #[doc = "Checks if the value of the field is `ADC1`"]
    #[inline]
    pub fn is_adc1(&self) -> bool {
        *self == PERIPHIDR::ADC1
    }
    #[doc = "Checks if the value of the field is `CAN0`"]
    #[inline]
    pub fn is_can0(&self) -> bool {
        *self == PERIPHIDR::CAN0
    }
    #[doc = "Checks if the value of the field is `CAN1`"]
    #[inline]
    pub fn is_can1(&self) -> bool {
        *self == PERIPHIDR::CAN1
    }
    #[doc = "Checks if the value of the field is `CMU`"]
    #[inline]
    pub fn is_cmu(&self) -> bool {
        *self == PERIPHIDR::CMU
    }
    #[doc = "Checks if the value of the field is `CRYOTIMER`"]
    #[inline]
    pub fn is_cryotimer(&self) -> bool {
        *self == PERIPHIDR::CRYOTIMER
    }
    #[doc = "Checks if the value of the field is `CRYPTO0`"]
    #[inline]
    pub fn is_crypto0(&self) -> bool {
        *self == PERIPHIDR::CRYPTO0
    }
    #[doc = "Checks if the value of the field is `CSEN`"]
    #[inline]
    pub fn is_csen(&self) -> bool {
        *self == PERIPHIDR::CSEN
    }
    #[doc = "Checks if the value of the field is `VDAC0`"]
    #[inline]
    pub fn is_vdac0(&self) -> bool {
        *self == PERIPHIDR::VDAC0
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline]
    pub fn is_prs(&self) -> bool {
        *self == PERIPHIDR::PRS
    }
    #[doc = "Checks if the value of the field is `EBI`"]
    #[inline]
    pub fn is_ebi(&self) -> bool {
        *self == PERIPHIDR::EBI
    }
    #[doc = "Checks if the value of the field is `EMU`"]
    #[inline]
    pub fn is_emu(&self) -> bool {
        *self == PERIPHIDR::EMU
    }
    #[doc = "Checks if the value of the field is `ETH`"]
    #[inline]
    pub fn is_eth(&self) -> bool {
        *self == PERIPHIDR::ETH
    }
    #[doc = "Checks if the value of the field is `FPUEH`"]
    #[inline]
    pub fn is_fpueh(&self) -> bool {
        *self == PERIPHIDR::FPUEH
    }
    #[doc = "Checks if the value of the field is `GPCRC`"]
    #[inline]
    pub fn is_gpcrc(&self) -> bool {
        *self == PERIPHIDR::GPCRC
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline]
    pub fn is_gpio(&self) -> bool {
        *self == PERIPHIDR::GPIO
    }
    #[doc = "Checks if the value of the field is `I2C0`"]
    #[inline]
    pub fn is_i2c0(&self) -> bool {
        *self == PERIPHIDR::I2C0
    }
    #[doc = "Checks if the value of the field is `I2C1`"]
    #[inline]
    pub fn is_i2c1(&self) -> bool {
        *self == PERIPHIDR::I2C1
    }
    #[doc = "Checks if the value of the field is `I2C2`"]
    #[inline]
    pub fn is_i2c2(&self) -> bool {
        *self == PERIPHIDR::I2C2
    }
    #[doc = "Checks if the value of the field is `IDAC0`"]
    #[inline]
    pub fn is_idac0(&self) -> bool {
        *self == PERIPHIDR::IDAC0
    }
    #[doc = "Checks if the value of the field is `MSC`"]
    #[inline]
    pub fn is_msc(&self) -> bool {
        *self == PERIPHIDR::MSC
    }
    #[doc = "Checks if the value of the field is `LCD`"]
    #[inline]
    pub fn is_lcd(&self) -> bool {
        *self == PERIPHIDR::LCD
    }
    #[doc = "Checks if the value of the field is `LDMA`"]
    #[inline]
    pub fn is_ldma(&self) -> bool {
        *self == PERIPHIDR::LDMA
    }
    #[doc = "Checks if the value of the field is `LESENSE`"]
    #[inline]
    pub fn is_lesense(&self) -> bool {
        *self == PERIPHIDR::LESENSE
    }
    #[doc = "Checks if the value of the field is `LETIMER0`"]
    #[inline]
    pub fn is_letimer0(&self) -> bool {
        *self == PERIPHIDR::LETIMER0
    }
    #[doc = "Checks if the value of the field is `LETIMER1`"]
    #[inline]
    pub fn is_letimer1(&self) -> bool {
        *self == PERIPHIDR::LETIMER1
    }
    #[doc = "Checks if the value of the field is `LEUART0`"]
    #[inline]
    pub fn is_leuart0(&self) -> bool {
        *self == PERIPHIDR::LEUART0
    }
    #[doc = "Checks if the value of the field is `LEUART1`"]
    #[inline]
    pub fn is_leuart1(&self) -> bool {
        *self == PERIPHIDR::LEUART1
    }
    #[doc = "Checks if the value of the field is `PCNT0`"]
    #[inline]
    pub fn is_pcnt0(&self) -> bool {
        *self == PERIPHIDR::PCNT0
    }
    #[doc = "Checks if the value of the field is `PCNT1`"]
    #[inline]
    pub fn is_pcnt1(&self) -> bool {
        *self == PERIPHIDR::PCNT1
    }
    #[doc = "Checks if the value of the field is `PCNT2`"]
    #[inline]
    pub fn is_pcnt2(&self) -> bool {
        *self == PERIPHIDR::PCNT2
    }
    #[doc = "Checks if the value of the field is `QSPI0`"]
    #[inline]
    pub fn is_qspi0(&self) -> bool {
        *self == PERIPHIDR::QSPI0
    }
    #[doc = "Checks if the value of the field is `RMU`"]
    #[inline]
    pub fn is_rmu(&self) -> bool {
        *self == PERIPHIDR::RMU
    }
    #[doc = "Checks if the value of the field is `RTC`"]
    #[inline]
    pub fn is_rtc(&self) -> bool {
        *self == PERIPHIDR::RTC
    }
    #[doc = "Checks if the value of the field is `RTCC`"]
    #[inline]
    pub fn is_rtcc(&self) -> bool {
        *self == PERIPHIDR::RTCC
    }
    #[doc = "Checks if the value of the field is `SDIO`"]
    #[inline]
    pub fn is_sdio(&self) -> bool {
        *self == PERIPHIDR::SDIO
    }
    #[doc = "Checks if the value of the field is `SMU`"]
    #[inline]
    pub fn is_smu(&self) -> bool {
        *self == PERIPHIDR::SMU
    }
    #[doc = "Checks if the value of the field is `TIMER0`"]
    #[inline]
    pub fn is_timer0(&self) -> bool {
        *self == PERIPHIDR::TIMER0
    }
    #[doc = "Checks if the value of the field is `TIMER1`"]
    #[inline]
    pub fn is_timer1(&self) -> bool {
        *self == PERIPHIDR::TIMER1
    }
    #[doc = "Checks if the value of the field is `TIMER2`"]
    #[inline]
    pub fn is_timer2(&self) -> bool {
        *self == PERIPHIDR::TIMER2
    }
    #[doc = "Checks if the value of the field is `TIMER3`"]
    #[inline]
    pub fn is_timer3(&self) -> bool {
        *self == PERIPHIDR::TIMER3
    }
    #[doc = "Checks if the value of the field is `TIMER4`"]
    #[inline]
    pub fn is_timer4(&self) -> bool {
        *self == PERIPHIDR::TIMER4
    }
    #[doc = "Checks if the value of the field is `TIMER5`"]
    #[inline]
    pub fn is_timer5(&self) -> bool {
        *self == PERIPHIDR::TIMER5
    }
    #[doc = "Checks if the value of the field is `TIMER6`"]
    #[inline]
    pub fn is_timer6(&self) -> bool {
        *self == PERIPHIDR::TIMER6
    }
    #[doc = "Checks if the value of the field is `TRNG0`"]
    #[inline]
    pub fn is_trng0(&self) -> bool {
        *self == PERIPHIDR::TRNG0
    }
    #[doc = "Checks if the value of the field is `UART0`"]
    #[inline]
    pub fn is_uart0(&self) -> bool {
        *self == PERIPHIDR::UART0
    }
    #[doc = "Checks if the value of the field is `UART1`"]
    #[inline]
    pub fn is_uart1(&self) -> bool {
        *self == PERIPHIDR::UART1
    }
    #[doc = "Checks if the value of the field is `USART0`"]
    #[inline]
    pub fn is_usart0(&self) -> bool {
        *self == PERIPHIDR::USART0
    }
    #[doc = "Checks if the value of the field is `USART1`"]
    #[inline]
    pub fn is_usart1(&self) -> bool {
        *self == PERIPHIDR::USART1
    }
    #[doc = "Checks if the value of the field is `USART2`"]
    #[inline]
    pub fn is_usart2(&self) -> bool {
        *self == PERIPHIDR::USART2
    }
    #[doc = "Checks if the value of the field is `USART3`"]
    #[inline]
    pub fn is_usart3(&self) -> bool {
        *self == PERIPHIDR::USART3
    }
    #[doc = "Checks if the value of the field is `USART4`"]
    #[inline]
    pub fn is_usart4(&self) -> bool {
        *self == PERIPHIDR::USART4
    }
    #[doc = "Checks if the value of the field is `USART5`"]
    #[inline]
    pub fn is_usart5(&self) -> bool {
        *self == PERIPHIDR::USART5
    }
    #[doc = "Checks if the value of the field is `USB`"]
    #[inline]
    pub fn is_usb(&self) -> bool {
        *self == PERIPHIDR::USB
    }
    #[doc = "Checks if the value of the field is `WDOG0`"]
    #[inline]
    pub fn is_wdog0(&self) -> bool {
        *self == PERIPHIDR::WDOG0
    }
    #[doc = "Checks if the value of the field is `WDOG1`"]
    #[inline]
    pub fn is_wdog1(&self) -> bool {
        *self == PERIPHIDR::WDOG1
    }
    #[doc = "Checks if the value of the field is `WTIMER0`"]
    #[inline]
    pub fn is_wtimer0(&self) -> bool {
        *self == PERIPHIDR::WTIMER0
    }
    #[doc = "Checks if the value of the field is `WTIMER1`"]
    #[inline]
    pub fn is_wtimer1(&self) -> bool {
        *self == PERIPHIDR::WTIMER1
    }
    #[doc = "Checks if the value of the field is `WTIMER2`"]
    #[inline]
    pub fn is_wtimer2(&self) -> bool {
        *self == PERIPHIDR::WTIMER2
    }
    #[doc = "Checks if the value of the field is `WTIMER3`"]
    #[inline]
    pub fn is_wtimer3(&self) -> bool {
        *self == PERIPHIDR::WTIMER3
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - \"\""]
    #[inline]
    pub fn periphid(&self) -> PERIPHIDR {
        PERIPHIDR::_from({
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
