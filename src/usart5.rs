#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - USART Frame Format Register"]
    pub frame: FRAME,
    #[doc = "0x08 - USART Trigger Control Register"]
    pub trigctrl: TRIGCTRL,
    #[doc = "0x0c - Command Register"]
    pub cmd: CMD,
    #[doc = "0x10 - USART Status Register"]
    pub status: STATUS,
    #[doc = "0x14 - Clock Control Register"]
    pub clkdiv: CLKDIV,
    #[doc = "0x18 - RX Buffer Data Extended Register"]
    pub rxdatax: RXDATAX,
    #[doc = "0x1c - RX Buffer Data Register"]
    pub rxdata: RXDATA,
    #[doc = "0x20 - RX Buffer Double Data Extended Register"]
    pub rxdoublex: RXDOUBLEX,
    #[doc = "0x24 - RX FIFO Double Data Register"]
    pub rxdouble: RXDOUBLE,
    #[doc = "0x28 - RX Buffer Data Extended Peek Register"]
    pub rxdataxp: RXDATAXP,
    #[doc = "0x2c - RX Buffer Double Data Extended Peek Register"]
    pub rxdoublexp: RXDOUBLEXP,
    #[doc = "0x30 - TX Buffer Data Extended Register"]
    pub txdatax: TXDATAX,
    #[doc = "0x34 - TX Buffer Data Register"]
    pub txdata: TXDATA,
    #[doc = "0x38 - TX Buffer Double Data Extended Register"]
    pub txdoublex: TXDOUBLEX,
    #[doc = "0x3c - TX Buffer Double Data Register"]
    pub txdouble: TXDOUBLE,
    #[doc = "0x40 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x44 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x48 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x4c - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x50 - IrDA Control Register"]
    pub irctrl: IRCTRL,
    _reserved21: [u8; 4usize],
    #[doc = "0x58 - USART Input Register"]
    pub input: INPUT,
    #[doc = "0x5c - I2S Control Register"]
    pub i2sctrl: I2SCTRL,
    #[doc = "0x60 - Timing Register"]
    pub timing: TIMING,
    #[doc = "0x64 - Control Register Extended"]
    pub ctrlx: CTRLX,
    #[doc = "0x68 - Used to Generate Interrupts and Various Delays"]
    pub timecmp0: TIMECMP0,
    #[doc = "0x6c - Used to Generate Interrupts and Various Delays"]
    pub timecmp1: TIMECMP1,
    #[doc = "0x70 - Used to Generate Interrupts and Various Delays"]
    pub timecmp2: TIMECMP2,
    #[doc = "0x74 - I/O Routing Pin Enable Register"]
    pub routepen: ROUTEPEN,
    #[doc = "0x78 - I/O Routing Location Register"]
    pub routeloc0: ROUTELOC0,
    #[doc = "0x7c - I/O Routing Location Register"]
    pub routeloc1: ROUTELOC1,
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "USART Frame Format Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [frame](frame) module"]
pub type FRAME = crate::Reg<u32, _FRAME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAME;
#[doc = "`read()` method returns [frame::R](frame::R) reader structure"]
impl crate::Readable for FRAME {}
#[doc = "`write(|w| ..)` method takes [frame::W](frame::W) writer structure"]
impl crate::Writable for FRAME {}
#[doc = "USART Frame Format Register"]
pub mod frame;
#[doc = "USART Trigger Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [trigctrl](trigctrl) module"]
pub type TRIGCTRL = crate::Reg<u32, _TRIGCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIGCTRL;
#[doc = "`read()` method returns [trigctrl::R](trigctrl::R) reader structure"]
impl crate::Readable for TRIGCTRL {}
#[doc = "`write(|w| ..)` method takes [trigctrl::W](trigctrl::W) writer structure"]
impl crate::Writable for TRIGCTRL {}
#[doc = "USART Trigger Control Register"]
pub mod trigctrl;
#[doc = "Command Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "USART Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "USART Status Register"]
pub mod status;
#[doc = "Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clkdiv](clkdiv) module"]
pub type CLKDIV = crate::Reg<u32, _CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKDIV;
#[doc = "`read()` method returns [clkdiv::R](clkdiv::R) reader structure"]
impl crate::Readable for CLKDIV {}
#[doc = "`write(|w| ..)` method takes [clkdiv::W](clkdiv::W) writer structure"]
impl crate::Writable for CLKDIV {}
#[doc = "Clock Control Register"]
pub mod clkdiv;
#[doc = "RX Buffer Data Extended Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxdatax](rxdatax) module"]
pub type RXDATAX = crate::Reg<u32, _RXDATAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDATAX;
#[doc = "`read()` method returns [rxdatax::R](rxdatax::R) reader structure"]
impl crate::Readable for RXDATAX {}
#[doc = "RX Buffer Data Extended Register"]
pub mod rxdatax;
#[doc = "RX Buffer Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxdata](rxdata) module"]
pub type RXDATA = crate::Reg<u32, _RXDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDATA;
#[doc = "`read()` method returns [rxdata::R](rxdata::R) reader structure"]
impl crate::Readable for RXDATA {}
#[doc = "RX Buffer Data Register"]
pub mod rxdata;
#[doc = "RX Buffer Double Data Extended Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxdoublex](rxdoublex) module"]
pub type RXDOUBLEX = crate::Reg<u32, _RXDOUBLEX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDOUBLEX;
#[doc = "`read()` method returns [rxdoublex::R](rxdoublex::R) reader structure"]
impl crate::Readable for RXDOUBLEX {}
#[doc = "RX Buffer Double Data Extended Register"]
pub mod rxdoublex;
#[doc = "RX FIFO Double Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxdouble](rxdouble) module"]
pub type RXDOUBLE = crate::Reg<u32, _RXDOUBLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDOUBLE;
#[doc = "`read()` method returns [rxdouble::R](rxdouble::R) reader structure"]
impl crate::Readable for RXDOUBLE {}
#[doc = "RX FIFO Double Data Register"]
pub mod rxdouble;
#[doc = "RX Buffer Data Extended Peek Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxdataxp](rxdataxp) module"]
pub type RXDATAXP = crate::Reg<u32, _RXDATAXP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDATAXP;
#[doc = "`read()` method returns [rxdataxp::R](rxdataxp::R) reader structure"]
impl crate::Readable for RXDATAXP {}
#[doc = "RX Buffer Data Extended Peek Register"]
pub mod rxdataxp;
#[doc = "RX Buffer Double Data Extended Peek Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxdoublexp](rxdoublexp) module"]
pub type RXDOUBLEXP = crate::Reg<u32, _RXDOUBLEXP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDOUBLEXP;
#[doc = "`read()` method returns [rxdoublexp::R](rxdoublexp::R) reader structure"]
impl crate::Readable for RXDOUBLEXP {}
#[doc = "RX Buffer Double Data Extended Peek Register"]
pub mod rxdoublexp;
#[doc = "TX Buffer Data Extended Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txdatax](txdatax) module"]
pub type TXDATAX = crate::Reg<u32, _TXDATAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXDATAX;
#[doc = "`read()` method returns [txdatax::R](txdatax::R) reader structure"]
impl crate::Readable for TXDATAX {}
#[doc = "`write(|w| ..)` method takes [txdatax::W](txdatax::W) writer structure"]
impl crate::Writable for TXDATAX {}
#[doc = "TX Buffer Data Extended Register"]
pub mod txdatax;
#[doc = "TX Buffer Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txdata](txdata) module"]
pub type TXDATA = crate::Reg<u32, _TXDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXDATA;
#[doc = "`read()` method returns [txdata::R](txdata::R) reader structure"]
impl crate::Readable for TXDATA {}
#[doc = "`write(|w| ..)` method takes [txdata::W](txdata::W) writer structure"]
impl crate::Writable for TXDATA {}
#[doc = "TX Buffer Data Register"]
pub mod txdata;
#[doc = "TX Buffer Double Data Extended Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txdoublex](txdoublex) module"]
pub type TXDOUBLEX = crate::Reg<u32, _TXDOUBLEX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXDOUBLEX;
#[doc = "`read()` method returns [txdoublex::R](txdoublex::R) reader structure"]
impl crate::Readable for TXDOUBLEX {}
#[doc = "`write(|w| ..)` method takes [txdoublex::W](txdoublex::W) writer structure"]
impl crate::Writable for TXDOUBLEX {}
#[doc = "TX Buffer Double Data Extended Register"]
pub mod txdoublex;
#[doc = "TX Buffer Double Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txdouble](txdouble) module"]
pub type TXDOUBLE = crate::Reg<u32, _TXDOUBLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXDOUBLE;
#[doc = "`read()` method returns [txdouble::R](txdouble::R) reader structure"]
impl crate::Readable for TXDOUBLE {}
#[doc = "`write(|w| ..)` method takes [txdouble::W](txdouble::W) writer structure"]
impl crate::Writable for TXDOUBLE {}
#[doc = "TX Buffer Double Data Register"]
pub mod txdouble;
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [if_](if_) module"]
pub type IF = crate::Reg<u32, _IF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF;
#[doc = "`read()` method returns [if_::R](if_::R) reader structure"]
impl crate::Readable for IF {}
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "Interrupt Flag Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ifs](ifs) module"]
pub type IFS = crate::Reg<u32, _IFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFS;
#[doc = "`write(|w| ..)` method takes [ifs::W](ifs::W) writer structure"]
impl crate::Writable for IFS {}
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "Interrupt Flag Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ifc](ifc) module"]
pub type IFC = crate::Reg<u32, _IFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFC;
#[doc = "`write(|w| ..)` method takes [ifc::W](ifc::W) writer structure"]
impl crate::Writable for IFC {}
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ien](ien) module"]
pub type IEN = crate::Reg<u32, _IEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEN;
#[doc = "`read()` method returns [ien::R](ien::R) reader structure"]
impl crate::Readable for IEN {}
#[doc = "`write(|w| ..)` method takes [ien::W](ien::W) writer structure"]
impl crate::Writable for IEN {}
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "IrDA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [irctrl](irctrl) module"]
pub type IRCTRL = crate::Reg<u32, _IRCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRCTRL;
#[doc = "`read()` method returns [irctrl::R](irctrl::R) reader structure"]
impl crate::Readable for IRCTRL {}
#[doc = "`write(|w| ..)` method takes [irctrl::W](irctrl::W) writer structure"]
impl crate::Writable for IRCTRL {}
#[doc = "IrDA Control Register"]
pub mod irctrl;
#[doc = "USART Input Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [input](input) module"]
pub type INPUT = crate::Reg<u32, _INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INPUT;
#[doc = "`read()` method returns [input::R](input::R) reader structure"]
impl crate::Readable for INPUT {}
#[doc = "`write(|w| ..)` method takes [input::W](input::W) writer structure"]
impl crate::Writable for INPUT {}
#[doc = "USART Input Register"]
pub mod input;
#[doc = "I2S Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2sctrl](i2sctrl) module"]
pub type I2SCTRL = crate::Reg<u32, _I2SCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SCTRL;
#[doc = "`read()` method returns [i2sctrl::R](i2sctrl::R) reader structure"]
impl crate::Readable for I2SCTRL {}
#[doc = "`write(|w| ..)` method takes [i2sctrl::W](i2sctrl::W) writer structure"]
impl crate::Writable for I2SCTRL {}
#[doc = "I2S Control Register"]
pub mod i2sctrl;
#[doc = "Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timing](timing) module"]
pub type TIMING = crate::Reg<u32, _TIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMING;
#[doc = "`read()` method returns [timing::R](timing::R) reader structure"]
impl crate::Readable for TIMING {}
#[doc = "`write(|w| ..)` method takes [timing::W](timing::W) writer structure"]
impl crate::Writable for TIMING {}
#[doc = "Timing Register"]
pub mod timing;
#[doc = "Control Register Extended\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrlx](ctrlx) module"]
pub type CTRLX = crate::Reg<u32, _CTRLX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLX;
#[doc = "`read()` method returns [ctrlx::R](ctrlx::R) reader structure"]
impl crate::Readable for CTRLX {}
#[doc = "`write(|w| ..)` method takes [ctrlx::W](ctrlx::W) writer structure"]
impl crate::Writable for CTRLX {}
#[doc = "Control Register Extended"]
pub mod ctrlx;
#[doc = "Used to Generate Interrupts and Various Delays\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timecmp0](timecmp0) module"]
pub type TIMECMP0 = crate::Reg<u32, _TIMECMP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMECMP0;
#[doc = "`read()` method returns [timecmp0::R](timecmp0::R) reader structure"]
impl crate::Readable for TIMECMP0 {}
#[doc = "`write(|w| ..)` method takes [timecmp0::W](timecmp0::W) writer structure"]
impl crate::Writable for TIMECMP0 {}
#[doc = "Used to Generate Interrupts and Various Delays"]
pub mod timecmp0;
#[doc = "Used to Generate Interrupts and Various Delays\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timecmp1](timecmp1) module"]
pub type TIMECMP1 = crate::Reg<u32, _TIMECMP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMECMP1;
#[doc = "`read()` method returns [timecmp1::R](timecmp1::R) reader structure"]
impl crate::Readable for TIMECMP1 {}
#[doc = "`write(|w| ..)` method takes [timecmp1::W](timecmp1::W) writer structure"]
impl crate::Writable for TIMECMP1 {}
#[doc = "Used to Generate Interrupts and Various Delays"]
pub mod timecmp1;
#[doc = "Used to Generate Interrupts and Various Delays\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timecmp2](timecmp2) module"]
pub type TIMECMP2 = crate::Reg<u32, _TIMECMP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMECMP2;
#[doc = "`read()` method returns [timecmp2::R](timecmp2::R) reader structure"]
impl crate::Readable for TIMECMP2 {}
#[doc = "`write(|w| ..)` method takes [timecmp2::W](timecmp2::W) writer structure"]
impl crate::Writable for TIMECMP2 {}
#[doc = "Used to Generate Interrupts and Various Delays"]
pub mod timecmp2;
#[doc = "I/O Routing Pin Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [routepen](routepen) module"]
pub type ROUTEPEN = crate::Reg<u32, _ROUTEPEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTEPEN;
#[doc = "`read()` method returns [routepen::R](routepen::R) reader structure"]
impl crate::Readable for ROUTEPEN {}
#[doc = "`write(|w| ..)` method takes [routepen::W](routepen::W) writer structure"]
impl crate::Writable for ROUTEPEN {}
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "I/O Routing Location Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [routeloc0](routeloc0) module"]
pub type ROUTELOC0 = crate::Reg<u32, _ROUTELOC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTELOC0;
#[doc = "`read()` method returns [routeloc0::R](routeloc0::R) reader structure"]
impl crate::Readable for ROUTELOC0 {}
#[doc = "`write(|w| ..)` method takes [routeloc0::W](routeloc0::W) writer structure"]
impl crate::Writable for ROUTELOC0 {}
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "I/O Routing Location Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [routeloc1](routeloc1) module"]
pub type ROUTELOC1 = crate::Reg<u32, _ROUTELOC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTELOC1;
#[doc = "`read()` method returns [routeloc1::R](routeloc1::R) reader structure"]
impl crate::Readable for ROUTELOC1 {}
#[doc = "`write(|w| ..)` method takes [routeloc1::W](routeloc1::W) writer structure"]
impl crate::Writable for ROUTELOC1 {}
#[doc = "I/O Routing Location Register"]
pub mod routeloc1;
