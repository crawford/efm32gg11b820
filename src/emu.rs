#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - Configuration Lock Register"]
    pub lock: LOCK,
    #[doc = "0x0c - Memory Control Register"]
    pub ram0ctrl: RAM0CTRL,
    #[doc = "0x10 - Command Register"]
    pub cmd: CMD,
    _reserved0: [u8; 4usize],
    #[doc = "0x18 - EM4 Control Register"]
    pub em4ctrl: EM4CTRL,
    #[doc = "0x1c - Temperature Limits for Interrupt Generation"]
    pub templimits: TEMPLIMITS,
    #[doc = "0x20 - Value of Last Temperature Measurement"]
    pub temp: TEMP,
    #[doc = "0x24 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x28 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x2c - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x30 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x34 - Regulator and Supply Lock Register"]
    pub pwrlock: PWRLOCK,
    _reserved1: [u8; 4usize],
    #[doc = "0x3c - Power Control Register"]
    pub pwrctrl: PWRCTRL,
    #[doc = "0x40 - DCDC Control"]
    pub dcdcctrl: DCDCCTRL,
    _reserved2: [u8; 8usize],
    #[doc = "0x4c - DCDC Miscellaneous Control Register"]
    pub dcdcmiscctrl: DCDCMISCCTRL,
    #[doc = "0x50 - DCDC Power Train NFET Zero Current Detector Control Register"]
    pub dcdczdetctrl: DCDCZDETCTRL,
    #[doc = "0x54 - DCDC Power Train PFET Current Limiter Control Register"]
    pub dcdcclimctrl: DCDCCLIMCTRL,
    #[doc = "0x58 - DCDC Low Noise Compensator Control Register"]
    pub dcdclncompctrl: DCDCLNCOMPCTRL,
    #[doc = "0x5c - DCDC Low Noise Voltage Register"]
    pub dcdclnvctrl: DCDCLNVCTRL,
    _reserved3: [u8; 4usize],
    #[doc = "0x64 - DCDC Low Power Voltage Register"]
    pub dcdclpvctrl: DCDCLPVCTRL,
    _reserved4: [u8; 4usize],
    #[doc = "0x6c - DCDC Low Power Control Register"]
    pub dcdclpctrl: DCDCLPCTRL,
    #[doc = "0x70 - DCDC Low Noise Controller Frequency Control"]
    pub dcdclnfreqctrl: DCDCLNFREQCTRL,
    _reserved5: [u8; 4usize],
    #[doc = "0x78 - DCDC Read Status Register"]
    pub dcdcsync: DCDCSYNC,
    _reserved6: [u8; 20usize],
    #[doc = "0x90 - VMON AVDD Channel Control"]
    pub vmonavddctrl: VMONAVDDCTRL,
    #[doc = "0x94 - Alternate VMON AVDD Channel Control"]
    pub vmonaltavddctrl: VMONALTAVDDCTRL,
    #[doc = "0x98 - VMON DVDD Channel Control"]
    pub vmondvddctrl: VMONDVDDCTRL,
    #[doc = "0x9c - VMON IOVDD0 Channel Control"]
    pub vmonio0ctrl: VMONIO0CTRL,
    #[doc = "0xa0 - VMON IOVDD1 Channel Control"]
    pub vmonio1ctrl: VMONIO1CTRL,
    #[doc = "0xa4 - VMON BUVDD Channel Control"]
    pub vmonbuvddctrl: VMONBUVDDCTRL,
    _reserved7: [u8; 12usize],
    #[doc = "0xb4 - Memory Control Register"]
    pub ram1ctrl: RAM1CTRL,
    #[doc = "0xb8 - Memory Control Register"]
    pub ram2ctrl: RAM2CTRL,
    #[doc = "0xbc - Backup Power Configuration Register"]
    pub buctrl: BUCTRL,
    _reserved8: [u8; 8usize],
    #[doc = "0xc8 - 5V Regulator Control"]
    pub r5vctrl: R5VCTRL,
    #[doc = "0xcc - 5V Regulator Control"]
    pub r5vadcctrl: R5VADCCTRL,
    #[doc = "0xd0 - 5V Regulator Voltage Select"]
    pub r5voutlevel: R5VOUTLEVEL,
    _reserved9: [u8; 8usize],
    #[doc = "0xdc - 5V Detector Enables"]
    pub r5vdetctrl: R5VDETCTRL,
    _reserved10: [u8; 12usize],
    #[doc = "0xec - Configuration Bits for Low Power Mode to Be Applied During EM01, This Field is Only Relevant If LP Mode is Used in EM01"]
    pub dcdclpem01cfg: DCDCLPEM01CFG,
    #[doc = "0xf0 - 5V Detector Status Register"]
    pub r5vstatus: R5VSTATUS,
    _reserved11: [u8; 4usize],
    #[doc = "0xf8 - 5V Read Status Register"]
    pub r5vsync: R5VSYNC,
    _reserved12: [u8; 4usize],
    #[doc = "0x100 - Clears Corresponding Bits in EM23PERNORETAINSTATUS Unlocking Access to Peripheral"]
    pub em23pernoretaincmd: EM23PERNORETAINCMD,
    #[doc = "0x104 - Status Indicating If Peripherals Were Powered Down in EM23, Subsequently Locking Access to It"]
    pub em23pernoretainstatus: EM23PERNORETAINSTATUS,
    #[doc = "0x108 - When Set Corresponding Peripherals May Get Powered Down in EM23"]
    pub em23pernoretainctrl: EM23PERNORETAINCTRL,
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
#[doc = "Configuration Lock Register"]
pub struct LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "Memory Control Register"]
pub struct RAM0CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Control Register"]
pub mod ram0ctrl;
#[doc = "Command Register"]
pub struct CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "EM4 Control Register"]
pub struct EM4CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EM4 Control Register"]
pub mod em4ctrl;
#[doc = "Temperature Limits for Interrupt Generation"]
pub struct TEMPLIMITS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Temperature Limits for Interrupt Generation"]
pub mod templimits;
#[doc = "Value of Last Temperature Measurement"]
pub struct TEMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value of Last Temperature Measurement"]
pub mod temp;
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
#[doc = "Regulator and Supply Lock Register"]
pub struct PWRLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Regulator and Supply Lock Register"]
pub mod pwrlock;
#[doc = "Power Control Register"]
pub struct PWRCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Control Register"]
pub mod pwrctrl;
#[doc = "DCDC Control"]
pub struct DCDCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Control"]
pub mod dcdcctrl;
#[doc = "DCDC Miscellaneous Control Register"]
pub struct DCDCMISCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Miscellaneous Control Register"]
pub mod dcdcmiscctrl;
#[doc = "DCDC Power Train NFET Zero Current Detector Control Register"]
pub struct DCDCZDETCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Power Train NFET Zero Current Detector Control Register"]
pub mod dcdczdetctrl;
#[doc = "DCDC Power Train PFET Current Limiter Control Register"]
pub struct DCDCCLIMCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Power Train PFET Current Limiter Control Register"]
pub mod dcdcclimctrl;
#[doc = "DCDC Low Noise Compensator Control Register"]
pub struct DCDCLNCOMPCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Low Noise Compensator Control Register"]
pub mod dcdclncompctrl;
#[doc = "DCDC Low Noise Voltage Register"]
pub struct DCDCLNVCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Low Noise Voltage Register"]
pub mod dcdclnvctrl;
#[doc = "DCDC Low Power Voltage Register"]
pub struct DCDCLPVCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Low Power Voltage Register"]
pub mod dcdclpvctrl;
#[doc = "DCDC Low Power Control Register"]
pub struct DCDCLPCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Low Power Control Register"]
pub mod dcdclpctrl;
#[doc = "DCDC Low Noise Controller Frequency Control"]
pub struct DCDCLNFREQCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Low Noise Controller Frequency Control"]
pub mod dcdclnfreqctrl;
#[doc = "DCDC Read Status Register"]
pub struct DCDCSYNC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCDC Read Status Register"]
pub mod dcdcsync;
#[doc = "VMON AVDD Channel Control"]
pub struct VMONAVDDCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VMON AVDD Channel Control"]
pub mod vmonavddctrl;
#[doc = "Alternate VMON AVDD Channel Control"]
pub struct VMONALTAVDDCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alternate VMON AVDD Channel Control"]
pub mod vmonaltavddctrl;
#[doc = "VMON DVDD Channel Control"]
pub struct VMONDVDDCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VMON DVDD Channel Control"]
pub mod vmondvddctrl;
#[doc = "VMON IOVDD0 Channel Control"]
pub struct VMONIO0CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VMON IOVDD0 Channel Control"]
pub mod vmonio0ctrl;
#[doc = "VMON IOVDD1 Channel Control"]
pub struct VMONIO1CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VMON IOVDD1 Channel Control"]
pub mod vmonio1ctrl;
#[doc = "VMON BUVDD Channel Control"]
pub struct VMONBUVDDCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VMON BUVDD Channel Control"]
pub mod vmonbuvddctrl;
#[doc = "Memory Control Register"]
pub struct RAM1CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Control Register"]
pub mod ram1ctrl;
#[doc = "Memory Control Register"]
pub struct RAM2CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Control Register"]
pub mod ram2ctrl;
#[doc = "Backup Power Configuration Register"]
pub struct BUCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup Power Configuration Register"]
pub mod buctrl;
#[doc = "5V Regulator Control"]
pub struct R5VCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "5V Regulator Control"]
pub mod r5vctrl;
#[doc = "5V Regulator Control"]
pub struct R5VADCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "5V Regulator Control"]
pub mod r5vadcctrl;
#[doc = "5V Regulator Voltage Select"]
pub struct R5VOUTLEVEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "5V Regulator Voltage Select"]
pub mod r5voutlevel;
#[doc = "5V Detector Enables"]
pub struct R5VDETCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "5V Detector Enables"]
pub mod r5vdetctrl;
#[doc = "Configuration Bits for Low Power Mode to Be Applied During EM01, This Field is Only Relevant If LP Mode is Used in EM01"]
pub struct DCDCLPEM01CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Bits for Low Power Mode to Be Applied During EM01, This Field is Only Relevant If LP Mode is Used in EM01"]
pub mod dcdclpem01cfg;
#[doc = "5V Detector Status Register"]
pub struct R5VSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "5V Detector Status Register"]
pub mod r5vstatus;
#[doc = "5V Read Status Register"]
pub struct R5VSYNC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "5V Read Status Register"]
pub mod r5vsync;
#[doc = "Clears Corresponding Bits in EM23PERNORETAINSTATUS Unlocking Access to Peripheral"]
pub struct EM23PERNORETAINCMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clears Corresponding Bits in EM23PERNORETAINSTATUS Unlocking Access to Peripheral"]
pub mod em23pernoretaincmd;
#[doc = "Status Indicating If Peripherals Were Powered Down in EM23, Subsequently Locking Access to It"]
pub struct EM23PERNORETAINSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Indicating If Peripherals Were Powered Down in EM23, Subsequently Locking Access to It"]
pub mod em23pernoretainstatus;
#[doc = "When Set Corresponding Peripherals May Get Powered Down in EM23"]
pub struct EM23PERNORETAINCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "When Set Corresponding Peripherals May Get Powered Down in EM23"]
pub mod em23pernoretainctrl;
