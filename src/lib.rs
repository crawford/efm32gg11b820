#![cfg_attr(feature = "rt", feature(global_asm))]
#![cfg_attr(feature = "rt", feature(macro_reexport))]
#![cfg_attr(feature = "rt", feature(used))]
#![doc = "Peripheral access API for EFM32GG11B820F2048GL192 microcontrollers (generated using svd2rust v0.12.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.12.0/svd2rust/#peripheral-api"]
#![allow(private_no_mangle_statics)]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![feature(const_fn)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[macro_reexport(default_handler, exception)]
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::ops::Deref;
use core::marker::PhantomData;
pub use interrupt::Interrupt;
#[doc(hidden)]
pub mod interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::CPUID;
pub use cortex_m::peripheral::DCB;
pub use cortex_m::peripheral::DWT;
pub use cortex_m::peripheral::MPU;
pub use cortex_m::peripheral::NVIC;
pub use cortex_m::peripheral::SCB;
pub use cortex_m::peripheral::SYST;
#[doc = "MSC"]
pub struct MSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MSC {}
impl MSC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const msc::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for MSC {
    type Target = msc::RegisterBlock;
    fn deref(&self) -> &msc::RegisterBlock {
        unsafe { &*MSC::ptr() }
    }
}
#[doc = "MSC"]
pub mod msc;
#[doc = "EMU"]
pub struct EMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EMU {}
impl EMU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const emu::RegisterBlock {
        1074671616 as *const _
    }
}
impl Deref for EMU {
    type Target = emu::RegisterBlock;
    fn deref(&self) -> &emu::RegisterBlock {
        unsafe { &*EMU::ptr() }
    }
}
#[doc = "EMU"]
pub mod emu;
#[doc = "RMU"]
pub struct RMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RMU {}
impl RMU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rmu::RegisterBlock {
        1074679808 as *const _
    }
}
impl Deref for RMU {
    type Target = rmu::RegisterBlock;
    fn deref(&self) -> &rmu::RegisterBlock {
        unsafe { &*RMU::ptr() }
    }
}
#[doc = "RMU"]
pub mod rmu;
#[doc = "CMU"]
pub struct CMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMU {}
impl CMU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cmu::RegisterBlock {
        1074675712 as *const _
    }
}
impl Deref for CMU {
    type Target = cmu::RegisterBlock;
    fn deref(&self) -> &cmu::RegisterBlock {
        unsafe { &*CMU::ptr() }
    }
}
#[doc = "CMU"]
pub mod cmu;
#[doc = "CRYPTO0"]
pub struct CRYPTO0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRYPTO0 {}
impl CRYPTO0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crypto0::RegisterBlock {
        1074724864 as *const _
    }
}
impl Deref for CRYPTO0 {
    type Target = crypto0::RegisterBlock;
    fn deref(&self) -> &crypto0::RegisterBlock {
        unsafe { &*CRYPTO0::ptr() }
    }
}
#[doc = "CRYPTO0"]
pub mod crypto0;
#[doc = "LESENSE"]
pub struct LESENSE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LESENSE {}
impl LESENSE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lesense::RegisterBlock {
        1074089984 as *const _
    }
}
impl Deref for LESENSE {
    type Target = lesense::RegisterBlock;
    fn deref(&self) -> &lesense::RegisterBlock {
        unsafe { &*LESENSE::ptr() }
    }
}
#[doc = "LESENSE"]
pub mod lesense;
#[doc = "EBI"]
pub struct EBI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EBI {}
impl EBI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ebi::RegisterBlock {
        1073786880 as *const _
    }
}
impl Deref for EBI {
    type Target = ebi::RegisterBlock;
    fn deref(&self) -> &ebi::RegisterBlock {
        unsafe { &*EBI::ptr() }
    }
}
#[doc = "EBI"]
pub mod ebi;
#[doc = "ETH"]
pub struct ETH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETH {}
impl ETH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const eth::RegisterBlock {
        1073889280 as *const _
    }
}
impl Deref for ETH {
    type Target = eth::RegisterBlock;
    fn deref(&self) -> &eth::RegisterBlock {
        unsafe { &*ETH::ptr() }
    }
}
#[doc = "ETH"]
pub mod eth;
#[doc = "SDIO"]
pub struct SDIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDIO {}
impl SDIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sdio::RegisterBlock {
        1074728960 as *const _
    }
}
impl Deref for SDIO {
    type Target = sdio::RegisterBlock;
    fn deref(&self) -> &sdio::RegisterBlock {
        unsafe { &*SDIO::ptr() }
    }
}
#[doc = "SDIO"]
pub mod sdio;
#[doc = "GPIO"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio::RegisterBlock {
        1074298880 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    fn deref(&self) -> &gpio::RegisterBlock {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "GPIO"]
