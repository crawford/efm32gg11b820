#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMU Control Register"]
    pub ctrl: CTRL,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - USHFRCO Control Register"]
    pub ushfrcoctrl: USHFRCOCTRL,
    _reserved1: [u8; 4usize],
    #[doc = "0x10 - HFRCO Control Register"]
    pub hfrcoctrl: HFRCOCTRL,
    _reserved2: [u8; 4usize],
    #[doc = "0x18 - AUXHFRCO Control Register"]
    pub auxhfrcoctrl: AUXHFRCOCTRL,
    _reserved3: [u8; 4usize],
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
    _reserved4: [u8; 4usize],
    #[doc = "0x40 - DPLL Control Register"]
    pub dpllctrl: DPLLCTRL,
    #[doc = "0x44 - DPLL Control Register"]
    pub dpllctrl1: DPLLCTRL1,
    _reserved5: [u8; 8usize],
    #[doc = "0x50 - Calibration Control Register"]
    pub calctrl: CALCTRL,
    #[doc = "0x54 - Calibration Counter Register"]
    pub calcnt: CALCNT,
    _reserved6: [u8; 8usize],
    #[doc = "0x60 - Oscillator Enable/Disable Command Register"]
    pub oscencmd: OSCENCMD,
    #[doc = "0x64 - Command Register"]
    pub cmd: CMD,
    _reserved7: [u8; 8usize],
    #[doc = "0x70 - Debug Trace Clock Select"]
    pub dbgclksel: DBGCLKSEL,
    #[doc = "0x74 - High Frequency Clock Select Command Register"]
    pub hfclksel: HFCLKSEL,
    _reserved8: [u8; 8usize],
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
    _reserved9: [u8; 4usize],
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
    _reserved10: [u8; 12usize],
    #[doc = "0xc0 - High Frequency Peripheral Clock Enable Register 0"]
    pub hfperclken0: HFPERCLKEN0,
    #[doc = "0xc4 - High Frequency Peripheral Clock Enable Register 1"]
    pub hfperclken1: HFPERCLKEN1,
    _reserved11: [u8; 24usize],
    #[doc = "0xe0 - Low Frequency a Clock Enable Register 0 (Async Reg)"]
    pub lfaclken0: LFACLKEN0,
    _reserved12: [u8; 4usize],
    #[doc = "0xe8 - Low Frequency B Clock Enable Register 0 (Async Reg)"]
    pub lfbclken0: LFBCLKEN0,
    #[doc = "0xec - Low Frequency C Clock Enable Register 0 (Async Reg)"]
    pub lfcclken0: LFCCLKEN0,
    #[doc = "0xf0 - Low Frequency E Clock Enable Register 0 (Async Reg)"]
    pub lfeclken0: LFECLKEN0,
    _reserved13: [u8; 12usize],
    #[doc = "0x100 - High Frequency Clock Prescaler Register"]
    pub hfpresc: HFPRESC,
    #[doc = "0x104 - High Frequency Bus Clock Prescaler Register"]
    pub hfbuspresc: HFBUSPRESC,
    #[doc = "0x108 - High Frequency Core Clock Prescaler Register"]
    pub hfcorepresc: HFCOREPRESC,
    #[doc = "0x10c - High Frequency Peripheral Clock Prescaler Register"]
    pub hfperpresc: HFPERPRESC,
    _reserved14: [u8; 4usize],
    #[doc = "0x114 - High Frequency Export Clock Prescaler Register"]
    pub hfexppresc: HFEXPPRESC,
    #[doc = "0x118 - High Frequency Peripheral Clock Prescaler B Register"]
    pub hfperprescb: HFPERPRESCB,
    #[doc = "0x11c - High Frequency Peripheral Clock Prescaler C Register"]
    pub hfperprescc: HFPERPRESCC,
    #[doc = "0x120 - Low Frequency a Prescaler Register 0 (Async Reg)"]
    pub lfapresc0: LFAPRESC0,
    _reserved15: [u8; 4usize],
    #[doc = "0x128 - Low Frequency B Prescaler Register 0 (Async Reg)"]
    pub lfbpresc0: LFBPRESC0,
    _reserved16: [u8; 4usize],
    #[doc = "0x130 - Low Frequency E Prescaler Register 0 (Async Reg)"]
    pub lfepresc0: LFEPRESC0,
    _reserved17: [u8; 12usize],
    #[doc = "0x140 - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x144 - Freeze Register"]
    pub freeze: FREEZE,
    _reserved18: [u8; 8usize],
    #[doc = "0x150 - PCNT Control Register"]
    pub pcntctrl: PCNTCTRL,
    _reserved19: [u8; 8usize],
    #[doc = "0x15c - ADC Control Register"]
    pub adcctrl: ADCCTRL,
    #[doc = "0x160 - SDIO Control Register"]
    pub sdioctrl: SDIOCTRL,
    #[doc = "0x164 - QSPI Control Register"]
    pub qspictrl: QSPICTRL,
    _reserved20: [u8; 8usize],
    #[doc = "0x170 - I/O Routing Pin Enable Register"]
    pub routepen: ROUTEPEN,
    #[doc = "0x174 - I/O Routing Location Register"]
    pub routeloc0: ROUTELOC0,
    #[doc = "0x178 - I/O Routing Location Register"]
    pub routeloc1: ROUTELOC1,
    _reserved21: [u8; 4usize],
    #[doc = "0x180 - Configuration Lock Register"]
    pub lock: LOCK,
    #[doc = "0x184 - HFRCO Spread Spectrum Register"]
    pub hfrcoss: HFRCOSS,
    _reserved22: [u8; 104usize],
    #[doc = "0x1f0 - USB Control Register"]
    pub usbctrl: USBCTRL,
    #[doc = "0x1f4 - USB Clock Recovery Control"]
    pub usbcrctrl: USBCRCTRL,
}
#[doc = "CMU Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CMU Control Register"]
pub mod ctrl;
#[doc = "USHFRCO Control Register"]
pub struct USHFRCOCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USHFRCO Control Register"]
pub mod ushfrcoctrl;
#[doc = "HFRCO Control Register"]
pub struct HFRCOCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HFRCO Control Register"]
pub mod hfrcoctrl;
#[doc = "AUXHFRCO Control Register"]
pub struct AUXHFRCOCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AUXHFRCO Control Register"]
pub mod auxhfrcoctrl;
#[doc = "LFRCO Control Register"]
pub struct LFRCOCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LFRCO Control Register"]
pub mod lfrcoctrl;
#[doc = "HFXO Control Register"]
pub struct HFXOCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HFXO Control Register"]
pub mod hfxoctrl;
#[doc = "HFXO Control 1"]
pub struct HFXOCTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HFXO Control 1"]
pub mod hfxoctrl1;
#[doc = "HFXO Startup Control"]
pub struct HFXOSTARTUPCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HFXO Startup Control"]
pub mod hfxostartupctrl;
#[doc = "HFXO Steady State Control"]
pub struct HFXOSTEADYSTATECTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HFXO Steady State Control"]
pub mod hfxosteadystatectrl;
#[doc = "HFXO Timeout Control"]
pub struct HFXOTIMEOUTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HFXO Timeout Control"]
pub mod hfxotimeoutctrl;
#[doc = "LFXO Control Register"]
pub struct LFXOCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LFXO Control Register"]
pub mod lfxoctrl;
#[doc = "DPLL Control Register"]
pub struct DPLLCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DPLL Control Register"]
pub mod dpllctrl;
#[doc = "DPLL Control Register"]
pub struct DPLLCTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DPLL Control Register"]
pub mod dpllctrl1;
#[doc = "Calibration Control Register"]
pub struct CALCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration Control Register"]
pub mod calctrl;
#[doc = "Calibration Counter Register"]
pub struct CALCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration Counter Register"]
pub mod calcnt;
#[doc = "Oscillator Enable/Disable Command Register"]
pub struct OSCENCMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Oscillator Enable/Disable Command Register"]
pub mod oscencmd;
#[doc = "Command Register"]
pub struct CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "Debug Trace Clock Select"]
pub struct DBGCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug Trace Clock Select"]
pub mod dbgclksel;
#[doc = "High Frequency Clock Select Command Register"]
pub struct HFCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "High Frequency Clock Select Command Register"]
pub mod hfclksel;
#[doc = "Low Frequency A Clock Select Register"]
pub struct LFACLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low Frequency A Clock Select Register"]
pub mod lfaclksel;
#[doc = "Low Frequency B Clock Select Register"]
pub struct LFBCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low Frequency B Clock Select Register"]
pub mod lfbclksel;
#[doc = "Low Frequency E Clock Select Register"]
pub struct LFECLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low Frequency E Clock Select Register"]
pub mod lfeclksel;
#[doc = "Low Frequency C Clock Select Register"]
pub struct LFCCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low Frequency C Clock Select Register"]
pub mod lfcclksel;
#[doc = "Status Register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod status;
#[doc = "HFCLK Status Register"]
pub struct HFCLKSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HFCLK Status Register"]
pub mod hfclkstatus;
#[doc = "HFXO Trim Status"]
pub struct HFXOTRIMSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HFXO Trim Status"]
pub mod hfxotrimstatus;
#[doc = "Interrupt Flag Register"]
pub struct IF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "Interrupt Flag Set Register"]
pub struct IFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "Interrupt Flag Clear Register"]
pub struct IFC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "Interrupt Enable Register"]
pub struct IEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "High Frequency Bus Clock Enable Register 0"]
pub struct HFBUSCLKEN0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "High Frequency Bus Clock Enable Register 0"]
pub mod hfbusclken0;
#[doc = "High Frequency Peripheral Clock Enable Register 0"]
pub struct HFPERCLKEN0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "High Frequency Peripheral Clock Enable Register 0"]
pub mod hfperclken0;
#[doc = "High Frequency Peripheral Clock Enable Register 1"]
pub struct HFPERCLKEN1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "High Frequency Peripheral Clock Enable Register 1"]
pub mod hfperclken1;
#[doc = "Low Frequency a Clock Enable Register 0 (Async Reg)"]
pub struct LFACLKEN0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low Frequency a Clock Enable Register 0 (Async Reg)"]
pub mod lfaclken0;
#[doc = "Low Frequency B Clock Enable Register 0 (Async Reg)"]
pub struct LFBCLKEN0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low Frequency B Clock Enable Register 0 (Async Reg)"]
pub mod lfbclken0;
#[doc = "Low Frequency C Clock Enable Register 0 (Async Reg)"]
pub struct LFCCLKEN0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low Frequency C Clock Enable Register 0 (Async Reg)"]
pub mod lfcclken0;
#[doc = "Low Frequency E Clock Enable Register 0 (Async Reg)"]
pub struct LFECLKEN0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low Frequency E Clock Enable Register 0 (Async Reg)"]
pub mod lfeclken0;
#[doc = "High Frequency Clock Prescaler Register"]
pub struct HFPRESC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "High Frequency Clock Prescaler Register"]
pub mod hfpresc;
#[doc = "High Frequency Bus Clock Prescaler Register"]
pub struct HFBUSPRESC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "High Frequency Bus Clock Prescaler Register"]
pub mod hfbuspresc;
#[doc = "High Frequency Core Clock Prescaler Register"]
pub struct HFCOREPRESC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "High Frequency Core Clock Prescaler Register"]
pub mod hfcorepresc;
#[doc = "High Frequency Peripheral Clock Prescaler Register"]
pub struct HFPERPRESC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "High Frequency Peripheral Clock Prescaler Register"]
pub mod hfperpresc;
#[doc = "High Frequency Export Clock Prescaler Register"]
pub struct HFEXPPRESC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "High Frequency Export Clock Prescaler Register"]
pub mod hfexppresc;
#[doc = "High Frequency Peripheral Clock Prescaler B Register"]
pub struct HFPERPRESCB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "High Frequency Peripheral Clock Prescaler B Register"]
pub mod hfperprescb;
#[doc = "High Frequency Peripheral Clock Prescaler C Register"]
pub struct HFPERPRESCC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "High Frequency Peripheral Clock Prescaler C Register"]
pub mod hfperprescc;
#[doc = "Low Frequency a Prescaler Register 0 (Async Reg)"]
pub struct LFAPRESC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low Frequency a Prescaler Register 0 (Async Reg)"]
pub mod lfapresc0;
#[doc = "Low Frequency B Prescaler Register 0 (Async Reg)"]
pub struct LFBPRESC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low Frequency B Prescaler Register 0 (Async Reg)"]
pub mod lfbpresc0;
#[doc = "Low Frequency E Prescaler Register 0 (Async Reg)"]
pub struct LFEPRESC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low Frequency E Prescaler Register 0 (Async Reg)"]
pub mod lfepresc0;
#[doc = "Synchronization Busy Register"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "Freeze Register"]
pub struct FREEZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "PCNT Control Register"]
pub struct PCNTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCNT Control Register"]
pub mod pcntctrl;
#[doc = "ADC Control Register"]
pub struct ADCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Control Register"]
pub mod adcctrl;
#[doc = "SDIO Control Register"]
pub struct SDIOCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDIO Control Register"]
pub mod sdioctrl;
#[doc = "QSPI Control Register"]
pub struct QSPICTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "QSPI Control Register"]
pub mod qspictrl;
#[doc = "I/O Routing Pin Enable Register"]
pub struct ROUTEPEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "I/O Routing Location Register"]
pub struct ROUTELOC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "I/O Routing Location Register"]
pub struct ROUTELOC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Routing Location Register"]
pub mod routeloc1;
#[doc = "Configuration Lock Register"]
pub struct LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "HFRCO Spread Spectrum Register"]
pub struct HFRCOSS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HFRCO Spread Spectrum Register"]
pub mod hfrcoss;
#[doc = "USB Control Register"]
pub struct USBCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Control Register"]
pub mod usbctrl;
#[doc = "USB Clock Recovery Control"]
pub struct USBCRCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Clock Recovery Control"]
pub mod usbcrctrl;
