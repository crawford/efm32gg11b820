#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Octal-SPI Configuration Register"]
    pub config: CONFIG,
    #[doc = "0x04 - Device Read Instruction Configuration Register"]
    pub devinstrrdconfig: DEVINSTRRDCONFIG,
    #[doc = "0x08 - Device Write Instruction Configuration Register"]
    pub devinstrwrconfig: DEVINSTRWRCONFIG,
    #[doc = "0x0c - Device Delay Register"]
    pub devdelay: DEVDELAY,
    #[doc = "0x10 - Read Data Capture Register"]
    pub rddatacapture: RDDATACAPTURE,
    #[doc = "0x14 - Device Size Configuration Register"]
    pub devsizeconfig: DEVSIZECONFIG,
    #[doc = "0x18 - SRAM Partition Configuration Register"]
    pub srampartitioncfg: SRAMPARTITIONCFG,
    #[doc = "0x1c - Indirect Address Trigger Register"]
    pub indahbaddrtrigger: INDAHBADDRTRIGGER,
    _reserved0: [u8; 4usize],
    #[doc = "0x24 - Remap Address Register"]
    pub remapaddr: REMAPADDR,
    #[doc = "0x28 - Mode Bit Configuration Register"]
    pub modebitconfig: MODEBITCONFIG,
    #[doc = "0x2c - SRAM Fill Register"]
    pub sramfill: SRAMFILL,
    #[doc = "0x30 - TX Threshold Register"]
    pub txthresh: TXTHRESH,
    #[doc = "0x34 - RX Threshold Register"]
    pub rxthresh: RXTHRESH,
    #[doc = "0x38 - Write Completion Control Register"]
    pub writecompletionctrl: WRITECOMPLETIONCTRL,
    #[doc = "0x3c - Polling Expiration Register"]
    pub noofpollsbefexp: NOOFPOLLSBEFEXP,
    #[doc = "0x40 - Interrupt Status Register"]
    pub irqstatus: IRQSTATUS,
    #[doc = "0x44 - Interrupt Mask"]
    pub irqmask: IRQMASK,
    _reserved1: [u8; 8usize],
    #[doc = "0x50 - Lower Write Protection Register"]
    pub lowerwrprot: LOWERWRPROT,
    #[doc = "0x54 - Upper Write Protection Register"]
    pub upperwrprot: UPPERWRPROT,
    #[doc = "0x58 - Write Protection Control Register"]
    pub wrprotctrl: WRPROTCTRL,
    _reserved2: [u8; 4usize],
    #[doc = "0x60 - Indirect Read Transfer Control Register"]
    pub indirectreadxferctrl: INDIRECTREADXFERCTRL,
    #[doc = "0x64 - Indirect Read Transfer Watermark Register"]
    pub indirectreadxferwatermark: INDIRECTREADXFERWATERMARK,
    #[doc = "0x68 - Indirect Read Transfer Start Address Register"]
    pub indirectreadxferstart: INDIRECTREADXFERSTART,
    #[doc = "0x6c - Indirect Read Transfer Number Bytes Register"]
    pub indirectreadxfernumbytes: INDIRECTREADXFERNUMBYTES,
    #[doc = "0x70 - Indirect Write Transfer Control Register"]
    pub indirectwritexferctrl: INDIRECTWRITEXFERCTRL,
    #[doc = "0x74 - Indirect Write Transfer Watermark Register"]
    pub indirectwritexferwatermark: INDIRECTWRITEXFERWATERMARK,
    #[doc = "0x78 - Indirect Write Transfer Start Address Register"]
    pub indirectwritexferstart: INDIRECTWRITEXFERSTART,
    #[doc = "0x7c - Indirect Write Transfer Number Bytes Register"]
    pub indirectwritexfernumbytes: INDIRECTWRITEXFERNUMBYTES,
    #[doc = "0x80 - Indirect Trigger Address Range Register"]
    pub indirecttriggeraddrrange: INDIRECTTRIGGERADDRRANGE,
    _reserved3: [u8; 8usize],
    #[doc = "0x8c - Flash Command Control Memory Register"]
    pub flashcommandctrlmem: FLASHCOMMANDCTRLMEM,
    #[doc = "0x90 - Flash Command Control Register"]
    pub flashcmdctrl: FLASHCMDCTRL,
    #[doc = "0x94 - Flash Command Address Register"]
    pub flashcmdaddr: FLASHCMDADDR,
    _reserved4: [u8; 8usize],
    #[doc = "0xa0 - Flash Command Read Data Register (Lower)"]
    pub flashrddatalower: FLASHRDDATALOWER,
    #[doc = "0xa4 - Flash Command Read Data Register (Upper)"]
    pub flashrddataupper: FLASHRDDATAUPPER,
    #[doc = "0xa8 - Flash Command Write Data Register (Lower)"]
    pub flashwrdatalower: FLASHWRDATALOWER,
    #[doc = "0xac - Flash Command Write Data Register (Upper)"]
    pub flashwrdataupper: FLASHWRDATAUPPER,
    #[doc = "0xb0 - Polling Flash Status Register"]
    pub pollingflashstatus: POLLINGFLASHSTATUS,
    #[doc = "0xb4 - PHY Configuration Register"]
    pub phyconfiguration: PHYCONFIGURATION,
    #[doc = "0xb8 - PHY DLL Master Control Register"]
    pub phymastercontrol: PHYMASTERCONTROL,
    #[doc = "0xbc - DLL Observable Register Lower"]
    pub dllobservablelower: DLLOBSERVABLELOWER,
    #[doc = "0xc0 - DLL Observable Register Upper"]
    pub dllobservableupper: DLLOBSERVABLEUPPER,
    _reserved5: [u8; 28usize],
    #[doc = "0xe0 - Opcode Extension Register (Lower)"]
    pub opcodeextlower: OPCODEEXTLOWER,
    #[doc = "0xe4 - Opcode Extension Register (Upper)"]
    pub opcodeextupper: OPCODEEXTUPPER,
    _reserved6: [u8; 20usize],
    #[doc = "0xfc - Module ID Register"]
    pub moduleid: MODULEID,
    #[doc = "0x100 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x104 - I/O Routing Pin Enable Register"]
    pub routepen: ROUTEPEN,
    #[doc = "0x108 - I/O Route Location Register 0"]
    pub routeloc0: ROUTELOC0,
}
#[doc = "Octal-SPI Configuration Register"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Octal-SPI Configuration Register"]
pub mod config;
#[doc = "Device Read Instruction Configuration Register"]
pub struct DEVINSTRRDCONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Read Instruction Configuration Register"]
pub mod devinstrrdconfig;
#[doc = "Device Write Instruction Configuration Register"]
pub struct DEVINSTRWRCONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Write Instruction Configuration Register"]
pub mod devinstrwrconfig;
#[doc = "Device Delay Register"]
pub struct DEVDELAY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Delay Register"]
pub mod devdelay;
#[doc = "Read Data Capture Register"]
pub struct RDDATACAPTURE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read Data Capture Register"]
pub mod rddatacapture;
#[doc = "Device Size Configuration Register"]
pub struct DEVSIZECONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Size Configuration Register"]
pub mod devsizeconfig;
#[doc = "SRAM Partition Configuration Register"]
pub struct SRAMPARTITIONCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM Partition Configuration Register"]
pub mod srampartitioncfg;
#[doc = "Indirect Address Trigger Register"]
pub struct INDAHBADDRTRIGGER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Indirect Address Trigger Register"]
pub mod indahbaddrtrigger;
#[doc = "Remap Address Register"]
pub struct REMAPADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Remap Address Register"]
pub mod remapaddr;
#[doc = "Mode Bit Configuration Register"]
pub struct MODEBITCONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Bit Configuration Register"]
pub mod modebitconfig;
#[doc = "SRAM Fill Register"]
pub struct SRAMFILL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM Fill Register"]
pub mod sramfill;
#[doc = "TX Threshold Register"]
pub struct TXTHRESH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX Threshold Register"]
pub mod txthresh;
#[doc = "RX Threshold Register"]
pub struct RXTHRESH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX Threshold Register"]
pub mod rxthresh;
#[doc = "Write Completion Control Register"]
pub struct WRITECOMPLETIONCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Completion Control Register"]
pub mod writecompletionctrl;
#[doc = "Polling Expiration Register"]
pub struct NOOFPOLLSBEFEXP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Polling Expiration Register"]
pub mod noofpollsbefexp;
#[doc = "Interrupt Status Register"]
pub struct IRQSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod irqstatus;
#[doc = "Interrupt Mask"]
pub struct IRQMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask"]
pub mod irqmask;
#[doc = "Lower Write Protection Register"]
pub struct LOWERWRPROT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lower Write Protection Register"]
pub mod lowerwrprot;
#[doc = "Upper Write Protection Register"]
pub struct UPPERWRPROT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Upper Write Protection Register"]
pub mod upperwrprot;
#[doc = "Write Protection Control Register"]
pub struct WRPROTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Control Register"]
pub mod wrprotctrl;
#[doc = "Indirect Read Transfer Control Register"]
pub struct INDIRECTREADXFERCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Indirect Read Transfer Control Register"]
pub mod indirectreadxferctrl;
#[doc = "Indirect Read Transfer Watermark Register"]
pub struct INDIRECTREADXFERWATERMARK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Indirect Read Transfer Watermark Register"]
pub mod indirectreadxferwatermark;
#[doc = "Indirect Read Transfer Start Address Register"]
pub struct INDIRECTREADXFERSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Indirect Read Transfer Start Address Register"]
pub mod indirectreadxferstart;
#[doc = "Indirect Read Transfer Number Bytes Register"]
pub struct INDIRECTREADXFERNUMBYTES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Indirect Read Transfer Number Bytes Register"]
pub mod indirectreadxfernumbytes;
#[doc = "Indirect Write Transfer Control Register"]
pub struct INDIRECTWRITEXFERCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Indirect Write Transfer Control Register"]
pub mod indirectwritexferctrl;
#[doc = "Indirect Write Transfer Watermark Register"]
pub struct INDIRECTWRITEXFERWATERMARK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Indirect Write Transfer Watermark Register"]
pub mod indirectwritexferwatermark;
#[doc = "Indirect Write Transfer Start Address Register"]
pub struct INDIRECTWRITEXFERSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Indirect Write Transfer Start Address Register"]
pub mod indirectwritexferstart;
#[doc = "Indirect Write Transfer Number Bytes Register"]
pub struct INDIRECTWRITEXFERNUMBYTES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Indirect Write Transfer Number Bytes Register"]
pub mod indirectwritexfernumbytes;
#[doc = "Indirect Trigger Address Range Register"]
pub struct INDIRECTTRIGGERADDRRANGE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Indirect Trigger Address Range Register"]
pub mod indirecttriggeraddrrange;
#[doc = "Flash Command Control Memory Register"]
pub struct FLASHCOMMANDCTRLMEM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Command Control Memory Register"]
pub mod flashcommandctrlmem;
#[doc = "Flash Command Control Register"]
pub struct FLASHCMDCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Command Control Register"]
pub mod flashcmdctrl;
#[doc = "Flash Command Address Register"]
pub struct FLASHCMDADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Command Address Register"]
pub mod flashcmdaddr;
#[doc = "Flash Command Read Data Register (Lower)"]
pub struct FLASHRDDATALOWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Command Read Data Register (Lower)"]
pub mod flashrddatalower;
#[doc = "Flash Command Read Data Register (Upper)"]
pub struct FLASHRDDATAUPPER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Command Read Data Register (Upper)"]
pub mod flashrddataupper;
#[doc = "Flash Command Write Data Register (Lower)"]
pub struct FLASHWRDATALOWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Command Write Data Register (Lower)"]
pub mod flashwrdatalower;
#[doc = "Flash Command Write Data Register (Upper)"]
pub struct FLASHWRDATAUPPER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Command Write Data Register (Upper)"]
pub mod flashwrdataupper;
#[doc = "Polling Flash Status Register"]
pub struct POLLINGFLASHSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Polling Flash Status Register"]
pub mod pollingflashstatus;
#[doc = "PHY Configuration Register"]
pub struct PHYCONFIGURATION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PHY Configuration Register"]
pub mod phyconfiguration;
#[doc = "PHY DLL Master Control Register"]
pub struct PHYMASTERCONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PHY DLL Master Control Register"]
pub mod phymastercontrol;
#[doc = "DLL Observable Register Lower"]
pub struct DLLOBSERVABLELOWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DLL Observable Register Lower"]
pub mod dllobservablelower;
#[doc = "DLL Observable Register Upper"]
pub struct DLLOBSERVABLEUPPER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DLL Observable Register Upper"]
pub mod dllobservableupper;
#[doc = "Opcode Extension Register (Lower)"]
pub struct OPCODEEXTLOWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Opcode Extension Register (Lower)"]
pub mod opcodeextlower;
#[doc = "Opcode Extension Register (Upper)"]
pub struct OPCODEEXTUPPER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Opcode Extension Register (Upper)"]
pub mod opcodeextupper;
#[doc = "Module ID Register"]
pub struct MODULEID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module ID Register"]
pub mod moduleid;
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "I/O Routing Pin Enable Register"]
pub struct ROUTEPEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "I/O Route Location Register 0"]
pub struct ROUTELOC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Route Location Register 0"]
pub mod routeloc0;