pub mod gpio;
#[doc = "PRS"]
pub struct PRS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PRS {}
impl PRS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const prs::RegisterBlock {
        1074683904 as *const _
    }
}
impl Deref for PRS {
    type Target = prs::RegisterBlock;
    fn deref(&self) -> &prs::RegisterBlock {
        unsafe { &*PRS::ptr() }
    }
}
#[doc = "PRS"]
pub mod prs;
#[doc = "LDMA"]
pub struct LDMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LDMA {}
impl LDMA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ldma::RegisterBlock {
        1073750016 as *const _
    }
}
impl Deref for LDMA {
    type Target = ldma::RegisterBlock;
    fn deref(&self) -> &ldma::RegisterBlock {
        unsafe { &*LDMA::ptr() }
    }
}
#[doc = "LDMA"]
pub mod ldma;
#[doc = "FPUEH"]
pub struct FPUEH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FPUEH {}
impl FPUEH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fpueh::RegisterBlock {
        1073745920 as *const _
    }
}
impl Deref for FPUEH {
    type Target = fpueh::RegisterBlock;
    fn deref(&self) -> &fpueh::RegisterBlock {
        unsafe { &*FPUEH::ptr() }
    }
}
#[doc = "FPUEH"]
pub mod fpueh;
#[doc = "GPCRC"]
pub struct GPCRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPCRC {}
impl GPCRC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpcrc::RegisterBlock {
        1073856512 as *const _
    }
}
impl Deref for GPCRC {
    type Target = gpcrc::RegisterBlock;
    fn deref(&self) -> &gpcrc::RegisterBlock {
        unsafe { &*GPCRC::ptr() }
    }
}
#[doc = "GPCRC"]
pub mod gpcrc;
#[doc = "CAN0"]
pub struct CAN0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN0 {}
impl CAN0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can0::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for CAN0 {
    type Target = can0::RegisterBlock;
    fn deref(&self) -> &can0::RegisterBlock {
        unsafe { &*CAN0::ptr() }
    }
}
#[doc = "CAN0"]
pub mod can0;
#[doc = "CAN1"]
pub struct CAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN1 {}
impl CAN1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can1::RegisterBlock {
        1073759232 as *const _
    }
}
impl Deref for CAN1 {
    type Target = can1::RegisterBlock;
    fn deref(&self) -> &can1::RegisterBlock {
        unsafe { &*CAN1::ptr() }
    }
}
#[doc = "CAN1"]
pub mod can1;
#[doc = "TIMER0"]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1073840128 as *const _
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER0::ptr() }
    }
}
#[doc = "TIMER0"]
pub mod timer0;
#[doc = "TIMER1"]
pub struct TIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1 {}
impl TIMER1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer1::RegisterBlock {
        1073841152 as *const _
    }
}
impl Deref for TIMER1 {
    type Target = timer1::RegisterBlock;
    fn deref(&self) -> &timer1::RegisterBlock {
        unsafe { &*TIMER1::ptr() }
    }
}
#[doc = "TIMER1"]
pub mod timer1;
#[doc = "TIMER2"]
pub struct TIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER2 {}
impl TIMER2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer2::RegisterBlock {
        1073842176 as *const _
    }
}
impl Deref for TIMER2 {
    type Target = timer2::RegisterBlock;
    fn deref(&self) -> &timer2::RegisterBlock {
        unsafe { &*TIMER2::ptr() }
    }
}
#[doc = "TIMER2"]
pub mod timer2;
#[doc = "TIMER3"]
pub struct TIMER3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER3 {}
impl TIMER3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer3::RegisterBlock {
        1073843200 as *const _
    }
}
impl Deref for TIMER3 {
    type Target = timer3::RegisterBlock;
    fn deref(&self) -> &timer3::RegisterBlock {
        unsafe { &*TIMER3::ptr() }
    }
}
#[doc = "TIMER3"]
pub mod timer3;
#[doc = "TIMER4"]
pub struct TIMER4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER4 {}
impl TIMER4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer4::RegisterBlock {
        1073844224 as *const _
    }
}
impl Deref for TIMER4 {
    type Target = timer4::RegisterBlock;
    fn deref(&self) -> &timer4::RegisterBlock {
        unsafe { &*TIMER4::ptr() }
    }
}
#[doc = "TIMER4"]
pub mod timer4;
#[doc = "TIMER5"]
pub struct TIMER5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER5 {}
impl TIMER5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer5::RegisterBlock {
        1073845248 as *const _
    }
}
impl Deref for TIMER5 {
    type Target = timer5::RegisterBlock;
    fn deref(&self) -> &timer5::RegisterBlock {
        unsafe { &*TIMER5::ptr() }
    }
}
#[doc = "TIMER5"]
pub mod timer5;
#[doc = "TIMER6"]
pub struct TIMER6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER6 {}
impl TIMER6 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer6::RegisterBlock {
        1073846272 as *const _
    }
}
impl Deref for TIMER6 {
    type Target = timer6::RegisterBlock;
    fn deref(&self) -> &timer6::RegisterBlock {
        unsafe { &*TIMER6::ptr() }
    }
}
#[doc = "TIMER6"]
pub mod timer6;
#[doc = "WTIMER0"]
pub struct WTIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WTIMER0 {}
impl WTIMER0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wtimer0::RegisterBlock {
        1073848320 as *const _
    }
}
impl Deref for WTIMER0 {
    type Target = wtimer0::RegisterBlock;
    fn deref(&self) -> &wtimer0::RegisterBlock {
        unsafe { &*WTIMER0::ptr() }
    }
}
#[doc = "WTIMER0"]
pub mod wtimer0;
#[doc = "WTIMER1"]
pub struct WTIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WTIMER1 {}
impl WTIMER1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wtimer1::RegisterBlock {
        1073849344 as *const _
    }
}
impl Deref for WTIMER1 {
    type Target = wtimer1::RegisterBlock;
    fn deref(&self) -> &wtimer1::RegisterBlock {
        unsafe { &*WTIMER1::ptr() }
    }
}
#[doc = "WTIMER1"]
pub mod wtimer1;
#[doc = "WTIMER2"]
pub struct WTIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WTIMER2 {}
impl WTIMER2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wtimer2::RegisterBlock {
        1073850368 as *const _
    }
}
impl Deref for WTIMER2 {
    type Target = wtimer2::RegisterBlock;
    fn deref(&self) -> &wtimer2::RegisterBlock {
        unsafe { &*WTIMER2::ptr() }
    }
}
#[doc = "WTIMER2"]
pub mod wtimer2;
#[doc = "WTIMER3"]
pub struct WTIMER3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WTIMER3 {}
impl WTIMER3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wtimer3::RegisterBlock {
        1073851392 as *const _
    }
}
impl Deref for WTIMER3 {
    type Target = wtimer3::RegisterBlock;
    fn deref(&self) -> &wtimer3::RegisterBlock {
        unsafe { &*WTIMER3::ptr() }
    }
}
#[doc = "WTIMER3"]
pub mod wtimer3;
#[doc = "USART0"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart0::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &usart0::RegisterBlock {
        unsafe { &*USART0::ptr() }
    }
}
#[doc = "USART0"]
pub mod usart0;
#[doc = "USART1"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073808384 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "USART1"]
pub mod usart1;
#[doc = "USART2"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart2::RegisterBlock {
        1073809408 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart2::RegisterBlock;
    fn deref(&self) -> &usart2::RegisterBlock {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "USART2"]
pub mod usart2;
#[doc = "USART3"]
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart3::RegisterBlock {
        1073810432 as *const _
    }
}
impl Deref for USART3 {
    type Target = usart3::RegisterBlock;
    fn deref(&self) -> &usart3::RegisterBlock {
        unsafe { &*USART3::ptr() }
    }
}
#[doc = "USART3"]
pub mod usart3;
#[doc = "USART4"]
pub struct USART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART4 {}
impl USART4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart4::RegisterBlock {
        1073811456 as *const _
    }
}
impl Deref for USART4 {
    type Target = usart4::RegisterBlock;
    fn deref(&self) -> &usart4::RegisterBlock {
        unsafe { &*USART4::ptr() }
    }
}
#[doc = "USART4"]
pub mod usart4;
#[doc = "USART5"]
pub struct USART5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART5 {}
impl USART5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart5::RegisterBlock {
        1073812480 as *const _
    }
}
impl Deref for USART5 {
    type Target = usart5::RegisterBlock;
    fn deref(&self) -> &usart5::RegisterBlock {
        unsafe { &*USART5::ptr() }
    }
}
#[doc = "USART5"]
pub mod usart5;
#[doc = "UART0"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        1073823744 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "UART0"]
pub mod uart0;
#[doc = "UART1"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart1::RegisterBlock {
        1073824768 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    fn deref(&self) -> &uart1::RegisterBlock {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "UART1"]
pub mod uart1;
#[doc = "QSPI0"]
pub struct QSPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QSPI0 {}
impl QSPI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const qspi0::RegisterBlock {
        1073857536 as *const _
    }
}
impl Deref for QSPI0 {
    type Target = qspi0::RegisterBlock;
    fn deref(&self) -> &qspi0::RegisterBlock {
        unsafe { &*QSPI0::ptr() }
    }
}
#[doc = "QSPI0"]
pub mod qspi0;
#[doc = "LEUART0"]
pub struct LEUART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LEUART0 {}
impl LEUART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const leuart0::RegisterBlock {
        1074176000 as *const _
    }
}
impl Deref for LEUART0 {
    type Target = leuart0::RegisterBlock;
    fn deref(&self) -> &leuart0::RegisterBlock {
        unsafe { &*LEUART0::ptr() }
    }
}
#[doc = "LEUART0"]
pub mod leuart0;
#[doc = "LEUART1"]
pub struct LEUART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LEUART1 {}
impl LEUART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const leuart1::RegisterBlock {
        1074177024 as *const _
    }
}
impl Deref for LEUART1 {
    type Target = leuart1::RegisterBlock;
    fn deref(&self) -> &leuart1::RegisterBlock {
        unsafe { &*LEUART1::ptr() }
    }
}
#[doc = "LEUART1"]
pub mod leuart1;
#[doc = "LETIMER0"]
pub struct LETIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LETIMER0 {}
impl LETIMER0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const letimer0::RegisterBlock {
        1074159616 as *const _
    }
}
impl Deref for LETIMER0 {
    type Target = letimer0::RegisterBlock;
    fn deref(&self) -> &letimer0::RegisterBlock {
        unsafe { &*LETIMER0::ptr() }
    }
}
#[doc = "LETIMER0"]
pub mod letimer0;
#[doc = "LETIMER1"]
pub struct LETIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LETIMER1 {}
impl LETIMER1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const letimer1::RegisterBlock {
        1074160640 as *const _
    }
}
impl Deref for LETIMER1 {
    type Target = letimer1::RegisterBlock;
    fn deref(&self) -> &letimer1::RegisterBlock {
        unsafe { &*LETIMER1::ptr() }
    }
}
#[doc = "LETIMER1"]
pub mod letimer1;
#[doc = "CRYOTIMER"]
pub struct CRYOTIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRYOTIMER {}
impl CRYOTIMER {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cryotimer::RegisterBlock {
        1074327552 as *const _
    }
}
impl Deref for CRYOTIMER {
    type Target = cryotimer::RegisterBlock;
    fn deref(&self) -> &cryotimer::RegisterBlock {
        unsafe { &*CRYOTIMER::ptr() }
    }
}
#[doc = "CRYOTIMER"]
pub mod cryotimer;
#[doc = "PCNT0"]
pub struct PCNT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PCNT0 {}
impl PCNT0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pcnt0::RegisterBlock {
        1074192384 as *const _
    }
}
impl Deref for PCNT0 {
    type Target = pcnt0::RegisterBlock;
    fn deref(&self) -> &pcnt0::RegisterBlock {
        unsafe { &*PCNT0::ptr() }
    }
}
#[doc = "PCNT0"]
pub mod pcnt0;
#[doc = "PCNT1"]
pub struct PCNT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PCNT1 {}
impl PCNT1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pcnt1::RegisterBlock {
        1074193408 as *const _
    }
}
impl Deref for PCNT1 {
    type Target = pcnt1::RegisterBlock;
    fn deref(&self) -> &pcnt1::RegisterBlock {
        unsafe { &*PCNT1::ptr() }
    }
}
#[doc = "PCNT1"]
pub mod pcnt1;
#[doc = "PCNT2"]
pub struct PCNT2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PCNT2 {}
impl PCNT2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pcnt2::RegisterBlock {
        1074194432 as *const _
    }
}
impl Deref for PCNT2 {
    type Target = pcnt2::RegisterBlock;
    fn deref(&self) -> &pcnt2::RegisterBlock {
        unsafe { &*PCNT2::ptr() }
    }
}
#[doc = "PCNT2"]
pub mod pcnt2;
#[doc = "I2C0"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1074302976 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "I2C0"]
pub mod i2c0;
#[doc = "I2C1"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        1074304000 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "I2C1"]
pub mod i2c1;
#[doc = "I2C2"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c2::RegisterBlock {
        1074305024 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c2::RegisterBlock;
    fn deref(&self) -> &i2c2::RegisterBlock {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "I2C2"]
