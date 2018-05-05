use bare_metal::Nr;
#[cfg(feature = "rt")]
extern "C" {
    fn DEFAULT_HANDLER();
}
#[cfg(feature = "rt")]
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn DH_TRAMPOLINE() {
    DEFAULT_HANDLER();
}
#[cfg(feature = "rt")]
global_asm ! ( "\n.weak EMU\nEMU = DH_TRAMPOLINE\n.weak WDOG0\nWDOG0 = DH_TRAMPOLINE\n.weak LDMA\nLDMA = DH_TRAMPOLINE\n.weak GPIO_EVEN\nGPIO_EVEN = DH_TRAMPOLINE\n.weak SMU\nSMU = DH_TRAMPOLINE\n.weak TIMER0\nTIMER0 = DH_TRAMPOLINE\n.weak USART0_RX\nUSART0_RX = DH_TRAMPOLINE\n.weak USART0_TX\nUSART0_TX = DH_TRAMPOLINE\n.weak ACMP0\nACMP0 = DH_TRAMPOLINE\n.weak ADC0\nADC0 = DH_TRAMPOLINE\n.weak IDAC0\nIDAC0 = DH_TRAMPOLINE\n.weak I2C0\nI2C0 = DH_TRAMPOLINE\n.weak I2C1\nI2C1 = DH_TRAMPOLINE\n.weak GPIO_ODD\nGPIO_ODD = DH_TRAMPOLINE\n.weak TIMER1\nTIMER1 = DH_TRAMPOLINE\n.weak TIMER2\nTIMER2 = DH_TRAMPOLINE\n.weak TIMER3\nTIMER3 = DH_TRAMPOLINE\n.weak USART1_RX\nUSART1_RX = DH_TRAMPOLINE\n.weak USART1_TX\nUSART1_TX = DH_TRAMPOLINE\n.weak USART2_RX\nUSART2_RX = DH_TRAMPOLINE\n.weak USART2_TX\nUSART2_TX = DH_TRAMPOLINE\n.weak UART0_RX\nUART0_RX = DH_TRAMPOLINE\n.weak UART0_TX\nUART0_TX = DH_TRAMPOLINE\n.weak UART1_RX\nUART1_RX = DH_TRAMPOLINE\n.weak UART1_TX\nUART1_TX = DH_TRAMPOLINE\n.weak LEUART0\nLEUART0 = DH_TRAMPOLINE\n.weak LEUART1\nLEUART1 = DH_TRAMPOLINE\n.weak LETIMER0\nLETIMER0 = DH_TRAMPOLINE\n.weak PCNT0\nPCNT0 = DH_TRAMPOLINE\n.weak PCNT1\nPCNT1 = DH_TRAMPOLINE\n.weak PCNT2\nPCNT2 = DH_TRAMPOLINE\n.weak RTCC\nRTCC = DH_TRAMPOLINE\n.weak CMU\nCMU = DH_TRAMPOLINE\n.weak MSC\nMSC = DH_TRAMPOLINE\n.weak CRYPTO0\nCRYPTO0 = DH_TRAMPOLINE\n.weak CRYOTIMER\nCRYOTIMER = DH_TRAMPOLINE\n.weak FPUEH\nFPUEH = DH_TRAMPOLINE\n.weak USART3_RX\nUSART3_RX = DH_TRAMPOLINE\n.weak USART3_TX\nUSART3_TX = DH_TRAMPOLINE\n.weak USART4_RX\nUSART4_RX = DH_TRAMPOLINE\n.weak USART4_TX\nUSART4_TX = DH_TRAMPOLINE\n.weak WTIMER0\nWTIMER0 = DH_TRAMPOLINE\n.weak WTIMER1\nWTIMER1 = DH_TRAMPOLINE\n.weak WTIMER2\nWTIMER2 = DH_TRAMPOLINE\n.weak WTIMER3\nWTIMER3 = DH_TRAMPOLINE\n.weak I2C2\nI2C2 = DH_TRAMPOLINE\n.weak VDAC0\nVDAC0 = DH_TRAMPOLINE\n.weak TIMER4\nTIMER4 = DH_TRAMPOLINE\n.weak TIMER5\nTIMER5 = DH_TRAMPOLINE\n.weak TIMER6\nTIMER6 = DH_TRAMPOLINE\n.weak USART5_RX\nUSART5_RX = DH_TRAMPOLINE\n.weak USART5_TX\nUSART5_TX = DH_TRAMPOLINE\n.weak CSEN\nCSEN = DH_TRAMPOLINE\n.weak LESENSE\nLESENSE = DH_TRAMPOLINE\n.weak EBI\nEBI = DH_TRAMPOLINE\n.weak ACMP2\nACMP2 = DH_TRAMPOLINE\n.weak ADC1\nADC1 = DH_TRAMPOLINE\n.weak LCD\nLCD = DH_TRAMPOLINE\n.weak SDIO\nSDIO = DH_TRAMPOLINE\n.weak ETH\nETH = DH_TRAMPOLINE\n.weak CAN0\nCAN0 = DH_TRAMPOLINE\n.weak CAN1\nCAN1 = DH_TRAMPOLINE\n.weak USB\nUSB = DH_TRAMPOLINE\n.weak RTC\nRTC = DH_TRAMPOLINE\n.weak WDOG1\nWDOG1 = DH_TRAMPOLINE\n.weak LETIMER1\nLETIMER1 = DH_TRAMPOLINE\n.weak TRNG0\nTRNG0 = DH_TRAMPOLINE\n.weak QSPI0\nQSPI0 = DH_TRAMPOLINE" ) ;
#[cfg(feature = "rt")]
extern "C" {
    fn EMU();
    fn WDOG0();
    fn LDMA();
    fn GPIO_EVEN();
    fn SMU();
    fn TIMER0();
    fn USART0_RX();
    fn USART0_TX();
    fn ACMP0();
    fn ADC0();
    fn IDAC0();
    fn I2C0();
    fn I2C1();
    fn GPIO_ODD();
    fn TIMER1();
    fn TIMER2();
    fn TIMER3();
    fn USART1_RX();
    fn USART1_TX();
    fn USART2_RX();
    fn USART2_TX();
    fn UART0_RX();
    fn UART0_TX();
    fn UART1_RX();
    fn UART1_TX();
    fn LEUART0();
    fn LEUART1();
    fn LETIMER0();
    fn PCNT0();
    fn PCNT1();
    fn PCNT2();
    fn RTCC();
    fn CMU();
    fn MSC();
    fn CRYPTO0();
    fn CRYOTIMER();
    fn FPUEH();
    fn USART3_RX();
    fn USART3_TX();
    fn USART4_RX();
    fn USART4_TX();
    fn WTIMER0();
    fn WTIMER1();
    fn WTIMER2();
    fn WTIMER3();
    fn I2C2();
    fn VDAC0();
    fn TIMER4();
    fn TIMER5();
    fn TIMER6();
    fn USART5_RX();
    fn USART5_TX();
    fn CSEN();
    fn LESENSE();
    fn EBI();
    fn ACMP2();
    fn ADC1();
    fn LCD();
    fn SDIO();
    fn ETH();
    fn CAN0();
    fn CAN1();
    fn USB();
    fn RTC();
    fn WDOG1();
    fn LETIMER1();
    fn TRNG0();
    fn QSPI0();
}
#[allow(private_no_mangle_statics)]
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static INTERRUPTS: [Option<unsafe extern "C" fn()>; 68] = [
    Some(EMU),
    Some(WDOG0),
    Some(LDMA),
    Some(GPIO_EVEN),
    Some(SMU),
    Some(TIMER0),
    Some(USART0_RX),
    Some(USART0_TX),
    Some(ACMP0),
    Some(ADC0),
    Some(IDAC0),
    Some(I2C0),
    Some(I2C1),
    Some(GPIO_ODD),
    Some(TIMER1),
    Some(TIMER2),
    Some(TIMER3),
    Some(USART1_RX),
    Some(USART1_TX),
    Some(USART2_RX),
    Some(USART2_TX),
    Some(UART0_RX),
    Some(UART0_TX),
    Some(UART1_RX),
    Some(UART1_TX),
    Some(LEUART0),
    Some(LEUART1),
    Some(LETIMER0),
    Some(PCNT0),
    Some(PCNT1),
    Some(PCNT2),
    Some(RTCC),
    Some(CMU),
    Some(MSC),
    Some(CRYPTO0),
    Some(CRYOTIMER),
    Some(FPUEH),
    Some(USART3_RX),
    Some(USART3_TX),
    Some(USART4_RX),
    Some(USART4_TX),
    Some(WTIMER0),
    Some(WTIMER1),
    Some(WTIMER2),
    Some(WTIMER3),
    Some(I2C2),
    Some(VDAC0),
    Some(TIMER4),
    Some(TIMER5),
    Some(TIMER6),
    Some(USART5_RX),
    Some(USART5_TX),
    Some(CSEN),
    Some(LESENSE),
    Some(EBI),
    Some(ACMP2),
    Some(ADC1),
    Some(LCD),
    Some(SDIO),
    Some(ETH),
    Some(CAN0),
    Some(CAN1),
    Some(USB),
    Some(RTC),
    Some(WDOG1),
    Some(LETIMER1),
    Some(TRNG0),
    Some(QSPI0),
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - EMU"]
    EMU,
    #[doc = "1 - WDOG0"]
    WDOG0,
    #[doc = "2 - LDMA"]
    LDMA,
    #[doc = "3 - GPIO_EVEN"]
    GPIO_EVEN,
    #[doc = "4 - SMU"]
    SMU,
    #[doc = "5 - TIMER0"]
    TIMER0,
    #[doc = "6 - USART0_RX"]
    USART0_RX,
    #[doc = "7 - USART0_TX"]
    USART0_TX,
    #[doc = "8 - ACMP0"]
    ACMP0,
    #[doc = "9 - ADC0"]
    ADC0,
    #[doc = "10 - IDAC0"]
    IDAC0,
    #[doc = "11 - I2C0"]
    I2C0,
    #[doc = "12 - I2C1"]
    I2C1,
    #[doc = "13 - GPIO_ODD"]
    GPIO_ODD,
    #[doc = "14 - TIMER1"]
    TIMER1,
    #[doc = "15 - TIMER2"]
    TIMER2,
    #[doc = "16 - TIMER3"]
    TIMER3,
    #[doc = "17 - USART1_RX"]
    USART1_RX,
    #[doc = "18 - USART1_TX"]
    USART1_TX,
    #[doc = "19 - USART2_RX"]
    USART2_RX,
    #[doc = "20 - USART2_TX"]
    USART2_TX,
    #[doc = "21 - UART0_RX"]
    UART0_RX,
    #[doc = "22 - UART0_TX"]
    UART0_TX,
    #[doc = "23 - UART1_RX"]
    UART1_RX,
    #[doc = "24 - UART1_TX"]
    UART1_TX,
    #[doc = "25 - LEUART0"]
    LEUART0,
    #[doc = "26 - LEUART1"]
    LEUART1,
    #[doc = "27 - LETIMER0"]
    LETIMER0,
    #[doc = "28 - PCNT0"]
    PCNT0,
    #[doc = "29 - PCNT1"]
    PCNT1,
    #[doc = "30 - PCNT2"]
    PCNT2,
    #[doc = "31 - RTCC"]
    RTCC,
    #[doc = "32 - CMU"]
    CMU,
    #[doc = "33 - MSC"]
    MSC,
    #[doc = "34 - CRYPTO0"]
    CRYPTO0,
    #[doc = "35 - CRYOTIMER"]
    CRYOTIMER,
    #[doc = "36 - FPUEH"]
    FPUEH,
    #[doc = "37 - USART3_RX"]
    USART3_RX,
    #[doc = "38 - USART3_TX"]
    USART3_TX,
    #[doc = "39 - USART4_RX"]
    USART4_RX,
    #[doc = "40 - USART4_TX"]
    USART4_TX,
    #[doc = "41 - WTIMER0"]
    WTIMER0,
    #[doc = "42 - WTIMER1"]
    WTIMER1,
    #[doc = "43 - WTIMER2"]
    WTIMER2,
    #[doc = "44 - WTIMER3"]
    WTIMER3,
    #[doc = "45 - I2C2"]
    I2C2,
    #[doc = "46 - VDAC0"]
    VDAC0,
    #[doc = "47 - TIMER4"]
    TIMER4,
    #[doc = "48 - TIMER5"]
    TIMER5,
    #[doc = "49 - TIMER6"]
    TIMER6,
    #[doc = "50 - USART5_RX"]
    USART5_RX,
    #[doc = "51 - USART5_TX"]
    USART5_TX,
    #[doc = "52 - CSEN"]
    CSEN,
    #[doc = "53 - LESENSE"]
    LESENSE,
    #[doc = "54 - EBI"]
    EBI,
    #[doc = "55 - ACMP2"]
    ACMP2,
    #[doc = "56 - ADC1"]
    ADC1,
    #[doc = "57 - LCD"]
    LCD,
    #[doc = "58 - SDIO"]
    SDIO,
    #[doc = "59 - ETH"]
    ETH,
    #[doc = "60 - CAN0"]
    CAN0,
    #[doc = "61 - CAN1"]
    CAN1,
    #[doc = "62 - USB"]
    USB,
    #[doc = "63 - RTC"]
    RTC,
    #[doc = "64 - WDOG1"]
    WDOG1,
    #[doc = "65 - LETIMER1"]
    LETIMER1,
    #[doc = "66 - TRNG0"]
    TRNG0,
    #[doc = "67 - QSPI0"]
    QSPI0,
}
unsafe impl Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::EMU => 0,
            Interrupt::WDOG0 => 1,
            Interrupt::LDMA => 2,
            Interrupt::GPIO_EVEN => 3,
            Interrupt::SMU => 4,
            Interrupt::TIMER0 => 5,
            Interrupt::USART0_RX => 6,
            Interrupt::USART0_TX => 7,
            Interrupt::ACMP0 => 8,
            Interrupt::ADC0 => 9,
            Interrupt::IDAC0 => 10,
            Interrupt::I2C0 => 11,
            Interrupt::I2C1 => 12,
            Interrupt::GPIO_ODD => 13,
            Interrupt::TIMER1 => 14,
            Interrupt::TIMER2 => 15,
            Interrupt::TIMER3 => 16,
            Interrupt::USART1_RX => 17,
            Interrupt::USART1_TX => 18,
            Interrupt::USART2_RX => 19,
            Interrupt::USART2_TX => 20,
            Interrupt::UART0_RX => 21,
            Interrupt::UART0_TX => 22,
            Interrupt::UART1_RX => 23,
            Interrupt::UART1_TX => 24,
            Interrupt::LEUART0 => 25,
            Interrupt::LEUART1 => 26,
            Interrupt::LETIMER0 => 27,
            Interrupt::PCNT0 => 28,
            Interrupt::PCNT1 => 29,
            Interrupt::PCNT2 => 30,
            Interrupt::RTCC => 31,
            Interrupt::CMU => 32,
            Interrupt::MSC => 33,
            Interrupt::CRYPTO0 => 34,
            Interrupt::CRYOTIMER => 35,
            Interrupt::FPUEH => 36,
            Interrupt::USART3_RX => 37,
            Interrupt::USART3_TX => 38,
            Interrupt::USART4_RX => 39,
            Interrupt::USART4_TX => 40,
            Interrupt::WTIMER0 => 41,
            Interrupt::WTIMER1 => 42,
            Interrupt::WTIMER2 => 43,
            Interrupt::WTIMER3 => 44,
            Interrupt::I2C2 => 45,
            Interrupt::VDAC0 => 46,
            Interrupt::TIMER4 => 47,
            Interrupt::TIMER5 => 48,
            Interrupt::TIMER6 => 49,
            Interrupt::USART5_RX => 50,
            Interrupt::USART5_TX => 51,
            Interrupt::CSEN => 52,
            Interrupt::LESENSE => 53,
            Interrupt::EBI => 54,
            Interrupt::ACMP2 => 55,
            Interrupt::ADC1 => 56,
            Interrupt::LCD => 57,
            Interrupt::SDIO => 58,
            Interrupt::ETH => 59,
            Interrupt::CAN0 => 60,
            Interrupt::CAN1 => 61,
            Interrupt::USB => 62,
            Interrupt::RTC => 63,
            Interrupt::WDOG1 => 64,
            Interrupt::LETIMER1 => 65,
            Interrupt::TRNG0 => 66,
            Interrupt::QSPI0 => 67,
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ty = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }
