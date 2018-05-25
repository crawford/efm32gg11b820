#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Timing Control"]
    pub timctrl: TIMCTRL,
    #[doc = "0x08 - Command"]
    pub cmd: CMD,
    #[doc = "0x0c - Status"]
    pub status: STATUS,
    #[doc = "0x10 - PRS Select"]
    pub prssel: PRSSEL,
    #[doc = "0x14 - Output Data"]
    pub data: DATA,
    #[doc = "0x18 - Scan Channel Mask 0"]
    pub scanmask0: SCANMASK0,
    #[doc = "0x1c - Scan Input Selection 0"]
    pub scaninputsel0: SCANINPUTSEL0,
    #[doc = "0x20 - Scan Channel Mask 1"]
    pub scanmask1: SCANMASK1,
    #[doc = "0x24 - Scan Input Selection 1"]
    pub scaninputsel1: SCANINPUTSEL1,
    #[doc = "0x28 - APORT Request Status"]
    pub aportreq: APORTREQ,
    #[doc = "0x2c - APORT Request Conflict"]
    pub aportconflict: APORTCONFLICT,
    #[doc = "0x30 - Comparator Threshold"]
    pub cmpthr: CMPTHR,
    #[doc = "0x34 - Exponential Moving Average"]
    pub ema: EMA,
    #[doc = "0x38 - Exponential Moving Average Control"]
    pub emactrl: EMACTRL,
    #[doc = "0x3c - Single Conversion Control"]
    pub singlectrl: SINGLECTRL,
    #[doc = "0x40 - Delta Modulation Baseline"]
    pub dmbaseline: DMBASELINE,
    #[doc = "0x44 - Delta Modulation Configuration"]
    pub dmcfg: DMCFG,
    #[doc = "0x48 - Analog Control"]
    pub anactrl: ANACTRL,
    _reserved0: [u8; 8usize],
    #[doc = "0x54 - Interrupt Flag"]
    pub if_: IF,
    #[doc = "0x58 - Interrupt Flag Set"]
    pub ifs: IFS,
    #[doc = "0x5c - Interrupt Flag Clear"]
    pub ifc: IFC,
    #[doc = "0x60 - Interrupt Enable"]
    pub ien: IEN,
}
#[doc = "Control"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control"]
pub mod ctrl;
#[doc = "Timing Control"]
pub struct TIMCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timing Control"]
pub mod timctrl;
#[doc = "Command"]
pub struct CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command"]
pub mod cmd;
#[doc = "Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status"]
pub mod status;
#[doc = "PRS Select"]
pub struct PRSSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PRS Select"]
pub mod prssel;
#[doc = "Output Data"]
pub struct DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Data"]
pub mod data;
#[doc = "Scan Channel Mask 0"]
pub struct SCANMASK0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Channel Mask 0"]
pub mod scanmask0;
#[doc = "Scan Input Selection 0"]
pub struct SCANINPUTSEL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Input Selection 0"]
pub mod scaninputsel0;
#[doc = "Scan Channel Mask 1"]
pub struct SCANMASK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Channel Mask 1"]
pub mod scanmask1;
#[doc = "Scan Input Selection 1"]
pub struct SCANINPUTSEL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Input Selection 1"]
pub mod scaninputsel1;
#[doc = "APORT Request Status"]
pub struct APORTREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APORT Request Status"]
pub mod aportreq;
#[doc = "APORT Request Conflict"]
pub struct APORTCONFLICT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APORT Request Conflict"]
pub mod aportconflict;
#[doc = "Comparator Threshold"]
pub struct CMPTHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Comparator Threshold"]
pub mod cmpthr;
#[doc = "Exponential Moving Average"]
pub struct EMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Exponential Moving Average"]
pub mod ema;
#[doc = "Exponential Moving Average Control"]
pub struct EMACTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Exponential Moving Average Control"]
pub mod emactrl;
#[doc = "Single Conversion Control"]
pub struct SINGLECTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Single Conversion Control"]
pub mod singlectrl;
#[doc = "Delta Modulation Baseline"]
pub struct DMBASELINE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Delta Modulation Baseline"]
pub mod dmbaseline;
#[doc = "Delta Modulation Configuration"]
pub struct DMCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Delta Modulation Configuration"]
pub mod dmcfg;
#[doc = "Analog Control"]
pub struct ANACTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Control"]
pub mod anactrl;
#[doc = "Interrupt Flag"]
pub struct IF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag"]
pub mod if_;
#[doc = "Interrupt Flag Set"]
pub struct IFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag Set"]
pub mod ifs;
#[doc = "Interrupt Flag Clear"]
pub struct IFC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag Clear"]
pub mod ifc;
#[doc = "Interrupt Enable"]
pub struct IEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable"]
pub mod ien;
