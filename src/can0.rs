#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - Error Count Register"]
    pub errcnt: ERRCNT,
    #[doc = "0x0c - Bit Timing Register"]
    pub bittiming: BITTIMING,
    #[doc = "0x10 - Interrupt Identification Register"]
    pub intid: INTID,
    #[doc = "0x14 - Test Register"]
    pub test: TEST,
    #[doc = "0x18 - BRP Extension Register"]
    pub brpe: BRPE,
    #[doc = "0x1c - Transmission Request Register"]
    pub transreq: TRANSREQ,
    #[doc = "0x20 - New Data Register"]
    pub messagedata: MESSAGEDATA,
    _reserved0: [u8; 4usize],
    #[doc = "0x28 - Message Valid Register"]
    pub messagestate: MESSAGESTATE,
    #[doc = "0x2c - Configuration Register"]
    pub config: CONFIG,
    #[doc = "0x30 - Message Object Interrupt Flag Register"]
    pub if0if: IF0IF,
    #[doc = "0x34 - Message Object Interrupt Flag Set Register"]
    pub if0ifs: IF0IFS,
    #[doc = "0x38 - Message Object Interrupt Flag Clear Register"]
    pub if0ifc: IF0IFC,
    #[doc = "0x3c - Message Object Interrupt Enable Register"]
    pub if0ien: IF0IEN,
    #[doc = "0x40 - Status Interrupt Flag Register"]
    pub if1if: IF1IF,
    #[doc = "0x44 - Message Object Interrupt Flag Set Register"]
    pub if1ifs: IF1IFS,
    #[doc = "0x48 - Message Object Interrupt Flag Clear Register"]
    pub if1ifc: IF1IFC,
    #[doc = "0x4c - Status Interrupt Enable Register"]
    pub if1ien: IF1IEN,
    #[doc = "0x50 - I/O Routing Register"]
    pub route: ROUTE,
    _reserved1: [u8; 12usize],
    #[doc = "0x60 - Interface Command Mask Register"]
    pub mir0_cmdmask: MIR0_CMDMASK,
    #[doc = "0x64 - Interface Mask Register"]
    pub mir0_mask: MIR0_MASK,
    #[doc = "0x68 - Interface Arbitration Register"]
    pub mir0_arb: MIR0_ARB,
    #[doc = "0x6c - Interface Message Control Register"]
    pub mir0_ctrl: MIR0_CTRL,
    #[doc = "0x70 - Interface Data a Register"]
    pub mir0_datal: MIR0_DATAL,
    #[doc = "0x74 - Interface Data B Register"]
    pub mir0_datah: MIR0_DATAH,
    #[doc = "0x78 - Interface Command Request Register"]
    pub mir0_cmdreq: MIR0_CMDREQ,
    _reserved2: [u8; 4usize],
    #[doc = "0x80 - Interface Command Mask Register"]
    pub mir1_cmdmask: MIR1_CMDMASK,
    #[doc = "0x84 - Interface Mask Register"]
    pub mir1_mask: MIR1_MASK,
    #[doc = "0x88 - Interface Arbitration Register"]
    pub mir1_arb: MIR1_ARB,
    #[doc = "0x8c - Interface Message Control Register"]
    pub mir1_ctrl: MIR1_CTRL,
    #[doc = "0x90 - Interface Data a Register"]
    pub mir1_datal: MIR1_DATAL,
    #[doc = "0x94 - Interface Data B Register"]
    pub mir1_datah: MIR1_DATAH,
    #[doc = "0x98 - Interface Command Request Register"]
    pub mir1_cmdreq: MIR1_CMDREQ,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Status Register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod status;
