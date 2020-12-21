#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMU Control Register"]
    pub ctrl: CTRL,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - USHFRCO Control Register"]
    pub ushfrcoctrl: USHFRCOCTRL,
    _reserved2: [u8; 4usize],
    #[doc = "0x10 - HFRCO Control Register"]
    pub hfrcoctrl: HFRCOCTRL,
    _reserved3: [u8; 4usize],
    #[doc = "0x18 - AUXHFRCO Control Register"]
    pub auxhfrcoctrl: AUXHFRCOCTRL,
    _reserved4: [u8; 4usize],
    #[doc = "0x20 - LFRCO Control Register"]
    pub lfrcoctrl: LFRCOCTRL,
    #[doc = "0x24 - HFXO Control Register"]
    pub hfxoctrl: HFXOCTRL,
    #[doc = "0x28 - HFXO Control 1"]
    pub hfxoctrl1: HFXOCTRL1,
    #[doc = "0x2c - HFXO Startup Control"]
    pub hfxostartupctrl: HFXOSTARTUPCTRL,
    #[doc = "0x30 - HFXO Steady State Control"]
    pub hfxosteadystatectrl: HFXOSTEADYSTATECTRL,
    #[doc = "0x34 - HFXO Timeout Control"]
    pub hfxotimeoutctrl: HFXOTIMEOUTCTRL,
    #[doc = "0x38 - LFXO Control Register"]
    pub lfxoctrl: LFXOCTRL,
    _reserved11: [u8; 4usize],
    #[doc = "0x40 - DPLL Control Register"]
    pub dpllctrl: DPLLCTRL,
    #[doc = "0x44 - DPLL Control Register"]
    pub dpllctrl1: DPLLCTRL1,
    _reserved13: [u8; 8usize],
    #[doc = "0x50 - Calibration Control Register"]
    pub calctrl: CALCTRL,
    #[doc = "0x54 - Calibration Counter Register"]
    pub calcnt: CALCNT,
    _reserved15: [u8; 8usize],
    #[doc = "0x60 - Oscillator Enable/Disable Command Register"]
    pub oscencmd: OSCENCMD,
    #[doc = "0x64 - Command Register"]
    pub cmd: CMD,
    _reserved17: [u8; 8usize],
    #[doc = "0x70 - Debug Trace Clock Select"]
    pub dbgclksel: DBGCLKSEL,
    #[doc = "0x74 - High Frequency Clock Select Command Register"]
    pub hfclksel: HFCLKSEL,
    _reserved19: [u8; 8usize],
    #[doc = "0x80 - Low Frequency A Clock Select Register"]
    pub lfaclksel: LFACLKSEL,
    #[doc = "0x84 - Low Frequency B Clock Select Register"]
    pub lfbclksel: LFBCLKSEL,
    #[doc = "0x88 - Low Frequency E Clock Select Register"]
    pub lfeclksel: LFECLKSEL,
    #[doc = "0x8c - Low Frequency C Clock Select Register"]
    pub lfcclksel: LFCCLKSEL,
    #[doc = "0x90 - Status Register"]
    pub status: STATUS,
    #[doc = "0x94 - HFCLK Status Register"]
    pub hfclkstatus: HFCLKSTATUS,
    _reserved25: [u8; 4usize],
    #[doc = "0x9c - HFXO Trim Status"]
    pub hfxotrimstatus: HFXOTRIMSTATUS,
    #[doc = "0xa0 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0xa4 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0xa8 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0xac - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0xb0 - High Frequency Bus Clock Enable Register 0"]
    pub hfbusclken0: HFBUSCLKEN0,
    _reserved31: [u8; 12usize],
    #[doc = "0xc0 - High Frequency Peripheral Clock Enable Register 0"]
    pub hfperclken0: HFPERCLKEN0,
    #[doc = "0xc4 - High Frequency Peripheral Clock Enable Register 1"]
    pub hfperclken1: HFPERCLKEN1,
    _reserved33: [u8; 24usize],
    #[doc = "0xe0 - Low Frequency a Clock Enable Register 0 (Async Reg)"]
    pub lfaclken0: LFACLKEN0,
    _reserved34: [u8; 4usize],
    #[doc = "0xe8 - Low Frequency B Clock Enable Register 0 (Async Reg)"]
    pub lfbclken0: LFBCLKEN0,
    #[doc = "0xec - Low Frequency C Clock Enable Register 0 (Async Reg)"]
    pub lfcclken0: LFCCLKEN0,
    #[doc = "0xf0 - Low Frequency E Clock Enable Register 0 (Async Reg)"]
    pub lfeclken0: LFECLKEN0,
    _reserved37: [u8; 12usize],
    #[doc = "0x100 - High Frequency Clock Prescaler Register"]
    pub hfpresc: HFPRESC,
    #[doc = "0x104 - High Frequency Bus Clock Prescaler Register"]
    pub hfbuspresc: HFBUSPRESC,
    #[doc = "0x108 - High Frequency Core Clock Prescaler Register"]
    pub hfcorepresc: HFCOREPRESC,
    #[doc = "0x10c - High Frequency Peripheral Clock Prescaler Register"]
    pub hfperpresc: HFPERPRESC,
    _reserved41: [u8; 4usize],
    #[doc = "0x114 - High Frequency Export Clock Prescaler Register"]
    pub hfexppresc: HFEXPPRESC,
    #[doc = "0x118 - High Frequency Peripheral Clock Prescaler B Register"]
    pub hfperprescb: HFPERPRESCB,
    #[doc = "0x11c - High Frequency Peripheral Clock Prescaler C Register"]
    pub hfperprescc: HFPERPRESCC,
    #[doc = "0x120 - Low Frequency a Prescaler Register 0 (Async Reg)"]
    pub lfapresc0: LFAPRESC0,
    _reserved45: [u8; 4usize],
    #[doc = "0x128 - Low Frequency B Prescaler Register 0 (Async Reg)"]
    pub lfbpresc0: LFBPRESC0,
    _reserved46: [u8; 4usize],
    #[doc = "0x130 - Low Frequency E Prescaler Register 0 (Async Reg)"]
    pub lfepresc0: LFEPRESC0,
    _reserved47: [u8; 12usize],
    #[doc = "0x140 - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x144 - Freeze Register"]
    pub freeze: FREEZE,
    _reserved49: [u8; 8usize],
    #[doc = "0x150 - PCNT Control Register"]
    pub pcntctrl: PCNTCTRL,
    _reserved50: [u8; 8usize],
    #[doc = "0x15c - ADC Control Register"]
    pub adcctrl: ADCCTRL,
    #[doc = "0x160 - SDIO Control Register"]
    pub sdioctrl: SDIOCTRL,
    #[doc = "0x164 - QSPI Control Register"]
    pub qspictrl: QSPICTRL,
    _reserved53: [u8; 8usize],
    #[doc = "0x170 - I/O Routing Pin Enable Register"]
    pub routepen: ROUTEPEN,
    #[doc = "0x174 - I/O Routing Location Register"]
    pub routeloc0: ROUTELOC0,
    #[doc = "0x178 - I/O Routing Location Register"]
    pub routeloc1: ROUTELOC1,
    _reserved56: [u8; 4usize],
    #[doc = "0x180 - Configuration Lock Register"]
    pub lock: LOCK,
    #[doc = "0x184 - HFRCO Spread Spectrum Register"]
    pub hfrcoss: HFRCOSS,
    _reserved58: [u8; 104usize],
    #[doc = "0x1f0 - USB Control Register"]
    pub usbctrl: USBCTRL,
    #[doc = "0x1f4 - USB Clock Recovery Control"]
    pub usbcrctrl: USBCRCTRL,
}
#[doc = "CMU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "CMU Control Register"]
pub mod ctrl;
#[doc = "USHFRCO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ushfrcoctrl](ushfrcoctrl) module"]
pub type USHFRCOCTRL = crate::Reg<u32, _USHFRCOCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USHFRCOCTRL;
#[doc = "`read()` method returns [ushfrcoctrl::R](ushfrcoctrl::R) reader structure"]
impl crate::Readable for USHFRCOCTRL {}
#[doc = "`write(|w| ..)` method takes [ushfrcoctrl::W](ushfrcoctrl::W) writer structure"]
impl crate::Writable for USHFRCOCTRL {}
#[doc = "USHFRCO Control Register"]
pub mod ushfrcoctrl;
#[doc = "HFRCO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfrcoctrl](hfrcoctrl) module"]
pub type HFRCOCTRL = crate::Reg<u32, _HFRCOCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFRCOCTRL;
#[doc = "`read()` method returns [hfrcoctrl::R](hfrcoctrl::R) reader structure"]
impl crate::Readable for HFRCOCTRL {}
#[doc = "`write(|w| ..)` method takes [hfrcoctrl::W](hfrcoctrl::W) writer structure"]
impl crate::Writable for HFRCOCTRL {}
#[doc = "HFRCO Control Register"]
pub mod hfrcoctrl;
#[doc = "AUXHFRCO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxhfrcoctrl](auxhfrcoctrl) module"]
pub type AUXHFRCOCTRL = crate::Reg<u32, _AUXHFRCOCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUXHFRCOCTRL;
#[doc = "`read()` method returns [auxhfrcoctrl::R](auxhfrcoctrl::R) reader structure"]
impl crate::Readable for AUXHFRCOCTRL {}
#[doc = "`write(|w| ..)` method takes [auxhfrcoctrl::W](auxhfrcoctrl::W) writer structure"]
impl crate::Writable for AUXHFRCOCTRL {}
#[doc = "AUXHFRCO Control Register"]
pub mod auxhfrcoctrl;
#[doc = "LFRCO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfrcoctrl](lfrcoctrl) module"]
pub type LFRCOCTRL = crate::Reg<u32, _LFRCOCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFRCOCTRL;
#[doc = "`read()` method returns [lfrcoctrl::R](lfrcoctrl::R) reader structure"]
impl crate::Readable for LFRCOCTRL {}
#[doc = "`write(|w| ..)` method takes [lfrcoctrl::W](lfrcoctrl::W) writer structure"]
impl crate::Writable for LFRCOCTRL {}
#[doc = "LFRCO Control Register"]
pub mod lfrcoctrl;
#[doc = "HFXO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfxoctrl](hfxoctrl) module"]
pub type HFXOCTRL = crate::Reg<u32, _HFXOCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFXOCTRL;
#[doc = "`read()` method returns [hfxoctrl::R](hfxoctrl::R) reader structure"]
impl crate::Readable for HFXOCTRL {}
#[doc = "`write(|w| ..)` method takes [hfxoctrl::W](hfxoctrl::W) writer structure"]
impl crate::Writable for HFXOCTRL {}
#[doc = "HFXO Control Register"]
pub mod hfxoctrl;
#[doc = "HFXO Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfxoctrl1](hfxoctrl1) module"]
pub type HFXOCTRL1 = crate::Reg<u32, _HFXOCTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFXOCTRL1;
#[doc = "`read()` method returns [hfxoctrl1::R](hfxoctrl1::R) reader structure"]
impl crate::Readable for HFXOCTRL1 {}
#[doc = "`write(|w| ..)` method takes [hfxoctrl1::W](hfxoctrl1::W) writer structure"]
impl crate::Writable for HFXOCTRL1 {}
#[doc = "HFXO Control 1"]
pub mod hfxoctrl1;
#[doc = "HFXO Startup Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfxostartupctrl](hfxostartupctrl) module"]
pub type HFXOSTARTUPCTRL = crate::Reg<u32, _HFXOSTARTUPCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFXOSTARTUPCTRL;
#[doc = "`read()` method returns [hfxostartupctrl::R](hfxostartupctrl::R) reader structure"]
impl crate::Readable for HFXOSTARTUPCTRL {}
#[doc = "`write(|w| ..)` method takes [hfxostartupctrl::W](hfxostartupctrl::W) writer structure"]
impl crate::Writable for HFXOSTARTUPCTRL {}
#[doc = "HFXO Startup Control"]
pub mod hfxostartupctrl;
#[doc = "HFXO Steady State Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfxosteadystatectrl](hfxosteadystatectrl) module"]
pub type HFXOSTEADYSTATECTRL = crate::Reg<u32, _HFXOSTEADYSTATECTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFXOSTEADYSTATECTRL;
#[doc = "`read()` method returns [hfxosteadystatectrl::R](hfxosteadystatectrl::R) reader structure"]
impl crate::Readable for HFXOSTEADYSTATECTRL {}
#[doc = "`write(|w| ..)` method takes [hfxosteadystatectrl::W](hfxosteadystatectrl::W) writer structure"]
impl crate::Writable for HFXOSTEADYSTATECTRL {}
#[doc = "HFXO Steady State Control"]
pub mod hfxosteadystatectrl;
#[doc = "HFXO Timeout Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfxotimeoutctrl](hfxotimeoutctrl) module"]
pub type HFXOTIMEOUTCTRL = crate::Reg<u32, _HFXOTIMEOUTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFXOTIMEOUTCTRL;
#[doc = "`read()` method returns [hfxotimeoutctrl::R](hfxotimeoutctrl::R) reader structure"]
impl crate::Readable for HFXOTIMEOUTCTRL {}
#[doc = "`write(|w| ..)` method takes [hfxotimeoutctrl::W](hfxotimeoutctrl::W) writer structure"]
impl crate::Writable for HFXOTIMEOUTCTRL {}
#[doc = "HFXO Timeout Control"]
pub mod hfxotimeoutctrl;
#[doc = "LFXO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfxoctrl](lfxoctrl) module"]
pub type LFXOCTRL = crate::Reg<u32, _LFXOCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFXOCTRL;
#[doc = "`read()` method returns [lfxoctrl::R](lfxoctrl::R) reader structure"]
impl crate::Readable for LFXOCTRL {}
#[doc = "`write(|w| ..)` method takes [lfxoctrl::W](lfxoctrl::W) writer structure"]
impl crate::Writable for LFXOCTRL {}
#[doc = "LFXO Control Register"]
pub mod lfxoctrl;
#[doc = "DPLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllctrl](dpllctrl) module"]
pub type DPLLCTRL = crate::Reg<u32, _DPLLCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPLLCTRL;
#[doc = "`read()` method returns [dpllctrl::R](dpllctrl::R) reader structure"]
impl crate::Readable for DPLLCTRL {}
#[doc = "`write(|w| ..)` method takes [dpllctrl::W](dpllctrl::W) writer structure"]
impl crate::Writable for DPLLCTRL {}
#[doc = "DPLL Control Register"]
pub mod dpllctrl;
#[doc = "DPLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllctrl1](dpllctrl1) module"]
pub type DPLLCTRL1 = crate::Reg<u32, _DPLLCTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPLLCTRL1;
#[doc = "`read()` method returns [dpllctrl1::R](dpllctrl1::R) reader structure"]
impl crate::Readable for DPLLCTRL1 {}
#[doc = "`write(|w| ..)` method takes [dpllctrl1::W](dpllctrl1::W) writer structure"]
impl crate::Writable for DPLLCTRL1 {}
#[doc = "DPLL Control Register"]
pub mod dpllctrl1;
#[doc = "Calibration Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calctrl](calctrl) module"]
pub type CALCTRL = crate::Reg<u32, _CALCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALCTRL;
#[doc = "`read()` method returns [calctrl::R](calctrl::R) reader structure"]
impl crate::Readable for CALCTRL {}
#[doc = "`write(|w| ..)` method takes [calctrl::W](calctrl::W) writer structure"]
impl crate::Writable for CALCTRL {}
#[doc = "Calibration Control Register"]
pub mod calctrl;
#[doc = "Calibration Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calcnt](calcnt) module"]
pub type CALCNT = crate::Reg<u32, _CALCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALCNT;
#[doc = "`read()` method returns [calcnt::R](calcnt::R) reader structure"]
impl crate::Readable for CALCNT {}
#[doc = "`write(|w| ..)` method takes [calcnt::W](calcnt::W) writer structure"]
impl crate::Writable for CALCNT {}
#[doc = "Calibration Counter Register"]
pub mod calcnt;
#[doc = "Oscillator Enable/Disable Command Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscencmd](oscencmd) module"]
pub type OSCENCMD = crate::Reg<u32, _OSCENCMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCENCMD;
#[doc = "`write(|w| ..)` method takes [oscencmd::W](oscencmd::W) writer structure"]
impl crate::Writable for OSCENCMD {}
#[doc = "Oscillator Enable/Disable Command Register"]
pub mod oscencmd;
#[doc = "Command Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "Debug Trace Clock Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgclksel](dbgclksel) module"]
pub type DBGCLKSEL = crate::Reg<u32, _DBGCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGCLKSEL;
#[doc = "`read()` method returns [dbgclksel::R](dbgclksel::R) reader structure"]
impl crate::Readable for DBGCLKSEL {}
#[doc = "`write(|w| ..)` method takes [dbgclksel::W](dbgclksel::W) writer structure"]
impl crate::Writable for DBGCLKSEL {}
#[doc = "Debug Trace Clock Select"]
pub mod dbgclksel;
#[doc = "High Frequency Clock Select Command Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfclksel](hfclksel) module"]
pub type HFCLKSEL = crate::Reg<u32, _HFCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFCLKSEL;
#[doc = "`write(|w| ..)` method takes [hfclksel::W](hfclksel::W) writer structure"]
impl crate::Writable for HFCLKSEL {}
#[doc = "High Frequency Clock Select Command Register"]
pub mod hfclksel;
#[doc = "Low Frequency A Clock Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfaclksel](lfaclksel) module"]
pub type LFACLKSEL = crate::Reg<u32, _LFACLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFACLKSEL;
#[doc = "`read()` method returns [lfaclksel::R](lfaclksel::R) reader structure"]
impl crate::Readable for LFACLKSEL {}
#[doc = "`write(|w| ..)` method takes [lfaclksel::W](lfaclksel::W) writer structure"]
impl crate::Writable for LFACLKSEL {}
#[doc = "Low Frequency A Clock Select Register"]
pub mod lfaclksel;
#[doc = "Low Frequency B Clock Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfbclksel](lfbclksel) module"]
pub type LFBCLKSEL = crate::Reg<u32, _LFBCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFBCLKSEL;
#[doc = "`read()` method returns [lfbclksel::R](lfbclksel::R) reader structure"]
impl crate::Readable for LFBCLKSEL {}
#[doc = "`write(|w| ..)` method takes [lfbclksel::W](lfbclksel::W) writer structure"]
impl crate::Writable for LFBCLKSEL {}
#[doc = "Low Frequency B Clock Select Register"]
pub mod lfbclksel;
#[doc = "Low Frequency E Clock Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfeclksel](lfeclksel) module"]
pub type LFECLKSEL = crate::Reg<u32, _LFECLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFECLKSEL;
#[doc = "`read()` method returns [lfeclksel::R](lfeclksel::R) reader structure"]
impl crate::Readable for LFECLKSEL {}
#[doc = "`write(|w| ..)` method takes [lfeclksel::W](lfeclksel::W) writer structure"]
impl crate::Writable for LFECLKSEL {}
#[doc = "Low Frequency E Clock Select Register"]
pub mod lfeclksel;
#[doc = "Low Frequency C Clock Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfcclksel](lfcclksel) module"]
pub type LFCCLKSEL = crate::Reg<u32, _LFCCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFCCLKSEL;
#[doc = "`read()` method returns [lfcclksel::R](lfcclksel::R) reader structure"]
impl crate::Readable for LFCCLKSEL {}
#[doc = "`write(|w| ..)` method takes [lfcclksel::W](lfcclksel::W) writer structure"]
impl crate::Writable for LFCCLKSEL {}
#[doc = "Low Frequency C Clock Select Register"]
pub mod lfcclksel;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status Register"]
pub mod status;
#[doc = "HFCLK Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfclkstatus](hfclkstatus) module"]
pub type HFCLKSTATUS = crate::Reg<u32, _HFCLKSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFCLKSTATUS;
#[doc = "`read()` method returns [hfclkstatus::R](hfclkstatus::R) reader structure"]
impl crate::Readable for HFCLKSTATUS {}
#[doc = "HFCLK Status Register"]
pub mod hfclkstatus;
#[doc = "HFXO Trim Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfxotrimstatus](hfxotrimstatus) module"]
pub type HFXOTRIMSTATUS = crate::Reg<u32, _HFXOTRIMSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFXOTRIMSTATUS;
#[doc = "`read()` method returns [hfxotrimstatus::R](hfxotrimstatus::R) reader structure"]
impl crate::Readable for HFXOTRIMSTATUS {}
#[doc = "HFXO Trim Status"]
pub mod hfxotrimstatus;
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](if_) module"]
pub type IF = crate::Reg<u32, _IF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF;
#[doc = "`read()` method returns [if_::R](if_::R) reader structure"]
impl crate::Readable for IF {}
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "Interrupt Flag Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs](ifs) module"]
pub type IFS = crate::Reg<u32, _IFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFS;
#[doc = "`write(|w| ..)` method takes [ifs::W](ifs::W) writer structure"]
impl crate::Writable for IFS {}
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "Interrupt Flag Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifc](ifc) module"]
pub type IFC = crate::Reg<u32, _IFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFC;
#[doc = "`write(|w| ..)` method takes [ifc::W](ifc::W) writer structure"]
impl crate::Writable for IFC {}
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](ien) module"]
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
#[doc = "High Frequency Bus Clock Enable Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfbusclken0](hfbusclken0) module"]
pub type HFBUSCLKEN0 = crate::Reg<u32, _HFBUSCLKEN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFBUSCLKEN0;
#[doc = "`read()` method returns [hfbusclken0::R](hfbusclken0::R) reader structure"]
impl crate::Readable for HFBUSCLKEN0 {}
#[doc = "`write(|w| ..)` method takes [hfbusclken0::W](hfbusclken0::W) writer structure"]
impl crate::Writable for HFBUSCLKEN0 {}
#[doc = "High Frequency Bus Clock Enable Register 0"]
pub mod hfbusclken0;
#[doc = "High Frequency Peripheral Clock Enable Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfperclken0](hfperclken0) module"]
pub type HFPERCLKEN0 = crate::Reg<u32, _HFPERCLKEN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFPERCLKEN0;
#[doc = "`read()` method returns [hfperclken0::R](hfperclken0::R) reader structure"]
impl crate::Readable for HFPERCLKEN0 {}
#[doc = "`write(|w| ..)` method takes [hfperclken0::W](hfperclken0::W) writer structure"]
impl crate::Writable for HFPERCLKEN0 {}
#[doc = "High Frequency Peripheral Clock Enable Register 0"]
pub mod hfperclken0;
#[doc = "High Frequency Peripheral Clock Enable Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfperclken1](hfperclken1) module"]
pub type HFPERCLKEN1 = crate::Reg<u32, _HFPERCLKEN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFPERCLKEN1;
#[doc = "`read()` method returns [hfperclken1::R](hfperclken1::R) reader structure"]
impl crate::Readable for HFPERCLKEN1 {}
#[doc = "`write(|w| ..)` method takes [hfperclken1::W](hfperclken1::W) writer structure"]
impl crate::Writable for HFPERCLKEN1 {}
#[doc = "High Frequency Peripheral Clock Enable Register 1"]
pub mod hfperclken1;
#[doc = "Low Frequency a Clock Enable Register 0 (Async Reg)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfaclken0](lfaclken0) module"]
pub type LFACLKEN0 = crate::Reg<u32, _LFACLKEN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFACLKEN0;
#[doc = "`read()` method returns [lfaclken0::R](lfaclken0::R) reader structure"]
impl crate::Readable for LFACLKEN0 {}
#[doc = "`write(|w| ..)` method takes [lfaclken0::W](lfaclken0::W) writer structure"]
impl crate::Writable for LFACLKEN0 {}
#[doc = "Low Frequency a Clock Enable Register 0 (Async Reg)"]
pub mod lfaclken0;
#[doc = "Low Frequency B Clock Enable Register 0 (Async Reg)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfbclken0](lfbclken0) module"]
pub type LFBCLKEN0 = crate::Reg<u32, _LFBCLKEN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFBCLKEN0;
#[doc = "`read()` method returns [lfbclken0::R](lfbclken0::R) reader structure"]
impl crate::Readable for LFBCLKEN0 {}
#[doc = "`write(|w| ..)` method takes [lfbclken0::W](lfbclken0::W) writer structure"]
impl crate::Writable for LFBCLKEN0 {}
#[doc = "Low Frequency B Clock Enable Register 0 (Async Reg)"]
pub mod lfbclken0;
#[doc = "Low Frequency C Clock Enable Register 0 (Async Reg)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfcclken0](lfcclken0) module"]
pub type LFCCLKEN0 = crate::Reg<u32, _LFCCLKEN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFCCLKEN0;
#[doc = "`read()` method returns [lfcclken0::R](lfcclken0::R) reader structure"]
impl crate::Readable for LFCCLKEN0 {}
#[doc = "`write(|w| ..)` method takes [lfcclken0::W](lfcclken0::W) writer structure"]
impl crate::Writable for LFCCLKEN0 {}
#[doc = "Low Frequency C Clock Enable Register 0 (Async Reg)"]
pub mod lfcclken0;
#[doc = "Low Frequency E Clock Enable Register 0 (Async Reg)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfeclken0](lfeclken0) module"]
pub type LFECLKEN0 = crate::Reg<u32, _LFECLKEN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFECLKEN0;
#[doc = "`read()` method returns [lfeclken0::R](lfeclken0::R) reader structure"]
impl crate::Readable for LFECLKEN0 {}
#[doc = "`write(|w| ..)` method takes [lfeclken0::W](lfeclken0::W) writer structure"]
impl crate::Writable for LFECLKEN0 {}
#[doc = "Low Frequency E Clock Enable Register 0 (Async Reg)"]
pub mod lfeclken0;
#[doc = "High Frequency Clock Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfpresc](hfpresc) module"]
pub type HFPRESC = crate::Reg<u32, _HFPRESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFPRESC;
#[doc = "`read()` method returns [hfpresc::R](hfpresc::R) reader structure"]
impl crate::Readable for HFPRESC {}
#[doc = "`write(|w| ..)` method takes [hfpresc::W](hfpresc::W) writer structure"]
impl crate::Writable for HFPRESC {}
#[doc = "High Frequency Clock Prescaler Register"]
pub mod hfpresc;
#[doc = "High Frequency Bus Clock Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfbuspresc](hfbuspresc) module"]
pub type HFBUSPRESC = crate::Reg<u32, _HFBUSPRESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFBUSPRESC;
#[doc = "`read()` method returns [hfbuspresc::R](hfbuspresc::R) reader structure"]
impl crate::Readable for HFBUSPRESC {}
#[doc = "`write(|w| ..)` method takes [hfbuspresc::W](hfbuspresc::W) writer structure"]
impl crate::Writable for HFBUSPRESC {}
#[doc = "High Frequency Bus Clock Prescaler Register"]
pub mod hfbuspresc;
#[doc = "High Frequency Core Clock Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfcorepresc](hfcorepresc) module"]
pub type HFCOREPRESC = crate::Reg<u32, _HFCOREPRESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFCOREPRESC;
#[doc = "`read()` method returns [hfcorepresc::R](hfcorepresc::R) reader structure"]
impl crate::Readable for HFCOREPRESC {}
#[doc = "`write(|w| ..)` method takes [hfcorepresc::W](hfcorepresc::W) writer structure"]
impl crate::Writable for HFCOREPRESC {}
#[doc = "High Frequency Core Clock Prescaler Register"]
pub mod hfcorepresc;
#[doc = "High Frequency Peripheral Clock Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfperpresc](hfperpresc) module"]
pub type HFPERPRESC = crate::Reg<u32, _HFPERPRESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFPERPRESC;
#[doc = "`read()` method returns [hfperpresc::R](hfperpresc::R) reader structure"]
impl crate::Readable for HFPERPRESC {}
#[doc = "`write(|w| ..)` method takes [hfperpresc::W](hfperpresc::W) writer structure"]
impl crate::Writable for HFPERPRESC {}
#[doc = "High Frequency Peripheral Clock Prescaler Register"]
pub mod hfperpresc;
#[doc = "High Frequency Export Clock Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfexppresc](hfexppresc) module"]
pub type HFEXPPRESC = crate::Reg<u32, _HFEXPPRESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFEXPPRESC;
#[doc = "`read()` method returns [hfexppresc::R](hfexppresc::R) reader structure"]
impl crate::Readable for HFEXPPRESC {}
#[doc = "`write(|w| ..)` method takes [hfexppresc::W](hfexppresc::W) writer structure"]
impl crate::Writable for HFEXPPRESC {}
#[doc = "High Frequency Export Clock Prescaler Register"]
pub mod hfexppresc;
#[doc = "High Frequency Peripheral Clock Prescaler B Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfperprescb](hfperprescb) module"]
pub type HFPERPRESCB = crate::Reg<u32, _HFPERPRESCB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFPERPRESCB;
#[doc = "`read()` method returns [hfperprescb::R](hfperprescb::R) reader structure"]
impl crate::Readable for HFPERPRESCB {}
#[doc = "`write(|w| ..)` method takes [hfperprescb::W](hfperprescb::W) writer structure"]
impl crate::Writable for HFPERPRESCB {}
#[doc = "High Frequency Peripheral Clock Prescaler B Register"]
pub mod hfperprescb;
#[doc = "High Frequency Peripheral Clock Prescaler C Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfperprescc](hfperprescc) module"]
pub type HFPERPRESCC = crate::Reg<u32, _HFPERPRESCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFPERPRESCC;
#[doc = "`read()` method returns [hfperprescc::R](hfperprescc::R) reader structure"]
impl crate::Readable for HFPERPRESCC {}
#[doc = "`write(|w| ..)` method takes [hfperprescc::W](hfperprescc::W) writer structure"]
impl crate::Writable for HFPERPRESCC {}
#[doc = "High Frequency Peripheral Clock Prescaler C Register"]
pub mod hfperprescc;
#[doc = "Low Frequency a Prescaler Register 0 (Async Reg)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfapresc0](lfapresc0) module"]
pub type LFAPRESC0 = crate::Reg<u32, _LFAPRESC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFAPRESC0;
#[doc = "`read()` method returns [lfapresc0::R](lfapresc0::R) reader structure"]
impl crate::Readable for LFAPRESC0 {}
#[doc = "`write(|w| ..)` method takes [lfapresc0::W](lfapresc0::W) writer structure"]
impl crate::Writable for LFAPRESC0 {}
#[doc = "Low Frequency a Prescaler Register 0 (Async Reg)"]
pub mod lfapresc0;
#[doc = "Low Frequency B Prescaler Register 0 (Async Reg)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfbpresc0](lfbpresc0) module"]
pub type LFBPRESC0 = crate::Reg<u32, _LFBPRESC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFBPRESC0;
#[doc = "`read()` method returns [lfbpresc0::R](lfbpresc0::R) reader structure"]
impl crate::Readable for LFBPRESC0 {}
#[doc = "`write(|w| ..)` method takes [lfbpresc0::W](lfbpresc0::W) writer structure"]
impl crate::Writable for LFBPRESC0 {}
#[doc = "Low Frequency B Prescaler Register 0 (Async Reg)"]
pub mod lfbpresc0;
#[doc = "Low Frequency E Prescaler Register 0 (Async Reg)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfepresc0](lfepresc0) module"]
pub type LFEPRESC0 = crate::Reg<u32, _LFEPRESC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFEPRESC0;
#[doc = "`read()` method returns [lfepresc0::R](lfepresc0::R) reader structure"]
impl crate::Readable for LFEPRESC0 {}
#[doc = "`write(|w| ..)` method takes [lfepresc0::W](lfepresc0::W) writer structure"]
impl crate::Writable for LFEPRESC0 {}
#[doc = "Low Frequency E Prescaler Register 0 (Async Reg)"]
pub mod lfepresc0;
#[doc = "Synchronization Busy Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](syncbusy) module"]
pub type SYNCBUSY = crate::Reg<u32, _SYNCBUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNCBUSY;
#[doc = "`read()` method returns [syncbusy::R](syncbusy::R) reader structure"]
impl crate::Readable for SYNCBUSY {}
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "Freeze Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freeze](freeze) module"]
pub type FREEZE = crate::Reg<u32, _FREEZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREEZE;
#[doc = "`read()` method returns [freeze::R](freeze::R) reader structure"]
impl crate::Readable for FREEZE {}
#[doc = "`write(|w| ..)` method takes [freeze::W](freeze::W) writer structure"]
impl crate::Writable for FREEZE {}
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "PCNT Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcntctrl](pcntctrl) module"]
pub type PCNTCTRL = crate::Reg<u32, _PCNTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCNTCTRL;
#[doc = "`read()` method returns [pcntctrl::R](pcntctrl::R) reader structure"]
impl crate::Readable for PCNTCTRL {}
#[doc = "`write(|w| ..)` method takes [pcntctrl::W](pcntctrl::W) writer structure"]
impl crate::Writable for PCNTCTRL {}
#[doc = "PCNT Control Register"]
pub mod pcntctrl;
#[doc = "ADC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcctrl](adcctrl) module"]
pub type ADCCTRL = crate::Reg<u32, _ADCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCCTRL;
#[doc = "`read()` method returns [adcctrl::R](adcctrl::R) reader structure"]
impl crate::Readable for ADCCTRL {}
#[doc = "`write(|w| ..)` method takes [adcctrl::W](adcctrl::W) writer structure"]
impl crate::Writable for ADCCTRL {}
#[doc = "ADC Control Register"]
pub mod adcctrl;
#[doc = "SDIO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdioctrl](sdioctrl) module"]
pub type SDIOCTRL = crate::Reg<u32, _SDIOCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDIOCTRL;
#[doc = "`read()` method returns [sdioctrl::R](sdioctrl::R) reader structure"]
impl crate::Readable for SDIOCTRL {}
#[doc = "`write(|w| ..)` method takes [sdioctrl::W](sdioctrl::W) writer structure"]
impl crate::Writable for SDIOCTRL {}
#[doc = "SDIO Control Register"]
pub mod sdioctrl;
#[doc = "QSPI Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspictrl](qspictrl) module"]
pub type QSPICTRL = crate::Reg<u32, _QSPICTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QSPICTRL;
#[doc = "`read()` method returns [qspictrl::R](qspictrl::R) reader structure"]
impl crate::Readable for QSPICTRL {}
#[doc = "`write(|w| ..)` method takes [qspictrl::W](qspictrl::W) writer structure"]
impl crate::Writable for QSPICTRL {}
#[doc = "QSPI Control Register"]
pub mod qspictrl;
#[doc = "I/O Routing Pin Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routepen](routepen) module"]
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
#[doc = "I/O Routing Location Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routeloc0](routeloc0) module"]
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
#[doc = "I/O Routing Location Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routeloc1](routeloc1) module"]
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
#[doc = "Configuration Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lock](lock) module"]
pub type LOCK = crate::Reg<u32, _LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCK;
#[doc = "`read()` method returns [lock::R](lock::R) reader structure"]
impl crate::Readable for LOCK {}
#[doc = "`write(|w| ..)` method takes [lock::W](lock::W) writer structure"]
impl crate::Writable for LOCK {}
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "HFRCO Spread Spectrum Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfrcoss](hfrcoss) module"]
pub type HFRCOSS = crate::Reg<u32, _HFRCOSS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFRCOSS;
#[doc = "`read()` method returns [hfrcoss::R](hfrcoss::R) reader structure"]
impl crate::Readable for HFRCOSS {}
#[doc = "`write(|w| ..)` method takes [hfrcoss::W](hfrcoss::W) writer structure"]
impl crate::Writable for HFRCOSS {}
#[doc = "HFRCO Spread Spectrum Register"]
pub mod hfrcoss;
#[doc = "USB Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbctrl](usbctrl) module"]
pub type USBCTRL = crate::Reg<u32, _USBCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBCTRL;
#[doc = "`read()` method returns [usbctrl::R](usbctrl::R) reader structure"]
impl crate::Readable for USBCTRL {}
#[doc = "`write(|w| ..)` method takes [usbctrl::W](usbctrl::W) writer structure"]
impl crate::Writable for USBCTRL {}
#[doc = "USB Control Register"]
pub mod usbctrl;
#[doc = "USB Clock Recovery Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbcrctrl](usbcrctrl) module"]
pub type USBCRCTRL = crate::Reg<u32, _USBCRCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBCRCTRL;
#[doc = "`read()` method returns [usbcrctrl::R](usbcrctrl::R) reader structure"]
impl crate::Readable for USBCRCTRL {}
#[doc = "`write(|w| ..)` method takes [usbcrctrl::W](usbcrctrl::W) writer structure"]
impl crate::Writable for USBCRCTRL {}
#[doc = "USB Clock Recovery Control"]
pub mod usbcrctrl;
