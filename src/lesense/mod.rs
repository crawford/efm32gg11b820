#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Timing Control Register"]
    pub timctrl: TIMCTRL,
    #[doc = "0x08 - Peripheral Control Register"]
    pub perctrl: PERCTRL,
    #[doc = "0x0c - Decoder Control Register"]
    pub decctrl: DECCTRL,
    #[doc = "0x10 - Bias Control Register"]
    pub biasctrl: BIASCTRL,
    #[doc = "0x14 - LESENSE Evaluation Control"]
    pub evalctrl: EVALCTRL,
    #[doc = "0x18 - PRS Control Register"]
    pub prsctrl: PRSCTRL,
    #[doc = "0x1c - Command Register"]
    pub cmd: CMD,
    #[doc = "0x20 - Channel Enable Register"]
    pub chen: CHEN,
    #[doc = "0x24 - Scan Result Register"]
    pub scanres: SCANRES,
    #[doc = "0x28 - Status Register"]
    pub status: STATUS,
    #[doc = "0x2c - Result Buffer Pointers"]
    pub ptr: PTR,
    #[doc = "0x30 - Result Buffer Data Register"]
    pub bufdata: BUFDATA,
    #[doc = "0x34 - Current Channel Index"]
    pub curch: CURCH,
    #[doc = "0x38 - Current Decoder State"]
    pub decstate: DECSTATE,
    #[doc = "0x3c - Decoder Input Register"]
    pub sensorstate: SENSORSTATE,
    #[doc = "0x40 - GPIO Idle Phase Configuration"]
    pub idleconf: IDLECONF,
    #[doc = "0x44 - Alternative Excite Pin Configuration"]
    pub altexconf: ALTEXCONF,
    _reserved0: [u8; 8usize],
    #[doc = "0x50 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x54 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x58 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x5c - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x60 - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x64 - I/O Routing Register"]
    pub routepen: ROUTEPEN,
    _reserved1: [u8; 152usize],
    #[doc = "0x100 - State Transition Configuration a"]
    pub st0_tconfa: ST0_TCONFA,
    #[doc = "0x104 - State Transition Configuration B"]
    pub st0_tconfb: ST0_TCONFB,
    #[doc = "0x108 - State Transition Configuration a"]
    pub st1_tconfa: ST1_TCONFA,
    #[doc = "0x10c - State Transition Configuration B"]
    pub st1_tconfb: ST1_TCONFB,
    #[doc = "0x110 - State Transition Configuration a"]
    pub st2_tconfa: ST2_TCONFA,
    #[doc = "0x114 - State Transition Configuration B"]
    pub st2_tconfb: ST2_TCONFB,
    #[doc = "0x118 - State Transition Configuration a"]
    pub st3_tconfa: ST3_TCONFA,
    #[doc = "0x11c - State Transition Configuration B"]
    pub st3_tconfb: ST3_TCONFB,
    #[doc = "0x120 - State Transition Configuration a"]
    pub st4_tconfa: ST4_TCONFA,
    #[doc = "0x124 - State Transition Configuration B"]
    pub st4_tconfb: ST4_TCONFB,
    #[doc = "0x128 - State Transition Configuration a"]
    pub st5_tconfa: ST5_TCONFA,
    #[doc = "0x12c - State Transition Configuration B"]
    pub st5_tconfb: ST5_TCONFB,
    #[doc = "0x130 - State Transition Configuration a"]
    pub st6_tconfa: ST6_TCONFA,
    #[doc = "0x134 - State Transition Configuration B"]
    pub st6_tconfb: ST6_TCONFB,
    #[doc = "0x138 - State Transition Configuration a"]
    pub st7_tconfa: ST7_TCONFA,
    #[doc = "0x13c - State Transition Configuration B"]
    pub st7_tconfb: ST7_TCONFB,
    #[doc = "0x140 - State Transition Configuration a"]
    pub st8_tconfa: ST8_TCONFA,
    #[doc = "0x144 - State Transition Configuration B"]
    pub st8_tconfb: ST8_TCONFB,
    #[doc = "0x148 - State Transition Configuration a"]
    pub st9_tconfa: ST9_TCONFA,
    #[doc = "0x14c - State Transition Configuration B"]
    pub st9_tconfb: ST9_TCONFB,
    #[doc = "0x150 - State Transition Configuration a"]
    pub st10_tconfa: ST10_TCONFA,
    #[doc = "0x154 - State Transition Configuration B"]
    pub st10_tconfb: ST10_TCONFB,
    #[doc = "0x158 - State Transition Configuration a"]
    pub st11_tconfa: ST11_TCONFA,
    #[doc = "0x15c - State Transition Configuration B"]
    pub st11_tconfb: ST11_TCONFB,
    #[doc = "0x160 - State Transition Configuration a"]
    pub st12_tconfa: ST12_TCONFA,
    #[doc = "0x164 - State Transition Configuration B"]
    pub st12_tconfb: ST12_TCONFB,
    #[doc = "0x168 - State Transition Configuration a"]
    pub st13_tconfa: ST13_TCONFA,
    #[doc = "0x16c - State Transition Configuration B"]
    pub st13_tconfb: ST13_TCONFB,
    #[doc = "0x170 - State Transition Configuration a"]
    pub st14_tconfa: ST14_TCONFA,
    #[doc = "0x174 - State Transition Configuration B"]
    pub st14_tconfb: ST14_TCONFB,
    #[doc = "0x178 - State Transition Configuration a"]
    pub st15_tconfa: ST15_TCONFA,
    #[doc = "0x17c - State Transition Configuration B"]
    pub st15_tconfb: ST15_TCONFB,
    #[doc = "0x180 - State Transition Configuration a"]
    pub st16_tconfa: ST16_TCONFA,
    #[doc = "0x184 - State Transition Configuration B"]
    pub st16_tconfb: ST16_TCONFB,
    #[doc = "0x188 - State Transition Configuration a"]
    pub st17_tconfa: ST17_TCONFA,
    #[doc = "0x18c - State Transition Configuration B"]
    pub st17_tconfb: ST17_TCONFB,
    #[doc = "0x190 - State Transition Configuration a"]
    pub st18_tconfa: ST18_TCONFA,
    #[doc = "0x194 - State Transition Configuration B"]
    pub st18_tconfb: ST18_TCONFB,
    #[doc = "0x198 - State Transition Configuration a"]
    pub st19_tconfa: ST19_TCONFA,
    #[doc = "0x19c - State Transition Configuration B"]
    pub st19_tconfb: ST19_TCONFB,
    #[doc = "0x1a0 - State Transition Configuration a"]
    pub st20_tconfa: ST20_TCONFA,
    #[doc = "0x1a4 - State Transition Configuration B"]
    pub st20_tconfb: ST20_TCONFB,
    #[doc = "0x1a8 - State Transition Configuration a"]
    pub st21_tconfa: ST21_TCONFA,
    #[doc = "0x1ac - State Transition Configuration B"]
    pub st21_tconfb: ST21_TCONFB,
    #[doc = "0x1b0 - State Transition Configuration a"]
    pub st22_tconfa: ST22_TCONFA,
    #[doc = "0x1b4 - State Transition Configuration B"]
    pub st22_tconfb: ST22_TCONFB,
    #[doc = "0x1b8 - State Transition Configuration a"]
    pub st23_tconfa: ST23_TCONFA,
    #[doc = "0x1bc - State Transition Configuration B"]
    pub st23_tconfb: ST23_TCONFB,
    #[doc = "0x1c0 - State Transition Configuration a"]
    pub st24_tconfa: ST24_TCONFA,
    #[doc = "0x1c4 - State Transition Configuration B"]
    pub st24_tconfb: ST24_TCONFB,
    #[doc = "0x1c8 - State Transition Configuration a"]
    pub st25_tconfa: ST25_TCONFA,
    #[doc = "0x1cc - State Transition Configuration B"]
    pub st25_tconfb: ST25_TCONFB,
    #[doc = "0x1d0 - State Transition Configuration a"]
    pub st26_tconfa: ST26_TCONFA,
    #[doc = "0x1d4 - State Transition Configuration B"]
    pub st26_tconfb: ST26_TCONFB,
    #[doc = "0x1d8 - State Transition Configuration a"]
    pub st27_tconfa: ST27_TCONFA,
    #[doc = "0x1dc - State Transition Configuration B"]
    pub st27_tconfb: ST27_TCONFB,
    #[doc = "0x1e0 - State Transition Configuration a"]
    pub st28_tconfa: ST28_TCONFA,
    #[doc = "0x1e4 - State Transition Configuration B"]
    pub st28_tconfb: ST28_TCONFB,
    #[doc = "0x1e8 - State Transition Configuration a"]
    pub st29_tconfa: ST29_TCONFA,
    #[doc = "0x1ec - State Transition Configuration B"]
    pub st29_tconfb: ST29_TCONFB,
    #[doc = "0x1f0 - State Transition Configuration a"]
    pub st30_tconfa: ST30_TCONFA,
    #[doc = "0x1f4 - State Transition Configuration B"]
    pub st30_tconfb: ST30_TCONFB,
    #[doc = "0x1f8 - State Transition Configuration a"]
    pub st31_tconfa: ST31_TCONFA,
    #[doc = "0x1fc - State Transition Configuration B"]
    pub st31_tconfb: ST31_TCONFB,
    #[doc = "0x200 - Scan Results"]
    pub buf0_data: BUF0_DATA,
    #[doc = "0x204 - Scan Results"]
    pub buf1_data: BUF1_DATA,
    #[doc = "0x208 - Scan Results"]
    pub buf2_data: BUF2_DATA,
    #[doc = "0x20c - Scan Results"]
    pub buf3_data: BUF3_DATA,
    #[doc = "0x210 - Scan Results"]
    pub buf4_data: BUF4_DATA,
    #[doc = "0x214 - Scan Results"]
    pub buf5_data: BUF5_DATA,
    #[doc = "0x218 - Scan Results"]
    pub buf6_data: BUF6_DATA,
    #[doc = "0x21c - Scan Results"]
    pub buf7_data: BUF7_DATA,
    #[doc = "0x220 - Scan Results"]
    pub buf8_data: BUF8_DATA,
    #[doc = "0x224 - Scan Results"]
    pub buf9_data: BUF9_DATA,
    #[doc = "0x228 - Scan Results"]
    pub buf10_data: BUF10_DATA,
    #[doc = "0x22c - Scan Results"]
    pub buf11_data: BUF11_DATA,
    #[doc = "0x230 - Scan Results"]
    pub buf12_data: BUF12_DATA,
    #[doc = "0x234 - Scan Results"]
    pub buf13_data: BUF13_DATA,
    #[doc = "0x238 - Scan Results"]
    pub buf14_data: BUF14_DATA,
    #[doc = "0x23c - Scan Results"]
    pub buf15_data: BUF15_DATA,
    #[doc = "0x240 - Scan Configuration"]
    pub ch0_timing: CH0_TIMING,
    #[doc = "0x244 - Scan Configuration"]
    pub ch0_interact: CH0_INTERACT,
    #[doc = "0x248 - Scan Configuration"]
    pub ch0_eval: CH0_EVAL,
    _reserved2: [u8; 4usize],
    #[doc = "0x250 - Scan Configuration"]
    pub ch1_timing: CH1_TIMING,
    #[doc = "0x254 - Scan Configuration"]
    pub ch1_interact: CH1_INTERACT,
    #[doc = "0x258 - Scan Configuration"]
    pub ch1_eval: CH1_EVAL,
    _reserved3: [u8; 4usize],
    #[doc = "0x260 - Scan Configuration"]
    pub ch2_timing: CH2_TIMING,
    #[doc = "0x264 - Scan Configuration"]
    pub ch2_interact: CH2_INTERACT,
    #[doc = "0x268 - Scan Configuration"]
    pub ch2_eval: CH2_EVAL,
    _reserved4: [u8; 4usize],
    #[doc = "0x270 - Scan Configuration"]
    pub ch3_timing: CH3_TIMING,
    #[doc = "0x274 - Scan Configuration"]
    pub ch3_interact: CH3_INTERACT,
    #[doc = "0x278 - Scan Configuration"]
    pub ch3_eval: CH3_EVAL,
    _reserved5: [u8; 4usize],
    #[doc = "0x280 - Scan Configuration"]
    pub ch4_timing: CH4_TIMING,
    #[doc = "0x284 - Scan Configuration"]
    pub ch4_interact: CH4_INTERACT,
    #[doc = "0x288 - Scan Configuration"]
    pub ch4_eval: CH4_EVAL,
    _reserved6: [u8; 4usize],
    #[doc = "0x290 - Scan Configuration"]
    pub ch5_timing: CH5_TIMING,
    #[doc = "0x294 - Scan Configuration"]
    pub ch5_interact: CH5_INTERACT,
    #[doc = "0x298 - Scan Configuration"]
    pub ch5_eval: CH5_EVAL,
    _reserved7: [u8; 4usize],
    #[doc = "0x2a0 - Scan Configuration"]
    pub ch6_timing: CH6_TIMING,
    #[doc = "0x2a4 - Scan Configuration"]
    pub ch6_interact: CH6_INTERACT,
    #[doc = "0x2a8 - Scan Configuration"]
    pub ch6_eval: CH6_EVAL,
    _reserved8: [u8; 4usize],
    #[doc = "0x2b0 - Scan Configuration"]
    pub ch7_timing: CH7_TIMING,
    #[doc = "0x2b4 - Scan Configuration"]
    pub ch7_interact: CH7_INTERACT,
    #[doc = "0x2b8 - Scan Configuration"]
    pub ch7_eval: CH7_EVAL,
    _reserved9: [u8; 4usize],
    #[doc = "0x2c0 - Scan Configuration"]
    pub ch8_timing: CH8_TIMING,
    #[doc = "0x2c4 - Scan Configuration"]
    pub ch8_interact: CH8_INTERACT,
    #[doc = "0x2c8 - Scan Configuration"]
    pub ch8_eval: CH8_EVAL,
    _reserved10: [u8; 4usize],
    #[doc = "0x2d0 - Scan Configuration"]
    pub ch9_timing: CH9_TIMING,
    #[doc = "0x2d4 - Scan Configuration"]
    pub ch9_interact: CH9_INTERACT,
    #[doc = "0x2d8 - Scan Configuration"]
    pub ch9_eval: CH9_EVAL,
    _reserved11: [u8; 4usize],
    #[doc = "0x2e0 - Scan Configuration"]
    pub ch10_timing: CH10_TIMING,
    #[doc = "0x2e4 - Scan Configuration"]
    pub ch10_interact: CH10_INTERACT,
    #[doc = "0x2e8 - Scan Configuration"]
    pub ch10_eval: CH10_EVAL,
    _reserved12: [u8; 4usize],
    #[doc = "0x2f0 - Scan Configuration"]
    pub ch11_timing: CH11_TIMING,
    #[doc = "0x2f4 - Scan Configuration"]
    pub ch11_interact: CH11_INTERACT,
    #[doc = "0x2f8 - Scan Configuration"]
    pub ch11_eval: CH11_EVAL,
    _reserved13: [u8; 4usize],
    #[doc = "0x300 - Scan Configuration"]
    pub ch12_timing: CH12_TIMING,
    #[doc = "0x304 - Scan Configuration"]
    pub ch12_interact: CH12_INTERACT,
    #[doc = "0x308 - Scan Configuration"]
    pub ch12_eval: CH12_EVAL,
    _reserved14: [u8; 4usize],
    #[doc = "0x310 - Scan Configuration"]
    pub ch13_timing: CH13_TIMING,
    #[doc = "0x314 - Scan Configuration"]
    pub ch13_interact: CH13_INTERACT,
    #[doc = "0x318 - Scan Configuration"]
    pub ch13_eval: CH13_EVAL,
    _reserved15: [u8; 4usize],
    #[doc = "0x320 - Scan Configuration"]
    pub ch14_timing: CH14_TIMING,
    #[doc = "0x324 - Scan Configuration"]
    pub ch14_interact: CH14_INTERACT,
    #[doc = "0x328 - Scan Configuration"]
    pub ch14_eval: CH14_EVAL,
    _reserved16: [u8; 4usize],
    #[doc = "0x330 - Scan Configuration"]
    pub ch15_timing: CH15_TIMING,
    #[doc = "0x334 - Scan Configuration"]
    pub ch15_interact: CH15_INTERACT,
    #[doc = "0x338 - Scan Configuration"]
    pub ch15_eval: CH15_EVAL,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Timing Control Register"]
