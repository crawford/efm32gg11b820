#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Main Control Register"]
    pub etmcr: ETMCR,
    #[doc = "0x04 - Configuration Code Register"]
    pub etmccr: ETMCCR,
    #[doc = "0x08 - ETM Trigger Event Register"]
    pub etmtrigger: ETMTRIGGER,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - ETM Status Register"]
    pub etmsr: ETMSR,
    #[doc = "0x14 - ETM System Configuration Register"]
    pub etmscr: ETMSCR,
    _reserved5: [u8; 8usize],
    #[doc = "0x20 - ETM TraceEnable Event Register"]
    pub etmteevr: ETMTEEVR,
    #[doc = "0x24 - ETM Trace control Register"]
    pub etmtecr1: ETMTECR1,
    _reserved7: [u8; 4usize],
    #[doc = "0x2c - ETM Fifo Full Level Register"]
    pub etmfflr: ETMFFLR,
    _reserved8: [u8; 272usize],
    #[doc = "0x140 - Counter Reload Value"]
    pub etmcntrldvr1: ETMCNTRLDVR1,
    _reserved9: [u8; 156usize],
    #[doc = "0x1e0 - Synchronisation Frequency Register"]
    pub etmsyncfr: ETMSYNCFR,
    #[doc = "0x1e4 - ID Register"]
    pub etmidr: ETMIDR,
    #[doc = "0x1e8 - Configuration Code Extension Register"]
    pub etmccer: ETMCCER,
    _reserved12: [u8; 4usize],
    #[doc = "0x1f0 - TraceEnable Start/Stop EmbeddedICE Control Register"]
    pub etmtesseicr: ETMTESSEICR,
    _reserved13: [u8; 4usize],
    #[doc = "0x1f8 - Timestamp Event Register"]
    pub etmtsevr: ETMTSEVR,
    _reserved14: [u8; 4usize],
    #[doc = "0x200 - CoreSight Trace ID Register"]
    pub etmtraceidr: ETMTRACEIDR,
    _reserved15: [u8; 4usize],
    #[doc = "0x208 - ETM ID Register 2"]
    pub etmidr2: ETMIDR2,
    _reserved16: [u8; 264usize],
    #[doc = "0x314 - Device Power-down Status Register"]
    pub etmpdsr: ETMPDSR,
    _reserved17: [u8; 3016usize],
    #[doc = "0xee0 - Integration Test Miscellaneous Inputs Register"]
    pub etmiscin: ETMISCIN,
    _reserved18: [u8; 4usize],
    #[doc = "0xee8 - Integration Test Trigger Out Register"]
    pub ittrigout: ITTRIGOUT,
    _reserved19: [u8; 4usize],
    #[doc = "0xef0 - ETM Integration Test ATB Control 2 Register"]
    pub etmitatbctr2: ETMITATBCTR2,
    _reserved20: [u8; 4usize],
    #[doc = "0xef8 - ETM Integration Test ATB Control 0 Register"]
    pub etmitatbctr0: ETMITATBCTR0,
    _reserved21: [u8; 4usize],
    #[doc = "0xf00 - ETM Integration Control Register"]
    pub etmitctrl: ETMITCTRL,
    _reserved22: [u8; 156usize],
    #[doc = "0xfa0 - ETM Claim Tag Set Register"]
    pub etmclaimset: ETMCLAIMSET,
    #[doc = "0xfa4 - ETM Claim Tag Clear Register"]
    pub etmclaimclr: ETMCLAIMCLR,
    _reserved24: [u8; 8usize],
    #[doc = "0xfb0 - ETM Lock Access Register"]
    pub etmlar: ETMLAR,
    #[doc = "0xfb4 - Lock Status Register"]
    pub etmlsr: ETMLSR,
    #[doc = "0xfb8 - ETM Authentication Status Register"]
    pub etmauthstatus: ETMAUTHSTATUS,
    _reserved27: [u8; 16usize],
    #[doc = "0xfcc - CoreSight Device Type Register"]
    pub etmdevtype: ETMDEVTYPE,
    #[doc = "0xfd0 - Peripheral ID4 Register"]
    pub etmpidr4: ETMPIDR4,
    #[doc = "0xfd4 - Peripheral ID5 Register"]
    pub etmpidr5: ETMPIDR5,
    #[doc = "0xfd8 - Peripheral ID6 Register"]
    pub etmpidr6: ETMPIDR6,
    #[doc = "0xfdc - Peripheral ID7 Register"]
    pub etmpidr7: ETMPIDR7,
    #[doc = "0xfe0 - Peripheral ID0 Register"]
    pub etmpidr0: ETMPIDR0,
    #[doc = "0xfe4 - Peripheral ID1 Register"]
    pub etmpidr1: ETMPIDR1,
    #[doc = "0xfe8 - Peripheral ID2 Register"]
    pub etmpidr2: ETMPIDR2,
    #[doc = "0xfec - Peripheral ID3 Register"]
    pub etmpidr3: ETMPIDR3,
    #[doc = "0xff0 - Component ID0 Register"]
    pub etmcidr0: ETMCIDR0,
    #[doc = "0xff4 - Component ID1 Register"]
    pub etmcidr1: ETMCIDR1,
    #[doc = "0xff8 - Component ID2 Register"]
    pub etmcidr2: ETMCIDR2,
    #[doc = "0xffc - Component ID3 Register"]
    pub etmcidr3: ETMCIDR3,
}
#[doc = "Main Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmcr](etmcr) module"]
pub type ETMCR = crate::Reg<u32, _ETMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMCR;
#[doc = "`read()` method returns [etmcr::R](etmcr::R) reader structure"]
impl crate::Readable for ETMCR {}
#[doc = "`write(|w| ..)` method takes [etmcr::W](etmcr::W) writer structure"]
impl crate::Writable for ETMCR {}
#[doc = "Main Control Register"]
pub mod etmcr;
#[doc = "Configuration Code Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmccr](etmccr) module"]
pub type ETMCCR = crate::Reg<u32, _ETMCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMCCR;
#[doc = "`read()` method returns [etmccr::R](etmccr::R) reader structure"]
impl crate::Readable for ETMCCR {}
#[doc = "Configuration Code Register"]
pub mod etmccr;
#[doc = "ETM Trigger Event Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmtrigger](etmtrigger) module"]
pub type ETMTRIGGER = crate::Reg<u32, _ETMTRIGGER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMTRIGGER;
#[doc = "`read()` method returns [etmtrigger::R](etmtrigger::R) reader structure"]
impl crate::Readable for ETMTRIGGER {}
#[doc = "`write(|w| ..)` method takes [etmtrigger::W](etmtrigger::W) writer structure"]
impl crate::Writable for ETMTRIGGER {}
#[doc = "ETM Trigger Event Register"]
pub mod etmtrigger;
#[doc = "ETM Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmsr](etmsr) module"]
pub type ETMSR = crate::Reg<u32, _ETMSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMSR;
#[doc = "`read()` method returns [etmsr::R](etmsr::R) reader structure"]
impl crate::Readable for ETMSR {}
#[doc = "`write(|w| ..)` method takes [etmsr::W](etmsr::W) writer structure"]
impl crate::Writable for ETMSR {}
#[doc = "ETM Status Register"]
pub mod etmsr;
#[doc = "ETM System Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmscr](etmscr) module"]
pub type ETMSCR = crate::Reg<u32, _ETMSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMSCR;
#[doc = "`read()` method returns [etmscr::R](etmscr::R) reader structure"]
impl crate::Readable for ETMSCR {}
#[doc = "ETM System Configuration Register"]
pub mod etmscr;
#[doc = "ETM TraceEnable Event Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmteevr](etmteevr) module"]
pub type ETMTEEVR = crate::Reg<u32, _ETMTEEVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMTEEVR;
#[doc = "`read()` method returns [etmteevr::R](etmteevr::R) reader structure"]
impl crate::Readable for ETMTEEVR {}
#[doc = "`write(|w| ..)` method takes [etmteevr::W](etmteevr::W) writer structure"]
impl crate::Writable for ETMTEEVR {}
#[doc = "ETM TraceEnable Event Register"]
pub mod etmteevr;
#[doc = "ETM Trace control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmtecr1](etmtecr1) module"]
pub type ETMTECR1 = crate::Reg<u32, _ETMTECR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMTECR1;
#[doc = "`read()` method returns [etmtecr1::R](etmtecr1::R) reader structure"]
impl crate::Readable for ETMTECR1 {}
#[doc = "`write(|w| ..)` method takes [etmtecr1::W](etmtecr1::W) writer structure"]
impl crate::Writable for ETMTECR1 {}
#[doc = "ETM Trace control Register"]
pub mod etmtecr1;
#[doc = "ETM Fifo Full Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmfflr](etmfflr) module"]
pub type ETMFFLR = crate::Reg<u32, _ETMFFLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMFFLR;
#[doc = "`read()` method returns [etmfflr::R](etmfflr::R) reader structure"]
impl crate::Readable for ETMFFLR {}
#[doc = "`write(|w| ..)` method takes [etmfflr::W](etmfflr::W) writer structure"]
impl crate::Writable for ETMFFLR {}
#[doc = "ETM Fifo Full Level Register"]
pub mod etmfflr;
#[doc = "Counter Reload Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmcntrldvr1](etmcntrldvr1) module"]
pub type ETMCNTRLDVR1 = crate::Reg<u32, _ETMCNTRLDVR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMCNTRLDVR1;
#[doc = "`read()` method returns [etmcntrldvr1::R](etmcntrldvr1::R) reader structure"]
impl crate::Readable for ETMCNTRLDVR1 {}
#[doc = "`write(|w| ..)` method takes [etmcntrldvr1::W](etmcntrldvr1::W) writer structure"]
impl crate::Writable for ETMCNTRLDVR1 {}
#[doc = "Counter Reload Value"]
pub mod etmcntrldvr1;
#[doc = "Synchronisation Frequency Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmsyncfr](etmsyncfr) module"]
pub type ETMSYNCFR = crate::Reg<u32, _ETMSYNCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMSYNCFR;
#[doc = "`read()` method returns [etmsyncfr::R](etmsyncfr::R) reader structure"]
impl crate::Readable for ETMSYNCFR {}
#[doc = "`write(|w| ..)` method takes [etmsyncfr::W](etmsyncfr::W) writer structure"]
impl crate::Writable for ETMSYNCFR {}
#[doc = "Synchronisation Frequency Register"]
pub mod etmsyncfr;
#[doc = "ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmidr](etmidr) module"]
pub type ETMIDR = crate::Reg<u32, _ETMIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMIDR;
#[doc = "`read()` method returns [etmidr::R](etmidr::R) reader structure"]
impl crate::Readable for ETMIDR {}
#[doc = "ID Register"]
pub mod etmidr;
#[doc = "Configuration Code Extension Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmccer](etmccer) module"]
pub type ETMCCER = crate::Reg<u32, _ETMCCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMCCER;
#[doc = "`read()` method returns [etmccer::R](etmccer::R) reader structure"]
impl crate::Readable for ETMCCER {}
#[doc = "Configuration Code Extension Register"]
pub mod etmccer;
#[doc = "TraceEnable Start/Stop EmbeddedICE Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmtesseicr](etmtesseicr) module"]
pub type ETMTESSEICR = crate::Reg<u32, _ETMTESSEICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMTESSEICR;
#[doc = "`read()` method returns [etmtesseicr::R](etmtesseicr::R) reader structure"]
impl crate::Readable for ETMTESSEICR {}
#[doc = "`write(|w| ..)` method takes [etmtesseicr::W](etmtesseicr::W) writer structure"]
impl crate::Writable for ETMTESSEICR {}
#[doc = "TraceEnable Start/Stop EmbeddedICE Control Register"]
pub mod etmtesseicr;
#[doc = "Timestamp Event Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmtsevr](etmtsevr) module"]
pub type ETMTSEVR = crate::Reg<u32, _ETMTSEVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMTSEVR;
#[doc = "`read()` method returns [etmtsevr::R](etmtsevr::R) reader structure"]
impl crate::Readable for ETMTSEVR {}
#[doc = "`write(|w| ..)` method takes [etmtsevr::W](etmtsevr::W) writer structure"]
impl crate::Writable for ETMTSEVR {}
#[doc = "Timestamp Event Register"]
pub mod etmtsevr;
#[doc = "CoreSight Trace ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmtraceidr](etmtraceidr) module"]
pub type ETMTRACEIDR = crate::Reg<u32, _ETMTRACEIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMTRACEIDR;
#[doc = "`read()` method returns [etmtraceidr::R](etmtraceidr::R) reader structure"]
impl crate::Readable for ETMTRACEIDR {}
#[doc = "`write(|w| ..)` method takes [etmtraceidr::W](etmtraceidr::W) writer structure"]
impl crate::Writable for ETMTRACEIDR {}
#[doc = "CoreSight Trace ID Register"]
pub mod etmtraceidr;
#[doc = "ETM ID Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmidr2](etmidr2) module"]
pub type ETMIDR2 = crate::Reg<u32, _ETMIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMIDR2;
#[doc = "`read()` method returns [etmidr2::R](etmidr2::R) reader structure"]
impl crate::Readable for ETMIDR2 {}
#[doc = "ETM ID Register 2"]
pub mod etmidr2;
#[doc = "Device Power-down Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmpdsr](etmpdsr) module"]
pub type ETMPDSR = crate::Reg<u32, _ETMPDSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMPDSR;
#[doc = "`read()` method returns [etmpdsr::R](etmpdsr::R) reader structure"]
impl crate::Readable for ETMPDSR {}
#[doc = "Device Power-down Status Register"]
pub mod etmpdsr;
#[doc = "Integration Test Miscellaneous Inputs Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmiscin](etmiscin) module"]
pub type ETMISCIN = crate::Reg<u32, _ETMISCIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMISCIN;
#[doc = "`read()` method returns [etmiscin::R](etmiscin::R) reader structure"]
impl crate::Readable for ETMISCIN {}
#[doc = "`write(|w| ..)` method takes [etmiscin::W](etmiscin::W) writer structure"]
impl crate::Writable for ETMISCIN {}
#[doc = "Integration Test Miscellaneous Inputs Register"]
pub mod etmiscin;
#[doc = "Integration Test Trigger Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ittrigout](ittrigout) module"]
pub type ITTRIGOUT = crate::Reg<u32, _ITTRIGOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITTRIGOUT;
#[doc = "`read()` method returns [ittrigout::R](ittrigout::R) reader structure"]
impl crate::Readable for ITTRIGOUT {}
#[doc = "`write(|w| ..)` method takes [ittrigout::W](ittrigout::W) writer structure"]
impl crate::Writable for ITTRIGOUT {}
#[doc = "Integration Test Trigger Out Register"]
pub mod ittrigout;
#[doc = "ETM Integration Test ATB Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmitatbctr2](etmitatbctr2) module"]
pub type ETMITATBCTR2 = crate::Reg<u32, _ETMITATBCTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMITATBCTR2;
#[doc = "`read()` method returns [etmitatbctr2::R](etmitatbctr2::R) reader structure"]
impl crate::Readable for ETMITATBCTR2 {}
#[doc = "ETM Integration Test ATB Control 2 Register"]
pub mod etmitatbctr2;
#[doc = "ETM Integration Test ATB Control 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmitatbctr0](etmitatbctr0) module"]
pub type ETMITATBCTR0 = crate::Reg<u32, _ETMITATBCTR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMITATBCTR0;
#[doc = "`read()` method returns [etmitatbctr0::R](etmitatbctr0::R) reader structure"]
impl crate::Readable for ETMITATBCTR0 {}
#[doc = "`write(|w| ..)` method takes [etmitatbctr0::W](etmitatbctr0::W) writer structure"]
impl crate::Writable for ETMITATBCTR0 {}
#[doc = "ETM Integration Test ATB Control 0 Register"]
pub mod etmitatbctr0;
#[doc = "ETM Integration Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmitctrl](etmitctrl) module"]
pub type ETMITCTRL = crate::Reg<u32, _ETMITCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMITCTRL;
#[doc = "`read()` method returns [etmitctrl::R](etmitctrl::R) reader structure"]
impl crate::Readable for ETMITCTRL {}
#[doc = "`write(|w| ..)` method takes [etmitctrl::W](etmitctrl::W) writer structure"]
impl crate::Writable for ETMITCTRL {}
#[doc = "ETM Integration Control Register"]
pub mod etmitctrl;
#[doc = "ETM Claim Tag Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmclaimset](etmclaimset) module"]
pub type ETMCLAIMSET = crate::Reg<u32, _ETMCLAIMSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMCLAIMSET;
#[doc = "`read()` method returns [etmclaimset::R](etmclaimset::R) reader structure"]
impl crate::Readable for ETMCLAIMSET {}
#[doc = "`write(|w| ..)` method takes [etmclaimset::W](etmclaimset::W) writer structure"]
impl crate::Writable for ETMCLAIMSET {}
#[doc = "ETM Claim Tag Set Register"]
pub mod etmclaimset;
#[doc = "ETM Claim Tag Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmclaimclr](etmclaimclr) module"]
pub type ETMCLAIMCLR = crate::Reg<u32, _ETMCLAIMCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMCLAIMCLR;
#[doc = "`read()` method returns [etmclaimclr::R](etmclaimclr::R) reader structure"]
impl crate::Readable for ETMCLAIMCLR {}
#[doc = "`write(|w| ..)` method takes [etmclaimclr::W](etmclaimclr::W) writer structure"]
impl crate::Writable for ETMCLAIMCLR {}
#[doc = "ETM Claim Tag Clear Register"]
pub mod etmclaimclr;
#[doc = "ETM Lock Access Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmlar](etmlar) module"]
pub type ETMLAR = crate::Reg<u32, _ETMLAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMLAR;
#[doc = "`read()` method returns [etmlar::R](etmlar::R) reader structure"]
impl crate::Readable for ETMLAR {}
#[doc = "`write(|w| ..)` method takes [etmlar::W](etmlar::W) writer structure"]
impl crate::Writable for ETMLAR {}
#[doc = "ETM Lock Access Register"]
pub mod etmlar;
#[doc = "Lock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmlsr](etmlsr) module"]
pub type ETMLSR = crate::Reg<u32, _ETMLSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMLSR;
#[doc = "`read()` method returns [etmlsr::R](etmlsr::R) reader structure"]
impl crate::Readable for ETMLSR {}
#[doc = "Lock Status Register"]
pub mod etmlsr;
#[doc = "ETM Authentication Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmauthstatus](etmauthstatus) module"]
pub type ETMAUTHSTATUS = crate::Reg<u32, _ETMAUTHSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMAUTHSTATUS;
#[doc = "`read()` method returns [etmauthstatus::R](etmauthstatus::R) reader structure"]
impl crate::Readable for ETMAUTHSTATUS {}
#[doc = "ETM Authentication Status Register"]
pub mod etmauthstatus;
#[doc = "CoreSight Device Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmdevtype](etmdevtype) module"]
pub type ETMDEVTYPE = crate::Reg<u32, _ETMDEVTYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMDEVTYPE;
#[doc = "`read()` method returns [etmdevtype::R](etmdevtype::R) reader structure"]
impl crate::Readable for ETMDEVTYPE {}
#[doc = "CoreSight Device Type Register"]
pub mod etmdevtype;
#[doc = "Peripheral ID4 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmpidr4](etmpidr4) module"]
pub type ETMPIDR4 = crate::Reg<u32, _ETMPIDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMPIDR4;
#[doc = "`read()` method returns [etmpidr4::R](etmpidr4::R) reader structure"]
impl crate::Readable for ETMPIDR4 {}
#[doc = "Peripheral ID4 Register"]
pub mod etmpidr4;
#[doc = "Peripheral ID5 Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmpidr5](etmpidr5) module"]
pub type ETMPIDR5 = crate::Reg<u32, _ETMPIDR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMPIDR5;
#[doc = "`write(|w| ..)` method takes [etmpidr5::W](etmpidr5::W) writer structure"]
impl crate::Writable for ETMPIDR5 {}
#[doc = "Peripheral ID5 Register"]
pub mod etmpidr5;
#[doc = "Peripheral ID6 Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmpidr6](etmpidr6) module"]
pub type ETMPIDR6 = crate::Reg<u32, _ETMPIDR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMPIDR6;
#[doc = "`write(|w| ..)` method takes [etmpidr6::W](etmpidr6::W) writer structure"]
impl crate::Writable for ETMPIDR6 {}
#[doc = "Peripheral ID6 Register"]
pub mod etmpidr6;
#[doc = "Peripheral ID7 Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmpidr7](etmpidr7) module"]
pub type ETMPIDR7 = crate::Reg<u32, _ETMPIDR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMPIDR7;
#[doc = "`write(|w| ..)` method takes [etmpidr7::W](etmpidr7::W) writer structure"]
impl crate::Writable for ETMPIDR7 {}
#[doc = "Peripheral ID7 Register"]
pub mod etmpidr7;
#[doc = "Peripheral ID0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmpidr0](etmpidr0) module"]
pub type ETMPIDR0 = crate::Reg<u32, _ETMPIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMPIDR0;
#[doc = "`read()` method returns [etmpidr0::R](etmpidr0::R) reader structure"]
impl crate::Readable for ETMPIDR0 {}
#[doc = "Peripheral ID0 Register"]
pub mod etmpidr0;
#[doc = "Peripheral ID1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmpidr1](etmpidr1) module"]
pub type ETMPIDR1 = crate::Reg<u32, _ETMPIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMPIDR1;
#[doc = "`read()` method returns [etmpidr1::R](etmpidr1::R) reader structure"]
impl crate::Readable for ETMPIDR1 {}
#[doc = "Peripheral ID1 Register"]
pub mod etmpidr1;
#[doc = "Peripheral ID2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmpidr2](etmpidr2) module"]
pub type ETMPIDR2 = crate::Reg<u32, _ETMPIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMPIDR2;
#[doc = "`read()` method returns [etmpidr2::R](etmpidr2::R) reader structure"]
impl crate::Readable for ETMPIDR2 {}
#[doc = "Peripheral ID2 Register"]
pub mod etmpidr2;
#[doc = "Peripheral ID3 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmpidr3](etmpidr3) module"]
pub type ETMPIDR3 = crate::Reg<u32, _ETMPIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMPIDR3;
#[doc = "`read()` method returns [etmpidr3::R](etmpidr3::R) reader structure"]
impl crate::Readable for ETMPIDR3 {}
#[doc = "Peripheral ID3 Register"]
pub mod etmpidr3;
#[doc = "Component ID0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmcidr0](etmcidr0) module"]
pub type ETMCIDR0 = crate::Reg<u32, _ETMCIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMCIDR0;
#[doc = "`read()` method returns [etmcidr0::R](etmcidr0::R) reader structure"]
impl crate::Readable for ETMCIDR0 {}
#[doc = "Component ID0 Register"]
pub mod etmcidr0;
#[doc = "Component ID1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmcidr1](etmcidr1) module"]
pub type ETMCIDR1 = crate::Reg<u32, _ETMCIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMCIDR1;
#[doc = "`read()` method returns [etmcidr1::R](etmcidr1::R) reader structure"]
impl crate::Readable for ETMCIDR1 {}
#[doc = "Component ID1 Register"]
pub mod etmcidr1;
#[doc = "Component ID2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmcidr2](etmcidr2) module"]
pub type ETMCIDR2 = crate::Reg<u32, _ETMCIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMCIDR2;
#[doc = "`read()` method returns [etmcidr2::R](etmcidr2::R) reader structure"]
impl crate::Readable for ETMCIDR2 {}
#[doc = "Component ID2 Register"]
pub mod etmcidr2;
#[doc = "Component ID3 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etmcidr3](etmcidr3) module"]
pub type ETMCIDR3 = crate::Reg<u32, _ETMCIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETMCIDR3;
#[doc = "`read()` method returns [etmcidr3::R](etmcidr3::R) reader structure"]
impl crate::Readable for ETMCIDR3 {}
#[doc = "Component ID3 Register"]
pub mod etmcidr3;
