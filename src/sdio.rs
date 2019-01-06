#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDMA System Address Register"]
    pub sdmasysaddr: SDMASYSADDR,
    #[doc = "0x04 - Block Size and Block Count Register"]
    pub blksize: BLKSIZE,
    #[doc = "0x08 - SD Command Argument Register"]
    pub cmdarg1: CMDARG1,
    #[doc = "0x0c - Transfer Mode and Command Register"]
    pub tfrmode: TFRMODE,
    #[doc = "0x10 - Response0 and Response1 Register"]
    pub resp0: RESP0,
    #[doc = "0x14 - Response2 and Response3 Register"]
    pub resp2: RESP2,
    #[doc = "0x18 - Response4 and Response5 Register"]
    pub resp4: RESP4,
    #[doc = "0x1c - Response6 and Response7 Register"]
    pub resp6: RESP6,
    #[doc = "0x20 - Buffer Data Register"]
    pub bufdatport: BUFDATPORT,
    #[doc = "0x24 - Present State Register"]
    pub prsstat: PRSSTAT,
    #[doc = "0x28 - Host Control1, Power, Block Gap and Wakeup-up Control Register"]
    pub hostctrl1: HOSTCTRL1,
    #[doc = "0x2c - Clock Control, Timeout Control and Software Register"]
    pub clockctrl: CLOCKCTRL,
    #[doc = "0x30 - Normal and Error Interrupt Status Register"]
    pub ifcr: IFCR,
    #[doc = "0x34 - Normal and Error Interrupt Status Enable Register"]
    pub ifenc: IFENC,
    #[doc = "0x38 - Normal and Error Interrupt Signal Enable Register"]
    pub ien: IEN,
    #[doc = "0x3c - AUTO CMD12 Error Status and Host Control2 Register"]
    pub ac12errstat: AC12ERRSTAT,
    #[doc = "0x40 - Capabilities Register to Hold Bits 31~0"]
    pub capab0: CAPAB0,
    #[doc = "0x44 - Capabilities Register to Hold Bits 63~32"]
    pub capab2: CAPAB2,
    #[doc = "0x48 - Maximum Current Capabilities Register"]
    pub maxcurcapab: MAXCURCAPAB,
    _reserved0: [u8; 4usize],
    #[doc = "0x50 - Force Event Register for Auto CMD Error Status"]
    pub fevterrstat: FEVTERRSTAT,
    #[doc = "0x54 - ADMA Error Status Register"]
    pub admaes: ADMAES,
    #[doc = "0x58 - ADMA System Address Register"]
    pub adsaddr: ADSADDR,
    _reserved1: [u8; 4usize],
    #[doc = "0x60 - Preset Value for Initialization and Default Speed Mode"]
    pub prstval0: PRSTVAL0,
    #[doc = "0x64 - Preset Value for High Speed and SDR12 Modes"]
    pub prstval2: PRSTVAL2,
    #[doc = "0x68 - Preset Value for SDR25 and SDR50 Modes"]
    pub prstval4: PRSTVAL4,
    #[doc = "0x6c - Preset Value for SDR104 and DDR50 Modes"]
    pub prstval6: PRSTVAL6,
    #[doc = "0x70 - Boot Timeout Control Register"]
    pub boottoctrl: BOOTTOCTRL,
    _reserved2: [u8; 136usize],
    #[doc = "0xfc - Slot Interrupt Status Register"]
    pub slotintstat: SLOTINTSTAT,
    _reserved3: [u8; 1792usize],
    #[doc = "0x800 - Core Control Signals"]
    pub ctrl: CTRL,
    #[doc = "0x804 - Core Configuration 0"]
    pub cfg0: CFG0,
    #[doc = "0x808 - Core Configuration 1"]
    pub cfg1: CFG1,
    #[doc = "0x80c - Core Configuration Preset Value 0"]
    pub cfgpresetval0: CFGPRESETVAL0,
    #[doc = "0x810 - Core Configuration Preset Value 1"]
    pub cfgpresetval1: CFGPRESETVAL1,
    #[doc = "0x814 - Core Configuration Preset Value 2"]
    pub cfgpresetval2: CFGPRESETVAL2,
    #[doc = "0x818 - Core Configuration Preset Value 3"]
    pub cfgpresetval3: CFGPRESETVAL3,
    #[doc = "0x81c - I/O LOCATION Register"]
    pub routeloc0: ROUTELOC0,
    #[doc = "0x820 - I/O LOCATION Register"]
    pub routeloc1: ROUTELOC1,
    #[doc = "0x824 - I/O LOCATION Enable Register"]
    pub routepen: ROUTEPEN,
}
#[doc = "SDMA System Address Register"]
pub struct SDMASYSADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDMA System Address Register"]
pub mod sdmasysaddr;
#[doc = "Block Size and Block Count Register"]
pub struct BLKSIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Block Size and Block Count Register"]
pub mod blksize;
#[doc = "SD Command Argument Register"]
pub struct CMDARG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SD Command Argument Register"]
pub mod cmdarg1;
#[doc = "Transfer Mode and Command Register"]
pub struct TFRMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer Mode and Command Register"]
pub mod tfrmode;
#[doc = "Response0 and Response1 Register"]
pub struct RESP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Response0 and Response1 Register"]
pub mod resp0;
#[doc = "Response2 and Response3 Register"]
pub struct RESP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Response2 and Response3 Register"]
pub mod resp2;
#[doc = "Response4 and Response5 Register"]
pub struct RESP4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Response4 and Response5 Register"]
pub mod resp4;
#[doc = "Response6 and Response7 Register"]
pub struct RESP6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Response6 and Response7 Register"]
pub mod resp6;
#[doc = "Buffer Data Register"]
pub struct BUFDATPORT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Buffer Data Register"]
pub mod bufdatport;
#[doc = "Present State Register"]
pub struct PRSSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Present State Register"]
pub mod prsstat;
#[doc = "Host Control1, Power, Block Gap and Wakeup-up Control Register"]
pub struct HOSTCTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Control1, Power, Block Gap and Wakeup-up Control Register"]
pub mod hostctrl1;
#[doc = "Clock Control, Timeout Control and Software Register"]
pub struct CLOCKCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Control, Timeout Control and Software Register"]
pub mod clockctrl;
#[doc = "Normal and Error Interrupt Status Register"]
pub struct IFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Normal and Error Interrupt Status Register"]
pub mod ifcr;
#[doc = "Normal and Error Interrupt Status Enable Register"]
pub struct IFENC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Normal and Error Interrupt Status Enable Register"]
pub mod ifenc;
#[doc = "Normal and Error Interrupt Signal Enable Register"]
pub struct IEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Normal and Error Interrupt Signal Enable Register"]
pub mod ien;
#[doc = "AUTO CMD12 Error Status and Host Control2 Register"]
pub struct AC12ERRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AUTO CMD12 Error Status and Host Control2 Register"]
pub mod ac12errstat;
#[doc = "Capabilities Register to Hold Bits 31~0"]
pub struct CAPAB0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capabilities Register to Hold Bits 31~0"]
pub mod capab0;
#[doc = "Capabilities Register to Hold Bits 63~32"]
pub struct CAPAB2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capabilities Register to Hold Bits 63~32"]
pub mod capab2;
#[doc = "Maximum Current Capabilities Register"]
pub struct MAXCURCAPAB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Maximum Current Capabilities Register"]
pub mod maxcurcapab;
#[doc = "Force Event Register for Auto CMD Error Status"]
pub struct FEVTERRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Force Event Register for Auto CMD Error Status"]
pub mod fevterrstat;
#[doc = "ADMA Error Status Register"]
pub struct ADMAES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADMA Error Status Register"]
pub mod admaes;
#[doc = "ADMA System Address Register"]
pub struct ADSADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADMA System Address Register"]
pub mod adsaddr;
#[doc = "Preset Value for Initialization and Default Speed Mode"]
pub struct PRSTVAL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Preset Value for Initialization and Default Speed Mode"]
pub mod prstval0;
#[doc = "Preset Value for High Speed and SDR12 Modes"]
pub struct PRSTVAL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Preset Value for High Speed and SDR12 Modes"]
pub mod prstval2;
#[doc = "Preset Value for SDR25 and SDR50 Modes"]
pub struct PRSTVAL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Preset Value for SDR25 and SDR50 Modes"]
pub mod prstval4;
#[doc = "Preset Value for SDR104 and DDR50 Modes"]
pub struct PRSTVAL6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Preset Value for SDR104 and DDR50 Modes"]
pub mod prstval6;
#[doc = "Boot Timeout Control Register"]
pub struct BOOTTOCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Boot Timeout Control Register"]
pub mod boottoctrl;
#[doc = "Slot Interrupt Status Register"]
pub struct SLOTINTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slot Interrupt Status Register"]
pub mod slotintstat;
#[doc = "Core Control Signals"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core Control Signals"]
pub mod ctrl;
#[doc = "Core Configuration 0"]
pub struct CFG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core Configuration 0"]
pub mod cfg0;
#[doc = "Core Configuration 1"]
pub struct CFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core Configuration 1"]
pub mod cfg1;
#[doc = "Core Configuration Preset Value 0"]
pub struct CFGPRESETVAL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core Configuration Preset Value 0"]
pub mod cfgpresetval0;
#[doc = "Core Configuration Preset Value 1"]
pub struct CFGPRESETVAL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core Configuration Preset Value 1"]
pub mod cfgpresetval1;
#[doc = "Core Configuration Preset Value 2"]
pub struct CFGPRESETVAL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core Configuration Preset Value 2"]
pub mod cfgpresetval2;
#[doc = "Core Configuration Preset Value 3"]
pub struct CFGPRESETVAL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core Configuration Preset Value 3"]
pub mod cfgpresetval3;
#[doc = "I/O LOCATION Register"]
pub struct ROUTELOC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O LOCATION Register"]
pub mod routeloc0;
#[doc = "I/O LOCATION Register"]
pub struct ROUTELOC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O LOCATION Register"]
pub mod routeloc1;
#[doc = "I/O LOCATION Enable Register"]
pub struct ROUTEPEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O LOCATION Enable Register"]
pub mod routepen;
