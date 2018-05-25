#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 12usize],
    #[doc = "0x0c - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x10 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x14 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x18 - Interrupt Enable Register"]
    pub ien: IEN,
    _reserved1: [u8; 36usize],
    #[doc = "0x40 - PPU Control Register"]
    pub ppuctrl: PPUCTRL,
    _reserved2: [u8; 12usize],
    #[doc = "0x50 - PPU Privilege Access Type Descriptor 0"]
    pub ppupatd0: PPUPATD0,
    #[doc = "0x54 - PPU Privilege Access Type Descriptor 1"]
    pub ppupatd1: PPUPATD1,
    #[doc = "0x58 - PPU Privilege Access Type Descriptor 2"]
    pub ppupatd2: PPUPATD2,
    _reserved3: [u8; 52usize],
    #[doc = "0x90 - PPU Fault Status"]
    pub ppufs: PPUFS,
}
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
#[doc = "PPU Control Register"]
pub struct PPUCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PPU Control Register"]
pub mod ppuctrl;
#[doc = "PPU Privilege Access Type Descriptor 0"]
pub struct PPUPATD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PPU Privilege Access Type Descriptor 0"]
pub mod ppupatd0;
#[doc = "PPU Privilege Access Type Descriptor 1"]
pub struct PPUPATD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PPU Privilege Access Type Descriptor 1"]
pub mod ppupatd1;
#[doc = "PPU Privilege Access Type Descriptor 2"]
pub struct PPUPATD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PPU Privilege Access Type Descriptor 2"]
pub mod ppupatd2;
#[doc = "PPU Fault Status"]
pub struct PPUFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PPU Fault Status"]
pub mod ppufs;