pub mod i2c2;
#[doc = "ADC0"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc0::RegisterBlock {
        1074274304 as *const _
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    fn deref(&self) -> &adc0::RegisterBlock {
        unsafe { &*ADC0::ptr() }
    }
}
#[doc = "ADC0"]
pub mod adc0;
#[doc = "ADC1"]
pub struct ADC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC1 {}
impl ADC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc1::RegisterBlock {
        1074275328 as *const _
    }
}
impl Deref for ADC1 {
    type Target = adc1::RegisterBlock;
    fn deref(&self) -> &adc1::RegisterBlock {
        unsafe { &*ADC1::ptr() }
    }
}
#[doc = "ADC1"]
pub mod adc1;
#[doc = "ACMP0"]
pub struct ACMP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACMP0 {}
impl ACMP0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const acmp0::RegisterBlock {
        1074266112 as *const _
    }
}
impl Deref for ACMP0 {
    type Target = acmp0::RegisterBlock;
    fn deref(&self) -> &acmp0::RegisterBlock {
        unsafe { &*ACMP0::ptr() }
    }
}
#[doc = "ACMP0"]
pub mod acmp0;
#[doc = "ACMP1"]
pub struct ACMP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACMP1 {}
impl ACMP1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const acmp1::RegisterBlock {
        1074267136 as *const _
    }
}
impl Deref for ACMP1 {
    type Target = acmp1::RegisterBlock;
    fn deref(&self) -> &acmp1::RegisterBlock {
        unsafe { &*ACMP1::ptr() }
    }
}
#[doc = "ACMP1"]
pub mod acmp1;
#[doc = "ACMP2"]
pub struct ACMP2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACMP2 {}
impl ACMP2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const acmp2::RegisterBlock {
        1074268160 as *const _
    }
}
impl Deref for ACMP2 {
    type Target = acmp2::RegisterBlock;
    fn deref(&self) -> &acmp2::RegisterBlock {
        unsafe { &*ACMP2::ptr() }
    }
}
#[doc = "ACMP2"]
pub mod acmp2;
#[doc = "ACMP3"]
pub struct ACMP3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACMP3 {}
impl ACMP3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const acmp3::RegisterBlock {
        1074269184 as *const _
    }
}
impl Deref for ACMP3 {
    type Target = acmp3::RegisterBlock;
    fn deref(&self) -> &acmp3::RegisterBlock {
        unsafe { &*ACMP3::ptr() }
    }
}
#[doc = "ACMP3"]
pub mod acmp3;
#[doc = "VDAC0"]
pub struct VDAC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VDAC0 {}
impl VDAC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const vdac0::RegisterBlock {
        1074290688 as *const _
    }
}
impl Deref for VDAC0 {
    type Target = vdac0::RegisterBlock;
    fn deref(&self) -> &vdac0::RegisterBlock {
        unsafe { &*VDAC0::ptr() }
    }
}
#[doc = "VDAC0"]
pub mod vdac0;
#[doc = "USB"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb::RegisterBlock {
        1073881088 as *const _
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    fn deref(&self) -> &usb::RegisterBlock {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "USB"]
pub mod usb;
#[doc = "IDAC0"]
pub struct IDAC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IDAC0 {}
impl IDAC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const idac0::RegisterBlock {
        1074282496 as *const _
    }
}
impl Deref for IDAC0 {
    type Target = idac0::RegisterBlock;
    fn deref(&self) -> &idac0::RegisterBlock {
        unsafe { &*IDAC0::ptr() }
    }
}
#[doc = "IDAC0"]
pub mod idac0;
#[doc = "CSEN"]
pub struct CSEN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CSEN {}
impl CSEN {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const csen::RegisterBlock {
        1074323456 as *const _
    }
}
impl Deref for CSEN {
    type Target = csen::RegisterBlock;
    fn deref(&self) -> &csen::RegisterBlock {
        unsafe { &*CSEN::ptr() }
    }
}
#[doc = "CSEN"]
pub mod csen;
#[doc = "LCD"]
pub struct LCD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LCD {}
impl LCD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lcd::RegisterBlock {
        1074085888 as *const _
    }
}
impl Deref for LCD {
    type Target = lcd::RegisterBlock;
    fn deref(&self) -> &lcd::RegisterBlock {
        unsafe { &*LCD::ptr() }
    }
}
#[doc = "LCD"]
pub mod lcd;
#[doc = "RTC"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1074135040 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "RTC"]
pub mod rtc;
#[doc = "RTCC"]
pub struct RTCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTCC {}
impl RTCC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtcc::RegisterBlock {
        1074143232 as *const _
    }
}
impl Deref for RTCC {
    type Target = rtcc::RegisterBlock;
    fn deref(&self) -> &rtcc::RegisterBlock {
        unsafe { &*RTCC::ptr() }
    }
}
#[doc = "RTCC"]
pub mod rtcc;
#[doc = "WDOG0"]
pub struct WDOG0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDOG0 {}
impl WDOG0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdog0::RegisterBlock {
        1074077696 as *const _
    }
}
impl Deref for WDOG0 {
    type Target = wdog0::RegisterBlock;
    fn deref(&self) -> &wdog0::RegisterBlock {
        unsafe { &*WDOG0::ptr() }
    }
}
#[doc = "WDOG0"]
pub mod wdog0;
#[doc = "WDOG1"]
pub struct WDOG1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDOG1 {}
impl WDOG1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdog1::RegisterBlock {
        1074078720 as *const _
    }
}
impl Deref for WDOG1 {
    type Target = wdog1::RegisterBlock;
    fn deref(&self) -> &wdog1::RegisterBlock {
        unsafe { &*WDOG1::ptr() }
    }
}
#[doc = "WDOG1"]
pub mod wdog1;
#[doc = "ETM"]
pub struct ETM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETM {}
impl ETM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const etm::RegisterBlock {
        3758362624 as *const _
    }
}
impl Deref for ETM {
    type Target = etm::RegisterBlock;
    fn deref(&self) -> &etm::RegisterBlock {
        unsafe { &*ETM::ptr() }
    }
}
#[doc = "ETM"]
pub mod etm;
#[doc = "SMU"]
pub struct SMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMU {}
impl SMU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const smu::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for SMU {
    type Target = smu::RegisterBlock;
    fn deref(&self) -> &smu::RegisterBlock {
        unsafe { &*SMU::ptr() }
    }
}
#[doc = "SMU"]
pub mod smu;
#[doc = "TRNG0"]
pub struct TRNG0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRNG0 {}
impl TRNG0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const trng0::RegisterBlock {
        1073860608 as *const _
    }
}
impl Deref for TRNG0 {
    type Target = trng0::RegisterBlock;
    fn deref(&self) -> &trng0::RegisterBlock {
        unsafe { &*TRNG0::ptr() }
    }
}
#[doc = "TRNG0"]
pub mod trng0;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "MSC"]
    pub MSC: MSC,
    #[doc = "EMU"]
    pub EMU: EMU,
    #[doc = "RMU"]
    pub RMU: RMU,
    #[doc = "CMU"]
    pub CMU: CMU,
    #[doc = "CRYPTO0"]
    pub CRYPTO0: CRYPTO0,
    #[doc = "LESENSE"]
    pub LESENSE: LESENSE,
    #[doc = "EBI"]
    pub EBI: EBI,
    #[doc = "ETH"]
    pub ETH: ETH,
    #[doc = "SDIO"]
    pub SDIO: SDIO,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "PRS"]
    pub PRS: PRS,
    #[doc = "LDMA"]
    pub LDMA: LDMA,
    #[doc = "FPUEH"]
    pub FPUEH: FPUEH,
    #[doc = "GPCRC"]
    pub GPCRC: GPCRC,
    #[doc = "CAN0"]
    pub CAN0: CAN0,
    #[doc = "CAN1"]
    pub CAN1: CAN1,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TIMER1"]
    pub TIMER1: TIMER1,
    #[doc = "TIMER2"]
    pub TIMER2: TIMER2,
    #[doc = "TIMER3"]
    pub TIMER3: TIMER3,
    #[doc = "TIMER4"]
    pub TIMER4: TIMER4,
    #[doc = "TIMER5"]
    pub TIMER5: TIMER5,
    #[doc = "TIMER6"]
    pub TIMER6: TIMER6,
    #[doc = "WTIMER0"]
    pub WTIMER0: WTIMER0,
    #[doc = "WTIMER1"]
    pub WTIMER1: WTIMER1,
    #[doc = "WTIMER2"]
    pub WTIMER2: WTIMER2,
    #[doc = "WTIMER3"]
    pub WTIMER3: WTIMER3,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART3"]
    pub USART3: USART3,
    #[doc = "USART4"]
    pub USART4: USART4,
    #[doc = "USART5"]
    pub USART5: USART5,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "QSPI0"]
    pub QSPI0: QSPI0,
    #[doc = "LEUART0"]
    pub LEUART0: LEUART0,
    #[doc = "LEUART1"]
    pub LEUART1: LEUART1,
    #[doc = "LETIMER0"]
    pub LETIMER0: LETIMER0,
    #[doc = "LETIMER1"]
    pub LETIMER1: LETIMER1,
    #[doc = "CRYOTIMER"]
    pub CRYOTIMER: CRYOTIMER,
    #[doc = "PCNT0"]
    pub PCNT0: PCNT0,
    #[doc = "PCNT1"]
    pub PCNT1: PCNT1,
    #[doc = "PCNT2"]
    pub PCNT2: PCNT2,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "ADC1"]
    pub ADC1: ADC1,
    #[doc = "ACMP0"]
    pub ACMP0: ACMP0,
    #[doc = "ACMP1"]
    pub ACMP1: ACMP1,
    #[doc = "ACMP2"]
    pub ACMP2: ACMP2,
    #[doc = "ACMP3"]
    pub ACMP3: ACMP3,
    #[doc = "VDAC0"]
    pub VDAC0: VDAC0,
    #[doc = "USB"]
    pub USB: USB,
    #[doc = "IDAC0"]
    pub IDAC0: IDAC0,
    #[doc = "CSEN"]
    pub CSEN: CSEN,
    #[doc = "LCD"]
    pub LCD: LCD,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "RTCC"]
    pub RTCC: RTCC,
    #[doc = "WDOG0"]
    pub WDOG0: WDOG0,
    #[doc = "WDOG1"]
    pub WDOG1: WDOG1,
    #[doc = "ETM"]
    pub ETM: ETM,
    #[doc = "SMU"]
    pub SMU: SMU,
    #[doc = "TRNG0"]
    pub TRNG0: TRNG0,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            MSC: MSC {
                _marker: PhantomData,
            },
            EMU: EMU {
                _marker: PhantomData,
            },
            RMU: RMU {
                _marker: PhantomData,
            },
            CMU: CMU {
                _marker: PhantomData,
            },
            CRYPTO0: CRYPTO0 {
                _marker: PhantomData,
            },
            LESENSE: LESENSE {
                _marker: PhantomData,
            },
            EBI: EBI {
                _marker: PhantomData,
            },
            ETH: ETH {
                _marker: PhantomData,
            },
            SDIO: SDIO {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            PRS: PRS {
                _marker: PhantomData,
            },
            LDMA: LDMA {
                _marker: PhantomData,
            },
            FPUEH: FPUEH {
                _marker: PhantomData,
            },
            GPCRC: GPCRC {
                _marker: PhantomData,
            },
            CAN0: CAN0 {
                _marker: PhantomData,
            },
            CAN1: CAN1 {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            TIMER1: TIMER1 {
                _marker: PhantomData,
            },
            TIMER2: TIMER2 {
                _marker: PhantomData,
            },
            TIMER3: TIMER3 {
                _marker: PhantomData,
            },
            TIMER4: TIMER4 {
                _marker: PhantomData,
            },
            TIMER5: TIMER5 {
                _marker: PhantomData,
            },
            TIMER6: TIMER6 {
                _marker: PhantomData,
            },
            WTIMER0: WTIMER0 {
                _marker: PhantomData,
            },
            WTIMER1: WTIMER1 {
                _marker: PhantomData,
            },
            WTIMER2: WTIMER2 {
                _marker: PhantomData,
            },
            WTIMER3: WTIMER3 {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            USART3: USART3 {
                _marker: PhantomData,
            },
            USART4: USART4 {
                _marker: PhantomData,
            },
            USART5: USART5 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            QSPI0: QSPI0 {
                _marker: PhantomData,
            },
            LEUART0: LEUART0 {
                _marker: PhantomData,
            },
            LEUART1: LEUART1 {
                _marker: PhantomData,
            },
            LETIMER0: LETIMER0 {
                _marker: PhantomData,
            },
            LETIMER1: LETIMER1 {
                _marker: PhantomData,
            },
            CRYOTIMER: CRYOTIMER {
                _marker: PhantomData,
            },
            PCNT0: PCNT0 {
                _marker: PhantomData,
            },
            PCNT1: PCNT1 {
                _marker: PhantomData,
            },
            PCNT2: PCNT2 {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            ADC0: ADC0 {
                _marker: PhantomData,
            },
            ADC1: ADC1 {
                _marker: PhantomData,
            },
            ACMP0: ACMP0 {
                _marker: PhantomData,
            },
            ACMP1: ACMP1 {
                _marker: PhantomData,
            },
            ACMP2: ACMP2 {
                _marker: PhantomData,
            },
            ACMP3: ACMP3 {
                _marker: PhantomData,
            },
            VDAC0: VDAC0 {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
            IDAC0: IDAC0 {
                _marker: PhantomData,
            },
            CSEN: CSEN {
                _marker: PhantomData,
            },
            LCD: LCD {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            RTCC: RTCC {
                _marker: PhantomData,
            },
            WDOG0: WDOG0 {
                _marker: PhantomData,
            },
            WDOG1: WDOG1 {
                _marker: PhantomData,
            },
            ETM: ETM {
                _marker: PhantomData,
            },
            SMU: SMU {
                _marker: PhantomData,
            },
            TRNG0: TRNG0 {
                _marker: PhantomData,
            },
        }
    }
}
