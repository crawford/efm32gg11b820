#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - Channel 0 Control Register"]
    pub ch0ctrl: CH0CTRL,
    #[doc = "0x0c - Channel 1 Control Register"]
    pub ch1ctrl: CH1CTRL,
    #[doc = "0x10 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x14 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x18 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x1c - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x20 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x24 - Channel 0 Data Register"]
    pub ch0data: CH0DATA,
    #[doc = "0x28 - Channel 1 Data Register"]
    pub ch1data: CH1DATA,
    #[doc = "0x2c - Combined Data Register"]
    pub combdata: COMBDATA,
    #[doc = "0x30 - Calibration Register"]
    pub cal: CAL,
    _reserved0: [u8; 108usize],
    #[doc = "0xa0 - Operational Amplifier APORT Request Status Register"]
    pub opa0_aportreq: OPA0_APORTREQ,
    #[doc = "0xa4 - Operational Amplifier APORT Conflict Status Register"]
    pub opa0_aportconflict: OPA0_APORTCONFLICT,
    #[doc = "0xa8 - Operational Amplifier Control Register"]
    pub opa0_ctrl: OPA0_CTRL,
    #[doc = "0xac - Operational Amplifier Timer Control Register"]
    pub opa0_timer: OPA0_TIMER,
    #[doc = "0xb0 - Operational Amplifier Mux Configuration Register"]
    pub opa0_mux: OPA0_MUX,
    #[doc = "0xb4 - Operational Amplifier Output Configuration Register"]
    pub opa0_out: OPA0_OUT,
    #[doc = "0xb8 - Operational Amplifier Calibration Register"]
    pub opa0_cal: OPA0_CAL,
    _reserved1: [u8; 4usize],
    #[doc = "0xc0 - Operational Amplifier APORT Request Status Register"]
    pub opa1_aportreq: OPA1_APORTREQ,
    #[doc = "0xc4 - Operational Amplifier APORT Conflict Status Register"]
    pub opa1_aportconflict: OPA1_APORTCONFLICT,
    #[doc = "0xc8 - Operational Amplifier Control Register"]
    pub opa1_ctrl: OPA1_CTRL,
    #[doc = "0xcc - Operational Amplifier Timer Control Register"]
    pub opa1_timer: OPA1_TIMER,
    #[doc = "0xd0 - Operational Amplifier Mux Configuration Register"]
    pub opa1_mux: OPA1_MUX,
    #[doc = "0xd4 - Operational Amplifier Output Configuration Register"]
    pub opa1_out: OPA1_OUT,
    #[doc = "0xd8 - Operational Amplifier Calibration Register"]
    pub opa1_cal: OPA1_CAL,
    _reserved2: [u8; 4usize],
    #[doc = "0xe0 - Operational Amplifier APORT Request Status Register"]
    pub opa2_aportreq: OPA2_APORTREQ,
    #[doc = "0xe4 - Operational Amplifier APORT Conflict Status Register"]
    pub opa2_aportconflict: OPA2_APORTCONFLICT,
    #[doc = "0xe8 - Operational Amplifier Control Register"]
    pub opa2_ctrl: OPA2_CTRL,
    #[doc = "0xec - Operational Amplifier Timer Control Register"]
    pub opa2_timer: OPA2_TIMER,
    #[doc = "0xf0 - Operational Amplifier Mux Configuration Register"]
    pub opa2_mux: OPA2_MUX,
    #[doc = "0xf4 - Operational Amplifier Output Configuration Register"]
    pub opa2_out: OPA2_OUT,
    #[doc = "0xf8 - Operational Amplifier Calibration Register"]
    pub opa2_cal: OPA2_CAL,
    _reserved3: [u8; 4usize],
    #[doc = "0x100 - Operational Amplifier APORT Request Status Register"]
    pub opa3_aportreq: OPA3_APORTREQ,
    #[doc = "0x104 - Operational Amplifier APORT Conflict Status Register"]
    pub opa3_aportconflict: OPA3_APORTCONFLICT,
    #[doc = "0x108 - Operational Amplifier Control Register"]
    pub opa3_ctrl: OPA3_CTRL,
    #[doc = "0x10c - Operational Amplifier Timer Control Register"]
    pub opa3_timer: OPA3_TIMER,
    #[doc = "0x110 - Operational Amplifier Mux Configuration Register"]
    pub opa3_mux: OPA3_MUX,
    #[doc = "0x114 - Operational Amplifier Output Configuration Register"]
    pub opa3_out: OPA3_OUT,
    #[doc = "0x118 - Operational Amplifier Calibration Register"]
    pub opa3_cal: OPA3_CAL,
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
#[doc = "Channel 0 Control Register"]
pub struct CH0CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 0 Control Register"]
pub mod ch0ctrl;
#[doc = "Channel 1 Control Register"]
pub struct CH1CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 1 Control Register"]
pub mod ch1ctrl;
#[doc = "Command Register"]
pub struct CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Register"]
pub mod cmd;
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
#[doc = "Channel 0 Data Register"]
pub struct CH0DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 0 Data Register"]
pub mod ch0data;
#[doc = "Channel 1 Data Register"]
pub struct CH1DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 1 Data Register"]
pub mod ch1data;
#[doc = "Combined Data Register"]
pub struct COMBDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Combined Data Register"]
pub mod combdata;
#[doc = "Calibration Register"]
pub struct CAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration Register"]
pub mod cal;
#[doc = "Operational Amplifier APORT Request Status Register"]
pub struct OPA0_APORTREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier APORT Request Status Register"]
pub mod opa0_aportreq;
#[doc = "Operational Amplifier APORT Conflict Status Register"]
pub struct OPA0_APORTCONFLICT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier APORT Conflict Status Register"]
pub mod opa0_aportconflict;
#[doc = "Operational Amplifier Control Register"]
pub struct OPA0_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier Control Register"]
pub mod opa0_ctrl;
#[doc = "Operational Amplifier Timer Control Register"]
pub struct OPA0_TIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier Timer Control Register"]
pub mod opa0_timer;
#[doc = "Operational Amplifier Mux Configuration Register"]
pub struct OPA0_MUX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa0_mux;
#[doc = "Operational Amplifier Output Configuration Register"]
pub struct OPA0_OUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier Output Configuration Register"]
pub mod opa0_out;
#[doc = "Operational Amplifier Calibration Register"]
pub struct OPA0_CAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier Calibration Register"]
pub mod opa0_cal;
#[doc = "Operational Amplifier APORT Request Status Register"]
pub struct OPA1_APORTREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier APORT Request Status Register"]
pub mod opa1_aportreq;
#[doc = "Operational Amplifier APORT Conflict Status Register"]
pub struct OPA1_APORTCONFLICT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier APORT Conflict Status Register"]
pub mod opa1_aportconflict;
#[doc = "Operational Amplifier Control Register"]
pub struct OPA1_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier Control Register"]
pub mod opa1_ctrl;
#[doc = "Operational Amplifier Timer Control Register"]
pub struct OPA1_TIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier Timer Control Register"]
pub mod opa1_timer;
#[doc = "Operational Amplifier Mux Configuration Register"]
pub struct OPA1_MUX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa1_mux;
#[doc = "Operational Amplifier Output Configuration Register"]
pub struct OPA1_OUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier Output Configuration Register"]
pub mod opa1_out;
#[doc = "Operational Amplifier Calibration Register"]
pub struct OPA1_CAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier Calibration Register"]
pub mod opa1_cal;
#[doc = "Operational Amplifier APORT Request Status Register"]
pub struct OPA2_APORTREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier APORT Request Status Register"]
pub mod opa2_aportreq;
#[doc = "Operational Amplifier APORT Conflict Status Register"]
pub struct OPA2_APORTCONFLICT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier APORT Conflict Status Register"]
pub mod opa2_aportconflict;
#[doc = "Operational Amplifier Control Register"]
pub struct OPA2_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier Control Register"]
pub mod opa2_ctrl;
#[doc = "Operational Amplifier Timer Control Register"]
pub struct OPA2_TIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier Timer Control Register"]
pub mod opa2_timer;
#[doc = "Operational Amplifier Mux Configuration Register"]
pub struct OPA2_MUX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa2_mux;
#[doc = "Operational Amplifier Output Configuration Register"]
pub struct OPA2_OUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier Output Configuration Register"]
pub mod opa2_out;
#[doc = "Operational Amplifier Calibration Register"]
pub struct OPA2_CAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier Calibration Register"]
pub mod opa2_cal;
#[doc = "Operational Amplifier APORT Request Status Register"]
pub struct OPA3_APORTREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier APORT Request Status Register"]
pub mod opa3_aportreq;
#[doc = "Operational Amplifier APORT Conflict Status Register"]
pub struct OPA3_APORTCONFLICT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier APORT Conflict Status Register"]
pub mod opa3_aportconflict;
#[doc = "Operational Amplifier Control Register"]
pub struct OPA3_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier Control Register"]
pub mod opa3_ctrl;
#[doc = "Operational Amplifier Timer Control Register"]
pub struct OPA3_TIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier Timer Control Register"]
pub mod opa3_timer;
#[doc = "Operational Amplifier Mux Configuration Register"]
pub struct OPA3_MUX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa3_mux;
#[doc = "Operational Amplifier Output Configuration Register"]
pub struct OPA3_OUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier Output Configuration Register"]
pub mod opa3_out;
#[doc = "Operational Amplifier Calibration Register"]
pub struct OPA3_CAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operational Amplifier Calibration Register"]
pub mod opa3_cal;