pub struct TIMCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timing Control Register"]
pub mod timctrl;
#[doc = "Peripheral Control Register"]
pub struct PERCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Control Register"]
pub mod perctrl;
#[doc = "Decoder Control Register"]
pub struct DECCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Decoder Control Register"]
pub mod decctrl;
#[doc = "Bias Control Register"]
pub struct BIASCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bias Control Register"]
pub mod biasctrl;
#[doc = "LESENSE Evaluation Control"]
pub struct EVALCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LESENSE Evaluation Control"]
pub mod evalctrl;
#[doc = "PRS Control Register"]
pub struct PRSCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PRS Control Register"]
pub mod prsctrl;
#[doc = "Command Register"]
pub struct CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "Channel Enable Register"]
pub struct CHEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Enable Register"]
pub mod chen;
#[doc = "Scan Result Register"]
pub struct SCANRES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Result Register"]
pub mod scanres;
#[doc = "Status Register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod status;
#[doc = "Result Buffer Pointers"]
pub struct PTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Result Buffer Pointers"]
pub mod ptr;
#[doc = "Result Buffer Data Register"]
pub struct BUFDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Result Buffer Data Register"]
pub mod bufdata;
#[doc = "Current Channel Index"]
pub struct CURCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Channel Index"]
pub mod curch;
#[doc = "Current Decoder State"]
pub struct DECSTATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Decoder State"]
pub mod decstate;
#[doc = "Decoder Input Register"]
pub struct SENSORSTATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Decoder Input Register"]
pub mod sensorstate;
#[doc = "GPIO Idle Phase Configuration"]
pub struct IDLECONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Idle Phase Configuration"]
pub mod idleconf;
#[doc = "Alternative Excite Pin Configuration"]
pub struct ALTEXCONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alternative Excite Pin Configuration"]
pub mod altexconf;
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
#[doc = "Synchronization Busy Register"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "I/O Routing Register"]
pub struct ROUTEPEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Routing Register"]
pub mod routepen;
#[doc = "State Transition Configuration a"]
pub struct ST0_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st0_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST0_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st0_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST1_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st1_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST1_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st1_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST2_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st2_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST2_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st2_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST3_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st3_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST3_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st3_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST4_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st4_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST4_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st4_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST5_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st5_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST5_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st5_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST6_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st6_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST6_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st6_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST7_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st7_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST7_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st7_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST8_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st8_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST8_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st8_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST9_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st9_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST9_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st9_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST10_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st10_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST10_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st10_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST11_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st11_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST11_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st11_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST12_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st12_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST12_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st12_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST13_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st13_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST13_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st13_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST14_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st14_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST14_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st14_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST15_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st15_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST15_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st15_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST16_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st16_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST16_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st16_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST17_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st17_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST17_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st17_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST18_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st18_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST18_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st18_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST19_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st19_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST19_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st19_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST20_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st20_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST20_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st20_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST21_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st21_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST21_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st21_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST22_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st22_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST22_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st22_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST23_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st23_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST23_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st23_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST24_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st24_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST24_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st24_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST25_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st25_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST25_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st25_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST26_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st26_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST26_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st26_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST27_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st27_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST27_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st27_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST28_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st28_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST28_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st28_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST29_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st29_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST29_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st29_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST30_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st30_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST30_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st30_tconfb;
#[doc = "State Transition Configuration a"]
pub struct ST31_TCONFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration a"]
pub mod st31_tconfa;
#[doc = "State Transition Configuration B"]
pub struct ST31_TCONFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State Transition Configuration B"]
pub mod st31_tconfb;
#[doc = "Scan Results"]
pub struct BUF0_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Results"]
pub mod buf0_data;
#[doc = "Scan Results"]
pub struct BUF1_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Results"]
pub mod buf1_data;
#[doc = "Scan Results"]
pub struct BUF2_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Results"]
pub mod buf2_data;
#[doc = "Scan Results"]
pub struct BUF3_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Results"]
pub mod buf3_data;
#[doc = "Scan Results"]
pub struct BUF4_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Results"]
pub mod buf4_data;
#[doc = "Scan Results"]
pub struct BUF5_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Results"]
pub mod buf5_data;
#[doc = "Scan Results"]
pub struct BUF6_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Results"]
pub mod buf6_data;
#[doc = "Scan Results"]
pub struct BUF7_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Results"]
pub mod buf7_data;
#[doc = "Scan Results"]
pub struct BUF8_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Results"]
pub mod buf8_data;
#[doc = "Scan Results"]
pub struct BUF9_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Results"]
pub mod buf9_data;
#[doc = "Scan Results"]
pub struct BUF10_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Results"]
pub mod buf10_data;
#[doc = "Scan Results"]
pub struct BUF11_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Results"]
pub mod buf11_data;
#[doc = "Scan Results"]
pub struct BUF12_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Results"]
pub mod buf12_data;
#[doc = "Scan Results"]
pub struct BUF13_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Results"]
pub mod buf13_data;
#[doc = "Scan Results"]
pub struct BUF14_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Results"]
pub mod buf14_data;
#[doc = "Scan Results"]
pub struct BUF15_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Results"]
pub mod buf15_data;
#[doc = "Scan Configuration"]
pub struct CH0_TIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch0_timing;
#[doc = "Scan Configuration"]
pub struct CH0_INTERACT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch0_interact;
#[doc = "Scan Configuration"]
pub struct CH0_EVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch0_eval;
#[doc = "Scan Configuration"]
pub struct CH1_TIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch1_timing;
#[doc = "Scan Configuration"]
pub struct CH1_INTERACT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch1_interact;
#[doc = "Scan Configuration"]
pub struct CH1_EVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch1_eval;
#[doc = "Scan Configuration"]
pub struct CH2_TIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch2_timing;
#[doc = "Scan Configuration"]
pub struct CH2_INTERACT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch2_interact;
#[doc = "Scan Configuration"]
pub struct CH2_EVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch2_eval;
#[doc = "Scan Configuration"]
pub struct CH3_TIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch3_timing;
#[doc = "Scan Configuration"]
pub struct CH3_INTERACT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch3_interact;
#[doc = "Scan Configuration"]
pub struct CH3_EVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch3_eval;
#[doc = "Scan Configuration"]
pub struct CH4_TIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch4_timing;
#[doc = "Scan Configuration"]
pub struct CH4_INTERACT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch4_interact;
#[doc = "Scan Configuration"]
pub struct CH4_EVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch4_eval;
#[doc = "Scan Configuration"]
pub struct CH5_TIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch5_timing;
#[doc = "Scan Configuration"]
pub struct CH5_INTERACT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch5_interact;
#[doc = "Scan Configuration"]
pub struct CH5_EVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch5_eval;
#[doc = "Scan Configuration"]
pub struct CH6_TIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch6_timing;
#[doc = "Scan Configuration"]
pub struct CH6_INTERACT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch6_interact;
#[doc = "Scan Configuration"]
pub struct CH6_EVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch6_eval;
#[doc = "Scan Configuration"]
pub struct CH7_TIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch7_timing;
#[doc = "Scan Configuration"]
pub struct CH7_INTERACT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch7_interact;
#[doc = "Scan Configuration"]
pub struct CH7_EVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch7_eval;
#[doc = "Scan Configuration"]
pub struct CH8_TIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch8_timing;
#[doc = "Scan Configuration"]
pub struct CH8_INTERACT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch8_interact;
#[doc = "Scan Configuration"]
pub struct CH8_EVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch8_eval;
#[doc = "Scan Configuration"]
pub struct CH9_TIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch9_timing;
#[doc = "Scan Configuration"]
pub struct CH9_INTERACT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch9_interact;
#[doc = "Scan Configuration"]
pub struct CH9_EVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch9_eval;
#[doc = "Scan Configuration"]
pub struct CH10_TIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch10_timing;
#[doc = "Scan Configuration"]
pub struct CH10_INTERACT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch10_interact;
#[doc = "Scan Configuration"]
pub struct CH10_EVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch10_eval;
#[doc = "Scan Configuration"]
pub struct CH11_TIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch11_timing;
#[doc = "Scan Configuration"]
pub struct CH11_INTERACT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch11_interact;
#[doc = "Scan Configuration"]
pub struct CH11_EVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch11_eval;
#[doc = "Scan Configuration"]
pub struct CH12_TIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch12_timing;
#[doc = "Scan Configuration"]
pub struct CH12_INTERACT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch12_interact;
#[doc = "Scan Configuration"]
pub struct CH12_EVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch12_eval;
#[doc = "Scan Configuration"]
pub struct CH13_TIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch13_timing;
#[doc = "Scan Configuration"]
pub struct CH13_INTERACT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch13_interact;
#[doc = "Scan Configuration"]
pub struct CH13_EVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch13_eval;
#[doc = "Scan Configuration"]
pub struct CH14_TIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch14_timing;
#[doc = "Scan Configuration"]
pub struct CH14_INTERACT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch14_interact;
#[doc = "Scan Configuration"]
pub struct CH14_EVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch14_eval;
#[doc = "Scan Configuration"]
pub struct CH15_TIMING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch15_timing;
#[doc = "Scan Configuration"]
pub struct CH15_INTERACT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch15_interact;
#[doc = "Scan Configuration"]
pub struct CH15_EVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scan Configuration"]
pub mod ch15_eval;
