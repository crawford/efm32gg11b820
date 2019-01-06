#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Display Control Register"]
    pub dispctrl: DISPCTRL,
    #[doc = "0x08 - Segment Enable Register"]
    pub segen: SEGEN,
    #[doc = "0x0c - Blink and Animation Control Register"]
    pub bactrl: BACTRL,
    #[doc = "0x10 - Status Register"]
    pub status: STATUS,
    #[doc = "0x14 - Animation Register a"]
    pub arega: AREGA,
    #[doc = "0x18 - Animation Register B"]
    pub aregb: AREGB,
    #[doc = "0x1c - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x20 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x24 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x28 - Interrupt Enable Register"]
    pub ien: IEN,
    _reserved0: [u8; 4usize],
    #[doc = "0x30 - Analog BIAS Control"]
    pub biasctrl: BIASCTRL,
    _reserved1: [u8; 12usize],
    #[doc = "0x40 - Segment Data Low Register 0"]
    pub segd0l: SEGD0L,
    #[doc = "0x44 - Segment Data Low Register 1"]
    pub segd1l: SEGD1L,
    #[doc = "0x48 - Segment Data Low Register 2"]
    pub segd2l: SEGD2L,
    #[doc = "0x4c - Segment Data Low Register 3"]
    pub segd3l: SEGD3L,
    #[doc = "0x50 - Segment Data High Register 0"]
    pub segd0h: SEGD0H,
    #[doc = "0x54 - Segment Data High Register 1"]
    pub segd1h: SEGD1H,
    #[doc = "0x58 - Segment Data High Register 2"]
    pub segd2h: SEGD2H,
    #[doc = "0x5c - Segment Data High Register 3"]
    pub segd3h: SEGD3H,
    #[doc = "0x60 - Segment Data Low Register 4"]
    pub segd4l: SEGD4L,
    #[doc = "0x64 - Segment Data Low Register 5"]
    pub segd5l: SEGD5L,
    #[doc = "0x68 - Segment Data Low Register 6"]
    pub segd6l: SEGD6L,
    #[doc = "0x6c - Segment Data Low Register 7"]
    pub segd7l: SEGD7L,
    #[doc = "0x70 - Segment Data High Register 4"]
    pub segd4h: SEGD4H,
    #[doc = "0x74 - Segment Data High Register 5"]
    pub segd5h: SEGD5H,
    #[doc = "0x78 - Segment Data High Register 6"]
    pub segd6h: SEGD6H,
    #[doc = "0x7c - Segment Data High Register 7"]
    pub segd7h: SEGD7H,
    _reserved2: [u8; 64usize],
    #[doc = "0xc0 - Freeze Register"]
    pub freeze: FREEZE,
    #[doc = "0xc4 - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    _reserved3: [u8; 40usize],
    #[doc = "0xf0 - Frame Rate"]
    pub framerate: FRAMERATE,
    #[doc = "0xf4 - Segment Enable (32 to 39)"]
    pub segen2: SEGEN2,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Display Control Register"]
pub struct DISPCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Display Control Register"]
pub mod dispctrl;
#[doc = "Segment Enable Register"]
pub struct SEGEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Segment Enable Register"]
pub mod segen;
#[doc = "Blink and Animation Control Register"]
pub struct BACTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Blink and Animation Control Register"]
pub mod bactrl;
#[doc = "Status Register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod status;
#[doc = "Animation Register a"]
pub struct AREGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Animation Register a"]
pub mod arega;
#[doc = "Animation Register B"]
pub struct AREGB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Animation Register B"]
pub mod aregb;
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
#[doc = "Analog BIAS Control"]
pub struct BIASCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog BIAS Control"]
pub mod biasctrl;
#[doc = "Segment Data Low Register 0"]
pub struct SEGD0L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Segment Data Low Register 0"]
pub mod segd0l;
#[doc = "Segment Data Low Register 1"]
pub struct SEGD1L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Segment Data Low Register 1"]
pub mod segd1l;
#[doc = "Segment Data Low Register 2"]
pub struct SEGD2L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Segment Data Low Register 2"]
pub mod segd2l;
#[doc = "Segment Data Low Register 3"]
pub struct SEGD3L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Segment Data Low Register 3"]
pub mod segd3l;
#[doc = "Segment Data High Register 0"]
pub struct SEGD0H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Segment Data High Register 0"]
pub mod segd0h;
#[doc = "Segment Data High Register 1"]
pub struct SEGD1H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Segment Data High Register 1"]
pub mod segd1h;
#[doc = "Segment Data High Register 2"]
pub struct SEGD2H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Segment Data High Register 2"]
pub mod segd2h;
#[doc = "Segment Data High Register 3"]
pub struct SEGD3H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Segment Data High Register 3"]
pub mod segd3h;
#[doc = "Segment Data Low Register 4"]
pub struct SEGD4L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Segment Data Low Register 4"]
pub mod segd4l;
#[doc = "Segment Data Low Register 5"]
pub struct SEGD5L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Segment Data Low Register 5"]
pub mod segd5l;
#[doc = "Segment Data Low Register 6"]
pub struct SEGD6L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Segment Data Low Register 6"]
pub mod segd6l;
#[doc = "Segment Data Low Register 7"]
pub struct SEGD7L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Segment Data Low Register 7"]
pub mod segd7l;
#[doc = "Segment Data High Register 4"]
pub struct SEGD4H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Segment Data High Register 4"]
pub mod segd4h;
#[doc = "Segment Data High Register 5"]
pub struct SEGD5H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Segment Data High Register 5"]
pub mod segd5h;
#[doc = "Segment Data High Register 6"]
pub struct SEGD6H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Segment Data High Register 6"]
pub mod segd6h;
#[doc = "Segment Data High Register 7"]
pub struct SEGD7H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Segment Data High Register 7"]
pub mod segd7h;
#[doc = "Freeze Register"]
pub struct FREEZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "Synchronization Busy Register"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "Frame Rate"]
pub struct FRAMERATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frame Rate"]
pub mod framerate;
#[doc = "Segment Enable (32 to 39)"]
pub struct SEGEN2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Segment Enable (32 to 39)"]
pub mod segen2;
