#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Counter Value Register"]
    pub cnt: CNT,
    #[doc = "0x08 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x0c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x10 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub ien: IEN,
    _reserved0: [u8; 8usize],
    #[doc = "0x20 - Compare Value Register X"]
    pub compa_comp: COMPA_COMP,
    #[doc = "0x24 - Compare Value Register X"]
    pub compb_comp: COMPB_COMP,
    #[doc = "0x28 - Compare Value Register X"]
    pub compc_comp: COMPC_COMP,
    #[doc = "0x2c - Compare Value Register X"]
    pub compd_comp: COMPD_COMP,
    #[doc = "0x30 - Compare Value Register X"]
    pub compe_comp: COMPE_COMP,
    #[doc = "0x34 - Compare Value Register X"]
    pub compf_comp: COMPF_COMP,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Counter Value Register"]
pub struct CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Value Register"]
pub mod cnt;
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
#[doc = "Compare Value Register X"]
pub struct COMPA_COMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Value Register X"]
pub mod compa_comp;
#[doc = "Compare Value Register X"]
pub struct COMPB_COMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Value Register X"]
pub mod compb_comp;
#[doc = "Compare Value Register X"]
pub struct COMPC_COMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Value Register X"]
pub mod compc_comp;
#[doc = "Compare Value Register X"]
pub struct COMPD_COMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Value Register X"]
pub mod compd_comp;
#[doc = "Compare Value Register X"]
pub struct COMPE_COMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Value Register X"]
pub mod compe_comp;
#[doc = "Compare Value Register X"]
pub struct COMPF_COMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Value Register X"]
pub mod compf_comp;
