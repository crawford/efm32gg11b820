#[doc = "Reader of register PPUFS"]
pub type R = crate::R<u32, super::PPUFS>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PERIPHID_A {
    #[doc = "0: Analog Comparator 0"]
    ACMP0 = 0,
    #[doc = "1: Analog Comparator 1"]
    ACMP1 = 1,
    #[doc = "2: Analog Comparator 1"]
    ACMP2 = 2,
    #[doc = "3: Analog Comparator 3"]
    ACMP3 = 3,
    #[doc = "4: Analog to Digital Converter 0"]
    ADC0 = 4,
    #[doc = "5: Analog to Digital Converter 0"]
    ADC1 = 5,
    #[doc = "6: CAN 0"]
    CAN0 = 6,
    #[doc = "7: CAN 1"]
    CAN1 = 7,
    #[doc = "8: Clock Management Unit"]
    CMU = 8,
    #[doc = "9: CryoTimer"]
    CRYOTIMER = 9,
    #[doc = "10: Advanced Encryption Standard Accelerator"]
    CRYPTO0 = 10,
    #[doc = "11: Capacitive touch sense module"]
    CSEN = 11,
    #[doc = "12: Digital to Analog Converter 0"]
    VDAC0 = 12,
    #[doc = "13: Peripheral Reflex System"]
    PRS = 13,
    #[doc = "14: External Bus Interface"]
    EBI = 14,
    #[doc = "15: Energy Management Unit"]
    EMU = 15,
    #[doc = "16: Ethernet Controller"]
    ETH = 16,
    #[doc = "17: FPU Exception Handler"]
    FPUEH = 17,
    #[doc = "18: General Purpose CRC"]
    GPCRC = 18,
    #[doc = "19: General purpose Input/Output"]
    GPIO = 19,
    #[doc = "20: I2C 0"]
    I2C0 = 20,
    #[doc = "21: I2C 1"]
    I2C1 = 21,
    #[doc = "22: I2C 2"]
    I2C2 = 22,
    #[doc = "23: Current Digital to Analog Converter 0"]
    IDAC0 = 23,
    #[doc = "24: Memory System Controller"]
    MSC = 24,
    #[doc = "25: Liquid Crystal Display Controller"]
    LCD = 25,
    #[doc = "26: Linked Direct Memory Access Controller"]
    LDMA = 26,
    #[doc = "27: Low Energy Sensor Interface"]
    LESENSE = 27,
    #[doc = "28: Low Energy Timer 0"]
    LETIMER0 = 28,
    #[doc = "29: Low Energy Timer 1"]
    LETIMER1 = 29,
    #[doc = "30: Low Energy UART 0"]
    LEUART0 = 30,
    #[doc = "31: Low Energy UART 1"]
    LEUART1 = 31,
    #[doc = "32: Pulse Counter 0"]
    PCNT0 = 32,
    #[doc = "33: Pulse Counter 1"]
    PCNT1 = 33,
    #[doc = "34: Pulse Counter 2"]
    PCNT2 = 34,
    #[doc = "35: Quad-SPI"]
    QSPI0 = 35,
    #[doc = "36: Reset Management Unit"]
    RMU = 36,
    #[doc = "37: Real-Time Counter"]
    RTC = 37,
    #[doc = "38: Real-Time Counter and Calendar"]
    RTCC = 38,
    #[doc = "39: SDIO Controller"]
    SDIO = 39,
    #[doc = "40: Security Management Unit"]
    SMU = 40,
    #[doc = "41: Timer 0"]
    TIMER0 = 41,
    #[doc = "42: Timer 1"]
    TIMER1 = 42,
    #[doc = "43: Timer 2"]
    TIMER2 = 43,
    #[doc = "44: Timer 3"]
    TIMER3 = 44,
    #[doc = "45: Timer 4"]
    TIMER4 = 45,
    #[doc = "46: Timer 5"]
    TIMER5 = 46,
    #[doc = "47: Timer 6"]
    TIMER6 = 47,
    #[doc = "48: True Random Number Generator 0"]
    TRNG0 = 48,
    #[doc = "49: Universal Asynchronous Receiver/Transmitter 0"]
    UART0 = 49,
    #[doc = "50: Universal Asynchronous Receiver/Transmitter 1"]
    UART1 = 50,
    #[doc = "51: Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    USART0 = 51,
    #[doc = "52: Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    USART1 = 52,
    #[doc = "53: Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    USART2 = 53,
    #[doc = "54: Universal Synchronous/Asynchronous Receiver/Transmitter 3"]
    USART3 = 54,
    #[doc = "55: Universal Synchronous/Asynchronous Receiver/Transmitter 4"]
    USART4 = 55,
    #[doc = "56: Universal Synchronous/Asynchronous Receiver/Transmitter 5"]
    USART5 = 56,
    #[doc = "57: Universal Serial Bus Interface"]
    USB = 57,
    #[doc = "58: Watchdog"]
    WDOG0 = 58,
    #[doc = "59: Watchdog"]
    WDOG1 = 59,
    #[doc = "60: Wide Timer 0"]
    WTIMER0 = 60,
    #[doc = "61: Wide Timer 0"]
    WTIMER1 = 61,
    #[doc = "62: Wide Timer 2"]
    WTIMER2 = 62,
    #[doc = "63: Wide Timer 3"]
    WTIMER3 = 63,
}
impl From<PERIPHID_A> for u8 {
    #[inline(always)]
    fn from(variant: PERIPHID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PERIPHID`"]
pub type PERIPHID_R = crate::R<u8, PERIPHID_A>;
impl PERIPHID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PERIPHID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PERIPHID_A::ACMP0),
            1 => Val(PERIPHID_A::ACMP1),
            2 => Val(PERIPHID_A::ACMP2),
            3 => Val(PERIPHID_A::ACMP3),
            4 => Val(PERIPHID_A::ADC0),
            5 => Val(PERIPHID_A::ADC1),
            6 => Val(PERIPHID_A::CAN0),
            7 => Val(PERIPHID_A::CAN1),
            8 => Val(PERIPHID_A::CMU),
            9 => Val(PERIPHID_A::CRYOTIMER),
            10 => Val(PERIPHID_A::CRYPTO0),
            11 => Val(PERIPHID_A::CSEN),
            12 => Val(PERIPHID_A::VDAC0),
            13 => Val(PERIPHID_A::PRS),
            14 => Val(PERIPHID_A::EBI),
            15 => Val(PERIPHID_A::EMU),
            16 => Val(PERIPHID_A::ETH),
            17 => Val(PERIPHID_A::FPUEH),
            18 => Val(PERIPHID_A::GPCRC),
            19 => Val(PERIPHID_A::GPIO),
            20 => Val(PERIPHID_A::I2C0),
            21 => Val(PERIPHID_A::I2C1),
            22 => Val(PERIPHID_A::I2C2),
            23 => Val(PERIPHID_A::IDAC0),
            24 => Val(PERIPHID_A::MSC),
            25 => Val(PERIPHID_A::LCD),
            26 => Val(PERIPHID_A::LDMA),
            27 => Val(PERIPHID_A::LESENSE),
            28 => Val(PERIPHID_A::LETIMER0),
            29 => Val(PERIPHID_A::LETIMER1),
            30 => Val(PERIPHID_A::LEUART0),
            31 => Val(PERIPHID_A::LEUART1),
            32 => Val(PERIPHID_A::PCNT0),
            33 => Val(PERIPHID_A::PCNT1),
            34 => Val(PERIPHID_A::PCNT2),
            35 => Val(PERIPHID_A::QSPI0),
            36 => Val(PERIPHID_A::RMU),
            37 => Val(PERIPHID_A::RTC),
            38 => Val(PERIPHID_A::RTCC),
            39 => Val(PERIPHID_A::SDIO),
            40 => Val(PERIPHID_A::SMU),
            41 => Val(PERIPHID_A::TIMER0),
            42 => Val(PERIPHID_A::TIMER1),
            43 => Val(PERIPHID_A::TIMER2),
            44 => Val(PERIPHID_A::TIMER3),
            45 => Val(PERIPHID_A::TIMER4),
            46 => Val(PERIPHID_A::TIMER5),
            47 => Val(PERIPHID_A::TIMER6),
            48 => Val(PERIPHID_A::TRNG0),
            49 => Val(PERIPHID_A::UART0),
            50 => Val(PERIPHID_A::UART1),
            51 => Val(PERIPHID_A::USART0),
            52 => Val(PERIPHID_A::USART1),
            53 => Val(PERIPHID_A::USART2),
            54 => Val(PERIPHID_A::USART3),
            55 => Val(PERIPHID_A::USART4),
            56 => Val(PERIPHID_A::USART5),
            57 => Val(PERIPHID_A::USB),
            58 => Val(PERIPHID_A::WDOG0),
            59 => Val(PERIPHID_A::WDOG1),
            60 => Val(PERIPHID_A::WTIMER0),
            61 => Val(PERIPHID_A::WTIMER1),
            62 => Val(PERIPHID_A::WTIMER2),
            63 => Val(PERIPHID_A::WTIMER3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ACMP0`"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        *self == PERIPHID_A::ACMP0
    }
    #[doc = "Checks if the value of the field is `ACMP1`"]
    #[inline(always)]
    pub fn is_acmp1(&self) -> bool {
        *self == PERIPHID_A::ACMP1
    }
    #[doc = "Checks if the value of the field is `ACMP2`"]
    #[inline(always)]
    pub fn is_acmp2(&self) -> bool {
        *self == PERIPHID_A::ACMP2
    }
    #[doc = "Checks if the value of the field is `ACMP3`"]
    #[inline(always)]
    pub fn is_acmp3(&self) -> bool {
        *self == PERIPHID_A::ACMP3
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == PERIPHID_A::ADC0
    }
    #[doc = "Checks if the value of the field is `ADC1`"]
    #[inline(always)]
    pub fn is_adc1(&self) -> bool {
        *self == PERIPHID_A::ADC1
    }
    #[doc = "Checks if the value of the field is `CAN0`"]
    #[inline(always)]
    pub fn is_can0(&self) -> bool {
        *self == PERIPHID_A::CAN0
    }
    #[doc = "Checks if the value of the field is `CAN1`"]
    #[inline(always)]
    pub fn is_can1(&self) -> bool {
        *self == PERIPHID_A::CAN1
    }
    #[doc = "Checks if the value of the field is `CMU`"]
    #[inline(always)]
    pub fn is_cmu(&self) -> bool {
        *self == PERIPHID_A::CMU
    }
    #[doc = "Checks if the value of the field is `CRYOTIMER`"]
    #[inline(always)]
    pub fn is_cryotimer(&self) -> bool {
        *self == PERIPHID_A::CRYOTIMER
    }
    #[doc = "Checks if the value of the field is `CRYPTO0`"]
    #[inline(always)]
    pub fn is_crypto0(&self) -> bool {
        *self == PERIPHID_A::CRYPTO0
    }
    #[doc = "Checks if the value of the field is `CSEN`"]
    #[inline(always)]
    pub fn is_csen(&self) -> bool {
        *self == PERIPHID_A::CSEN
    }
    #[doc = "Checks if the value of the field is `VDAC0`"]
    #[inline(always)]
    pub fn is_vdac0(&self) -> bool {
        *self == PERIPHID_A::VDAC0
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == PERIPHID_A::PRS
    }
    #[doc = "Checks if the value of the field is `EBI`"]
    #[inline(always)]
    pub fn is_ebi(&self) -> bool {
        *self == PERIPHID_A::EBI
    }
    #[doc = "Checks if the value of the field is `EMU`"]
    #[inline(always)]
    pub fn is_emu(&self) -> bool {
        *self == PERIPHID_A::EMU
    }
    #[doc = "Checks if the value of the field is `ETH`"]
    #[inline(always)]
    pub fn is_eth(&self) -> bool {
        *self == PERIPHID_A::ETH
    }
    #[doc = "Checks if the value of the field is `FPUEH`"]
    #[inline(always)]
    pub fn is_fpueh(&self) -> bool {
        *self == PERIPHID_A::FPUEH
    }
    #[doc = "Checks if the value of the field is `GPCRC`"]
    #[inline(always)]
    pub fn is_gpcrc(&self) -> bool {
        *self == PERIPHID_A::GPCRC
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == PERIPHID_A::GPIO
    }
    #[doc = "Checks if the value of the field is `I2C0`"]
    #[inline(always)]
    pub fn is_i2c0(&self) -> bool {
        *self == PERIPHID_A::I2C0
    }
    #[doc = "Checks if the value of the field is `I2C1`"]
    #[inline(always)]
    pub fn is_i2c1(&self) -> bool {
        *self == PERIPHID_A::I2C1
    }
    #[doc = "Checks if the value of the field is `I2C2`"]
    #[inline(always)]
    pub fn is_i2c2(&self) -> bool {
        *self == PERIPHID_A::I2C2
    }
    #[doc = "Checks if the value of the field is `IDAC0`"]
    #[inline(always)]
    pub fn is_idac0(&self) -> bool {
        *self == PERIPHID_A::IDAC0
    }
    #[doc = "Checks if the value of the field is `MSC`"]
    #[inline(always)]
    pub fn is_msc(&self) -> bool {
        *self == PERIPHID_A::MSC
    }
    #[doc = "Checks if the value of the field is `LCD`"]
    #[inline(always)]
    pub fn is_lcd(&self) -> bool {
        *self == PERIPHID_A::LCD
    }
    #[doc = "Checks if the value of the field is `LDMA`"]
    #[inline(always)]
    pub fn is_ldma(&self) -> bool {
        *self == PERIPHID_A::LDMA
    }
    #[doc = "Checks if the value of the field is `LESENSE`"]
    #[inline(always)]
    pub fn is_lesense(&self) -> bool {
        *self == PERIPHID_A::LESENSE
    }
    #[doc = "Checks if the value of the field is `LETIMER0`"]
    #[inline(always)]
    pub fn is_letimer0(&self) -> bool {
        *self == PERIPHID_A::LETIMER0
    }
    #[doc = "Checks if the value of the field is `LETIMER1`"]
    #[inline(always)]
    pub fn is_letimer1(&self) -> bool {
        *self == PERIPHID_A::LETIMER1
    }
    #[doc = "Checks if the value of the field is `LEUART0`"]
    #[inline(always)]
    pub fn is_leuart0(&self) -> bool {
        *self == PERIPHID_A::LEUART0
    }
    #[doc = "Checks if the value of the field is `LEUART1`"]
    #[inline(always)]
    pub fn is_leuart1(&self) -> bool {
        *self == PERIPHID_A::LEUART1
    }
    #[doc = "Checks if the value of the field is `PCNT0`"]
    #[inline(always)]
    pub fn is_pcnt0(&self) -> bool {
        *self == PERIPHID_A::PCNT0
    }
    #[doc = "Checks if the value of the field is `PCNT1`"]
    #[inline(always)]
    pub fn is_pcnt1(&self) -> bool {
        *self == PERIPHID_A::PCNT1
    }
    #[doc = "Checks if the value of the field is `PCNT2`"]
    #[inline(always)]
    pub fn is_pcnt2(&self) -> bool {
        *self == PERIPHID_A::PCNT2
    }
    #[doc = "Checks if the value of the field is `QSPI0`"]
    #[inline(always)]
    pub fn is_qspi0(&self) -> bool {
        *self == PERIPHID_A::QSPI0
    }
    #[doc = "Checks if the value of the field is `RMU`"]
    #[inline(always)]
    pub fn is_rmu(&self) -> bool {
        *self == PERIPHID_A::RMU
    }
    #[doc = "Checks if the value of the field is `RTC`"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == PERIPHID_A::RTC
    }
    #[doc = "Checks if the value of the field is `RTCC`"]
    #[inline(always)]
    pub fn is_rtcc(&self) -> bool {
        *self == PERIPHID_A::RTCC
    }
    #[doc = "Checks if the value of the field is `SDIO`"]
    #[inline(always)]
    pub fn is_sdio(&self) -> bool {
        *self == PERIPHID_A::SDIO
    }
    #[doc = "Checks if the value of the field is `SMU`"]
    #[inline(always)]
    pub fn is_smu(&self) -> bool {
        *self == PERIPHID_A::SMU
    }
    #[doc = "Checks if the value of the field is `TIMER0`"]
    #[inline(always)]
    pub fn is_timer0(&self) -> bool {
        *self == PERIPHID_A::TIMER0
    }
    #[doc = "Checks if the value of the field is `TIMER1`"]
    #[inline(always)]
    pub fn is_timer1(&self) -> bool {
        *self == PERIPHID_A::TIMER1
    }
    #[doc = "Checks if the value of the field is `TIMER2`"]
    #[inline(always)]
    pub fn is_timer2(&self) -> bool {
        *self == PERIPHID_A::TIMER2
    }
    #[doc = "Checks if the value of the field is `TIMER3`"]
    #[inline(always)]
    pub fn is_timer3(&self) -> bool {
        *self == PERIPHID_A::TIMER3
    }
    #[doc = "Checks if the value of the field is `TIMER4`"]
    #[inline(always)]
    pub fn is_timer4(&self) -> bool {
        *self == PERIPHID_A::TIMER4
    }
    #[doc = "Checks if the value of the field is `TIMER5`"]
    #[inline(always)]
    pub fn is_timer5(&self) -> bool {
        *self == PERIPHID_A::TIMER5
    }
    #[doc = "Checks if the value of the field is `TIMER6`"]
    #[inline(always)]
    pub fn is_timer6(&self) -> bool {
        *self == PERIPHID_A::TIMER6
    }
    #[doc = "Checks if the value of the field is `TRNG0`"]
    #[inline(always)]
    pub fn is_trng0(&self) -> bool {
        *self == PERIPHID_A::TRNG0
    }
    #[doc = "Checks if the value of the field is `UART0`"]
    #[inline(always)]
    pub fn is_uart0(&self) -> bool {
        *self == PERIPHID_A::UART0
    }
    #[doc = "Checks if the value of the field is `UART1`"]
    #[inline(always)]
    pub fn is_uart1(&self) -> bool {
        *self == PERIPHID_A::UART1
    }
    #[doc = "Checks if the value of the field is `USART0`"]
    #[inline(always)]
    pub fn is_usart0(&self) -> bool {
        *self == PERIPHID_A::USART0
    }
    #[doc = "Checks if the value of the field is `USART1`"]
    #[inline(always)]
    pub fn is_usart1(&self) -> bool {
        *self == PERIPHID_A::USART1
    }
    #[doc = "Checks if the value of the field is `USART2`"]
    #[inline(always)]
    pub fn is_usart2(&self) -> bool {
        *self == PERIPHID_A::USART2
    }
    #[doc = "Checks if the value of the field is `USART3`"]
    #[inline(always)]
    pub fn is_usart3(&self) -> bool {
        *self == PERIPHID_A::USART3
    }
    #[doc = "Checks if the value of the field is `USART4`"]
    #[inline(always)]
    pub fn is_usart4(&self) -> bool {
        *self == PERIPHID_A::USART4
    }
    #[doc = "Checks if the value of the field is `USART5`"]
    #[inline(always)]
    pub fn is_usart5(&self) -> bool {
        *self == PERIPHID_A::USART5
    }
    #[doc = "Checks if the value of the field is `USB`"]
    #[inline(always)]
    pub fn is_usb(&self) -> bool {
        *self == PERIPHID_A::USB
    }
    #[doc = "Checks if the value of the field is `WDOG0`"]
    #[inline(always)]
    pub fn is_wdog0(&self) -> bool {
        *self == PERIPHID_A::WDOG0
    }
    #[doc = "Checks if the value of the field is `WDOG1`"]
    #[inline(always)]
    pub fn is_wdog1(&self) -> bool {
        *self == PERIPHID_A::WDOG1
    }
    #[doc = "Checks if the value of the field is `WTIMER0`"]
    #[inline(always)]
    pub fn is_wtimer0(&self) -> bool {
        *self == PERIPHID_A::WTIMER0
    }
    #[doc = "Checks if the value of the field is `WTIMER1`"]
    #[inline(always)]
    pub fn is_wtimer1(&self) -> bool {
        *self == PERIPHID_A::WTIMER1
    }
    #[doc = "Checks if the value of the field is `WTIMER2`"]
    #[inline(always)]
    pub fn is_wtimer2(&self) -> bool {
        *self == PERIPHID_A::WTIMER2
    }
    #[doc = "Checks if the value of the field is `WTIMER3`"]
    #[inline(always)]
    pub fn is_wtimer3(&self) -> bool {
        *self == PERIPHID_A::WTIMER3
    }
}
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn periphid(&self) -> PERIPHID_R {
        PERIPHID_R::new((self.bits & 0x7f) as u8)
    }
}