#[doc = "Error Count Register"]
pub struct ERRCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Count Register"]
pub mod errcnt;
#[doc = "Bit Timing Register"]
pub struct BITTIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bit Timing Register"]
pub mod bittiming;
#[doc = "Interrupt Identification Register"]
pub struct INTID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Identification Register"]
pub mod intid;
#[doc = "Test Register"]
pub struct TEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Test Register"]
pub mod test;
#[doc = "BRP Extension Register"]
pub struct BRPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BRP Extension Register"]
pub mod brpe;
#[doc = "Transmission Request Register"]
pub struct TRANSREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmission Request Register"]
pub mod transreq;
#[doc = "New Data Register"]
pub struct MESSAGEDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "New Data Register"]
pub mod messagedata;
#[doc = "Message Valid Register"]
pub struct MESSAGESTATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Valid Register"]
pub mod messagestate;
#[doc = "Configuration Register"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Register"]
pub mod config;
#[doc = "Message Object Interrupt Flag Register"]
pub struct IF0IF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Object Interrupt Flag Register"]
pub mod if0if;
#[doc = "Message Object Interrupt Flag Set Register"]
pub struct IF0IFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Object Interrupt Flag Set Register"]
pub mod if0ifs;
#[doc = "Message Object Interrupt Flag Clear Register"]
pub struct IF0IFC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Object Interrupt Flag Clear Register"]
pub mod if0ifc;
#[doc = "Message Object Interrupt Enable Register"]
pub struct IF0IEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Object Interrupt Enable Register"]
pub mod if0ien;
#[doc = "Status Interrupt Flag Register"]
pub struct IF1IF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Interrupt Flag Register"]
pub mod if1if;
#[doc = "Message Object Interrupt Flag Set Register"]
pub struct IF1IFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Object Interrupt Flag Set Register"]
pub mod if1ifs;
#[doc = "Message Object Interrupt Flag Clear Register"]
pub struct IF1IFC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Object Interrupt Flag Clear Register"]
pub mod if1ifc;
#[doc = "Status Interrupt Enable Register"]
pub struct IF1IEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Interrupt Enable Register"]
pub mod if1ien;
#[doc = "I/O Routing Register"]
pub struct ROUTE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "Interface Command Mask Register"]
pub struct MIR0_CMDMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interface Command Mask Register"]
pub mod mir0_cmdmask;
#[doc = "Interface Mask Register"]
pub struct MIR0_MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interface Mask Register"]
pub mod mir0_mask;
#[doc = "Interface Arbitration Register"]
pub struct MIR0_ARB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interface Arbitration Register"]
pub mod mir0_arb;
#[doc = "Interface Message Control Register"]
pub struct MIR0_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interface Message Control Register"]
pub mod mir0_ctrl;
#[doc = "Interface Data a Register"]
pub struct MIR0_DATAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interface Data a Register"]
pub mod mir0_datal;
#[doc = "Interface Data B Register"]
pub struct MIR0_DATAH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interface Data B Register"]
pub mod mir0_datah;
#[doc = "Interface Command Request Register"]
pub struct MIR0_CMDREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interface Command Request Register"]
pub mod mir0_cmdreq;
#[doc = "Interface Command Mask Register"]
pub struct MIR1_CMDMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interface Command Mask Register"]
pub mod mir1_cmdmask;
#[doc = "Interface Mask Register"]
pub struct MIR1_MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interface Mask Register"]
pub mod mir1_mask;
#[doc = "Interface Arbitration Register"]
pub struct MIR1_ARB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interface Arbitration Register"]
pub mod mir1_arb;
#[doc = "Interface Message Control Register"]
pub struct MIR1_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interface Message Control Register"]
pub mod mir1_ctrl;
#[doc = "Interface Data a Register"]
pub struct MIR1_DATAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interface Data a Register"]
pub mod mir1_datal;
#[doc = "Interface Data B Register"]
pub struct MIR1_DATAH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interface Data B Register"]
pub mod mir1_datah;
#[doc = "Interface Command Request Register"]
pub struct MIR1_CMDREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interface Command Request Register"]
pub mod mir1_cmdreq;
