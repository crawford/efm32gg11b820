#[doc = r"Register block"]
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
    _reserved18: [u8; 8usize],
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
    _reserved24: [u8; 152usize],
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
    _reserved107: [u8; 4usize],
    #[doc = "0x250 - Scan Configuration"]
    pub ch1_timing: CH1_TIMING,
    #[doc = "0x254 - Scan Configuration"]
    pub ch1_interact: CH1_INTERACT,
    #[doc = "0x258 - Scan Configuration"]
    pub ch1_eval: CH1_EVAL,
    _reserved110: [u8; 4usize],
    #[doc = "0x260 - Scan Configuration"]
    pub ch2_timing: CH2_TIMING,
    #[doc = "0x264 - Scan Configuration"]
    pub ch2_interact: CH2_INTERACT,
    #[doc = "0x268 - Scan Configuration"]
    pub ch2_eval: CH2_EVAL,
    _reserved113: [u8; 4usize],
    #[doc = "0x270 - Scan Configuration"]
    pub ch3_timing: CH3_TIMING,
    #[doc = "0x274 - Scan Configuration"]
    pub ch3_interact: CH3_INTERACT,
    #[doc = "0x278 - Scan Configuration"]
    pub ch3_eval: CH3_EVAL,
    _reserved116: [u8; 4usize],
    #[doc = "0x280 - Scan Configuration"]
    pub ch4_timing: CH4_TIMING,
    #[doc = "0x284 - Scan Configuration"]
    pub ch4_interact: CH4_INTERACT,
    #[doc = "0x288 - Scan Configuration"]
    pub ch4_eval: CH4_EVAL,
    _reserved119: [u8; 4usize],
    #[doc = "0x290 - Scan Configuration"]
    pub ch5_timing: CH5_TIMING,
    #[doc = "0x294 - Scan Configuration"]
    pub ch5_interact: CH5_INTERACT,
    #[doc = "0x298 - Scan Configuration"]
    pub ch5_eval: CH5_EVAL,
    _reserved122: [u8; 4usize],
    #[doc = "0x2a0 - Scan Configuration"]
    pub ch6_timing: CH6_TIMING,
    #[doc = "0x2a4 - Scan Configuration"]
    pub ch6_interact: CH6_INTERACT,
    #[doc = "0x2a8 - Scan Configuration"]
    pub ch6_eval: CH6_EVAL,
    _reserved125: [u8; 4usize],
    #[doc = "0x2b0 - Scan Configuration"]
    pub ch7_timing: CH7_TIMING,
    #[doc = "0x2b4 - Scan Configuration"]
    pub ch7_interact: CH7_INTERACT,
    #[doc = "0x2b8 - Scan Configuration"]
    pub ch7_eval: CH7_EVAL,
    _reserved128: [u8; 4usize],
    #[doc = "0x2c0 - Scan Configuration"]
    pub ch8_timing: CH8_TIMING,
    #[doc = "0x2c4 - Scan Configuration"]
    pub ch8_interact: CH8_INTERACT,
    #[doc = "0x2c8 - Scan Configuration"]
    pub ch8_eval: CH8_EVAL,
    _reserved131: [u8; 4usize],
    #[doc = "0x2d0 - Scan Configuration"]
    pub ch9_timing: CH9_TIMING,
    #[doc = "0x2d4 - Scan Configuration"]
    pub ch9_interact: CH9_INTERACT,
    #[doc = "0x2d8 - Scan Configuration"]
    pub ch9_eval: CH9_EVAL,
    _reserved134: [u8; 4usize],
    #[doc = "0x2e0 - Scan Configuration"]
    pub ch10_timing: CH10_TIMING,
    #[doc = "0x2e4 - Scan Configuration"]
    pub ch10_interact: CH10_INTERACT,
    #[doc = "0x2e8 - Scan Configuration"]
    pub ch10_eval: CH10_EVAL,
    _reserved137: [u8; 4usize],
    #[doc = "0x2f0 - Scan Configuration"]
    pub ch11_timing: CH11_TIMING,
    #[doc = "0x2f4 - Scan Configuration"]
    pub ch11_interact: CH11_INTERACT,
    #[doc = "0x2f8 - Scan Configuration"]
    pub ch11_eval: CH11_EVAL,
    _reserved140: [u8; 4usize],
    #[doc = "0x300 - Scan Configuration"]
    pub ch12_timing: CH12_TIMING,
    #[doc = "0x304 - Scan Configuration"]
    pub ch12_interact: CH12_INTERACT,
    #[doc = "0x308 - Scan Configuration"]
    pub ch12_eval: CH12_EVAL,
    _reserved143: [u8; 4usize],
    #[doc = "0x310 - Scan Configuration"]
    pub ch13_timing: CH13_TIMING,
    #[doc = "0x314 - Scan Configuration"]
    pub ch13_interact: CH13_INTERACT,
    #[doc = "0x318 - Scan Configuration"]
    pub ch13_eval: CH13_EVAL,
    _reserved146: [u8; 4usize],
    #[doc = "0x320 - Scan Configuration"]
    pub ch14_timing: CH14_TIMING,
    #[doc = "0x324 - Scan Configuration"]
    pub ch14_interact: CH14_INTERACT,
    #[doc = "0x328 - Scan Configuration"]
    pub ch14_eval: CH14_EVAL,
    _reserved149: [u8; 4usize],
    #[doc = "0x330 - Scan Configuration"]
    pub ch15_timing: CH15_TIMING,
    #[doc = "0x334 - Scan Configuration"]
    pub ch15_interact: CH15_INTERACT,
    #[doc = "0x338 - Scan Configuration"]
    pub ch15_eval: CH15_EVAL,
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Timing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timctrl](timctrl) module"]
pub type TIMCTRL = crate::Reg<u32, _TIMCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCTRL;
#[doc = "`read()` method returns [timctrl::R](timctrl::R) reader structure"]
impl crate::Readable for TIMCTRL {}
#[doc = "`write(|w| ..)` method takes [timctrl::W](timctrl::W) writer structure"]
impl crate::Writable for TIMCTRL {}
#[doc = "Timing Control Register"]
pub mod timctrl;
#[doc = "Peripheral Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perctrl](perctrl) module"]
pub type PERCTRL = crate::Reg<u32, _PERCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERCTRL;
#[doc = "`read()` method returns [perctrl::R](perctrl::R) reader structure"]
impl crate::Readable for PERCTRL {}
#[doc = "`write(|w| ..)` method takes [perctrl::W](perctrl::W) writer structure"]
impl crate::Writable for PERCTRL {}
#[doc = "Peripheral Control Register"]
pub mod perctrl;
#[doc = "Decoder Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [decctrl](decctrl) module"]
pub type DECCTRL = crate::Reg<u32, _DECCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DECCTRL;
#[doc = "`read()` method returns [decctrl::R](decctrl::R) reader structure"]
impl crate::Readable for DECCTRL {}
#[doc = "`write(|w| ..)` method takes [decctrl::W](decctrl::W) writer structure"]
impl crate::Writable for DECCTRL {}
#[doc = "Decoder Control Register"]
pub mod decctrl;
#[doc = "Bias Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [biasctrl](biasctrl) module"]
pub type BIASCTRL = crate::Reg<u32, _BIASCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BIASCTRL;
#[doc = "`read()` method returns [biasctrl::R](biasctrl::R) reader structure"]
impl crate::Readable for BIASCTRL {}
#[doc = "`write(|w| ..)` method takes [biasctrl::W](biasctrl::W) writer structure"]
impl crate::Writable for BIASCTRL {}
#[doc = "Bias Control Register"]
pub mod biasctrl;
#[doc = "LESENSE Evaluation Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evalctrl](evalctrl) module"]
pub type EVALCTRL = crate::Reg<u32, _EVALCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVALCTRL;
#[doc = "`read()` method returns [evalctrl::R](evalctrl::R) reader structure"]
impl crate::Readable for EVALCTRL {}
#[doc = "`write(|w| ..)` method takes [evalctrl::W](evalctrl::W) writer structure"]
impl crate::Writable for EVALCTRL {}
#[doc = "LESENSE Evaluation Control"]
pub mod evalctrl;
#[doc = "PRS Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prsctrl](prsctrl) module"]
pub type PRSCTRL = crate::Reg<u32, _PRSCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRSCTRL;
#[doc = "`read()` method returns [prsctrl::R](prsctrl::R) reader structure"]
impl crate::Readable for PRSCTRL {}
#[doc = "`write(|w| ..)` method takes [prsctrl::W](prsctrl::W) writer structure"]
impl crate::Writable for PRSCTRL {}
#[doc = "PRS Control Register"]
pub mod prsctrl;
#[doc = "Command Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "Channel Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chen](chen) module"]
pub type CHEN = crate::Reg<u32, _CHEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHEN;
#[doc = "`read()` method returns [chen::R](chen::R) reader structure"]
impl crate::Readable for CHEN {}
#[doc = "`write(|w| ..)` method takes [chen::W](chen::W) writer structure"]
impl crate::Writable for CHEN {}
#[doc = "Channel Enable Register"]
pub mod chen;
#[doc = "Scan Result Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scanres](scanres) module"]
pub type SCANRES = crate::Reg<u32, _SCANRES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCANRES;
#[doc = "`read()` method returns [scanres::R](scanres::R) reader structure"]
impl crate::Readable for SCANRES {}
#[doc = "`write(|w| ..)` method takes [scanres::W](scanres::W) writer structure"]
impl crate::Writable for SCANRES {}
#[doc = "Scan Result Register"]
pub mod scanres;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status Register"]
pub mod status;
#[doc = "Result Buffer Pointers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptr](ptr) module"]
pub type PTR = crate::Reg<u32, _PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTR;
#[doc = "`read()` method returns [ptr::R](ptr::R) reader structure"]
impl crate::Readable for PTR {}
#[doc = "Result Buffer Pointers"]
pub mod ptr;
#[doc = "Result Buffer Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bufdata](bufdata) module"]
pub type BUFDATA = crate::Reg<u32, _BUFDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUFDATA;
#[doc = "`read()` method returns [bufdata::R](bufdata::R) reader structure"]
impl crate::Readable for BUFDATA {}
#[doc = "Result Buffer Data Register"]
pub mod bufdata;
#[doc = "Current Channel Index\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [curch](curch) module"]
pub type CURCH = crate::Reg<u32, _CURCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CURCH;
#[doc = "`read()` method returns [curch::R](curch::R) reader structure"]
impl crate::Readable for CURCH {}
#[doc = "Current Channel Index"]
pub mod curch;
#[doc = "Current Decoder State\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [decstate](decstate) module"]
pub type DECSTATE = crate::Reg<u32, _DECSTATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DECSTATE;
#[doc = "`read()` method returns [decstate::R](decstate::R) reader structure"]
impl crate::Readable for DECSTATE {}
#[doc = "`write(|w| ..)` method takes [decstate::W](decstate::W) writer structure"]
impl crate::Writable for DECSTATE {}
#[doc = "Current Decoder State"]
pub mod decstate;
#[doc = "Decoder Input Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensorstate](sensorstate) module"]
pub type SENSORSTATE = crate::Reg<u32, _SENSORSTATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SENSORSTATE;
#[doc = "`read()` method returns [sensorstate::R](sensorstate::R) reader structure"]
impl crate::Readable for SENSORSTATE {}
#[doc = "`write(|w| ..)` method takes [sensorstate::W](sensorstate::W) writer structure"]
impl crate::Writable for SENSORSTATE {}
#[doc = "Decoder Input Register"]
pub mod sensorstate;
#[doc = "GPIO Idle Phase Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idleconf](idleconf) module"]
pub type IDLECONF = crate::Reg<u32, _IDLECONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDLECONF;
#[doc = "`read()` method returns [idleconf::R](idleconf::R) reader structure"]
impl crate::Readable for IDLECONF {}
#[doc = "`write(|w| ..)` method takes [idleconf::W](idleconf::W) writer structure"]
impl crate::Writable for IDLECONF {}
#[doc = "GPIO Idle Phase Configuration"]
pub mod idleconf;
#[doc = "Alternative Excite Pin Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altexconf](altexconf) module"]
pub type ALTEXCONF = crate::Reg<u32, _ALTEXCONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTEXCONF;
#[doc = "`read()` method returns [altexconf::R](altexconf::R) reader structure"]
impl crate::Readable for ALTEXCONF {}
#[doc = "`write(|w| ..)` method takes [altexconf::W](altexconf::W) writer structure"]
impl crate::Writable for ALTEXCONF {}
#[doc = "Alternative Excite Pin Configuration"]
pub mod altexconf;
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](if_) module"]
pub type IF = crate::Reg<u32, _IF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF;
#[doc = "`read()` method returns [if_::R](if_::R) reader structure"]
impl crate::Readable for IF {}
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "Interrupt Flag Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs](ifs) module"]
pub type IFS = crate::Reg<u32, _IFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFS;
#[doc = "`write(|w| ..)` method takes [ifs::W](ifs::W) writer structure"]
impl crate::Writable for IFS {}
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "Interrupt Flag Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifc](ifc) module"]
pub type IFC = crate::Reg<u32, _IFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFC;
#[doc = "`write(|w| ..)` method takes [ifc::W](ifc::W) writer structure"]
impl crate::Writable for IFC {}
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](ien) module"]
pub type IEN = crate::Reg<u32, _IEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEN;
#[doc = "`read()` method returns [ien::R](ien::R) reader structure"]
impl crate::Readable for IEN {}
#[doc = "`write(|w| ..)` method takes [ien::W](ien::W) writer structure"]
impl crate::Writable for IEN {}
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "Synchronization Busy Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](syncbusy) module"]
pub type SYNCBUSY = crate::Reg<u32, _SYNCBUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNCBUSY;
#[doc = "`read()` method returns [syncbusy::R](syncbusy::R) reader structure"]
impl crate::Readable for SYNCBUSY {}
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "I/O Routing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routepen](routepen) module"]
pub type ROUTEPEN = crate::Reg<u32, _ROUTEPEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTEPEN;
#[doc = "`read()` method returns [routepen::R](routepen::R) reader structure"]
impl crate::Readable for ROUTEPEN {}
#[doc = "`write(|w| ..)` method takes [routepen::W](routepen::W) writer structure"]
impl crate::Writable for ROUTEPEN {}
#[doc = "I/O Routing Register"]
pub mod routepen;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st0_tconfa](st0_tconfa) module"]
pub type ST0_TCONFA = crate::Reg<u32, _ST0_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST0_TCONFA;
#[doc = "`read()` method returns [st0_tconfa::R](st0_tconfa::R) reader structure"]
impl crate::Readable for ST0_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st0_tconfa::W](st0_tconfa::W) writer structure"]
impl crate::Writable for ST0_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st0_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st0_tconfb](st0_tconfb) module"]
pub type ST0_TCONFB = crate::Reg<u32, _ST0_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST0_TCONFB;
#[doc = "`read()` method returns [st0_tconfb::R](st0_tconfb::R) reader structure"]
impl crate::Readable for ST0_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st0_tconfb::W](st0_tconfb::W) writer structure"]
impl crate::Writable for ST0_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st0_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st1_tconfa](st1_tconfa) module"]
pub type ST1_TCONFA = crate::Reg<u32, _ST1_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST1_TCONFA;
#[doc = "`read()` method returns [st1_tconfa::R](st1_tconfa::R) reader structure"]
impl crate::Readable for ST1_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st1_tconfa::W](st1_tconfa::W) writer structure"]
impl crate::Writable for ST1_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st1_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st1_tconfb](st1_tconfb) module"]
pub type ST1_TCONFB = crate::Reg<u32, _ST1_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST1_TCONFB;
#[doc = "`read()` method returns [st1_tconfb::R](st1_tconfb::R) reader structure"]
impl crate::Readable for ST1_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st1_tconfb::W](st1_tconfb::W) writer structure"]
impl crate::Writable for ST1_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st1_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2_tconfa](st2_tconfa) module"]
pub type ST2_TCONFA = crate::Reg<u32, _ST2_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2_TCONFA;
#[doc = "`read()` method returns [st2_tconfa::R](st2_tconfa::R) reader structure"]
impl crate::Readable for ST2_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st2_tconfa::W](st2_tconfa::W) writer structure"]
impl crate::Writable for ST2_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st2_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2_tconfb](st2_tconfb) module"]
pub type ST2_TCONFB = crate::Reg<u32, _ST2_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2_TCONFB;
#[doc = "`read()` method returns [st2_tconfb::R](st2_tconfb::R) reader structure"]
impl crate::Readable for ST2_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st2_tconfb::W](st2_tconfb::W) writer structure"]
impl crate::Writable for ST2_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st2_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st3_tconfa](st3_tconfa) module"]
pub type ST3_TCONFA = crate::Reg<u32, _ST3_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST3_TCONFA;
#[doc = "`read()` method returns [st3_tconfa::R](st3_tconfa::R) reader structure"]
impl crate::Readable for ST3_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st3_tconfa::W](st3_tconfa::W) writer structure"]
impl crate::Writable for ST3_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st3_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st3_tconfb](st3_tconfb) module"]
pub type ST3_TCONFB = crate::Reg<u32, _ST3_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST3_TCONFB;
#[doc = "`read()` method returns [st3_tconfb::R](st3_tconfb::R) reader structure"]
impl crate::Readable for ST3_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st3_tconfb::W](st3_tconfb::W) writer structure"]
impl crate::Writable for ST3_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st3_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st4_tconfa](st4_tconfa) module"]
pub type ST4_TCONFA = crate::Reg<u32, _ST4_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST4_TCONFA;
#[doc = "`read()` method returns [st4_tconfa::R](st4_tconfa::R) reader structure"]
impl crate::Readable for ST4_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st4_tconfa::W](st4_tconfa::W) writer structure"]
impl crate::Writable for ST4_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st4_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st4_tconfb](st4_tconfb) module"]
pub type ST4_TCONFB = crate::Reg<u32, _ST4_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST4_TCONFB;
#[doc = "`read()` method returns [st4_tconfb::R](st4_tconfb::R) reader structure"]
impl crate::Readable for ST4_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st4_tconfb::W](st4_tconfb::W) writer structure"]
impl crate::Writable for ST4_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st4_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st5_tconfa](st5_tconfa) module"]
pub type ST5_TCONFA = crate::Reg<u32, _ST5_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST5_TCONFA;
#[doc = "`read()` method returns [st5_tconfa::R](st5_tconfa::R) reader structure"]
impl crate::Readable for ST5_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st5_tconfa::W](st5_tconfa::W) writer structure"]
impl crate::Writable for ST5_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st5_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st5_tconfb](st5_tconfb) module"]
pub type ST5_TCONFB = crate::Reg<u32, _ST5_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST5_TCONFB;
#[doc = "`read()` method returns [st5_tconfb::R](st5_tconfb::R) reader structure"]
impl crate::Readable for ST5_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st5_tconfb::W](st5_tconfb::W) writer structure"]
impl crate::Writable for ST5_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st5_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st6_tconfa](st6_tconfa) module"]
pub type ST6_TCONFA = crate::Reg<u32, _ST6_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST6_TCONFA;
#[doc = "`read()` method returns [st6_tconfa::R](st6_tconfa::R) reader structure"]
impl crate::Readable for ST6_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st6_tconfa::W](st6_tconfa::W) writer structure"]
impl crate::Writable for ST6_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st6_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st6_tconfb](st6_tconfb) module"]
pub type ST6_TCONFB = crate::Reg<u32, _ST6_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST6_TCONFB;
#[doc = "`read()` method returns [st6_tconfb::R](st6_tconfb::R) reader structure"]
impl crate::Readable for ST6_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st6_tconfb::W](st6_tconfb::W) writer structure"]
impl crate::Writable for ST6_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st6_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st7_tconfa](st7_tconfa) module"]
pub type ST7_TCONFA = crate::Reg<u32, _ST7_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST7_TCONFA;
#[doc = "`read()` method returns [st7_tconfa::R](st7_tconfa::R) reader structure"]
impl crate::Readable for ST7_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st7_tconfa::W](st7_tconfa::W) writer structure"]
impl crate::Writable for ST7_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st7_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st7_tconfb](st7_tconfb) module"]
pub type ST7_TCONFB = crate::Reg<u32, _ST7_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST7_TCONFB;
#[doc = "`read()` method returns [st7_tconfb::R](st7_tconfb::R) reader structure"]
impl crate::Readable for ST7_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st7_tconfb::W](st7_tconfb::W) writer structure"]
impl crate::Writable for ST7_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st7_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st8_tconfa](st8_tconfa) module"]
pub type ST8_TCONFA = crate::Reg<u32, _ST8_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST8_TCONFA;
#[doc = "`read()` method returns [st8_tconfa::R](st8_tconfa::R) reader structure"]
impl crate::Readable for ST8_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st8_tconfa::W](st8_tconfa::W) writer structure"]
impl crate::Writable for ST8_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st8_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st8_tconfb](st8_tconfb) module"]
pub type ST8_TCONFB = crate::Reg<u32, _ST8_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST8_TCONFB;
#[doc = "`read()` method returns [st8_tconfb::R](st8_tconfb::R) reader structure"]
impl crate::Readable for ST8_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st8_tconfb::W](st8_tconfb::W) writer structure"]
impl crate::Writable for ST8_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st8_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st9_tconfa](st9_tconfa) module"]
pub type ST9_TCONFA = crate::Reg<u32, _ST9_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST9_TCONFA;
#[doc = "`read()` method returns [st9_tconfa::R](st9_tconfa::R) reader structure"]
impl crate::Readable for ST9_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st9_tconfa::W](st9_tconfa::W) writer structure"]
impl crate::Writable for ST9_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st9_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st9_tconfb](st9_tconfb) module"]
pub type ST9_TCONFB = crate::Reg<u32, _ST9_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST9_TCONFB;
#[doc = "`read()` method returns [st9_tconfb::R](st9_tconfb::R) reader structure"]
impl crate::Readable for ST9_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st9_tconfb::W](st9_tconfb::W) writer structure"]
impl crate::Writable for ST9_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st9_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st10_tconfa](st10_tconfa) module"]
pub type ST10_TCONFA = crate::Reg<u32, _ST10_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST10_TCONFA;
#[doc = "`read()` method returns [st10_tconfa::R](st10_tconfa::R) reader structure"]
impl crate::Readable for ST10_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st10_tconfa::W](st10_tconfa::W) writer structure"]
impl crate::Writable for ST10_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st10_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st10_tconfb](st10_tconfb) module"]
pub type ST10_TCONFB = crate::Reg<u32, _ST10_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST10_TCONFB;
#[doc = "`read()` method returns [st10_tconfb::R](st10_tconfb::R) reader structure"]
impl crate::Readable for ST10_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st10_tconfb::W](st10_tconfb::W) writer structure"]
impl crate::Writable for ST10_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st10_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st11_tconfa](st11_tconfa) module"]
pub type ST11_TCONFA = crate::Reg<u32, _ST11_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST11_TCONFA;
#[doc = "`read()` method returns [st11_tconfa::R](st11_tconfa::R) reader structure"]
impl crate::Readable for ST11_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st11_tconfa::W](st11_tconfa::W) writer structure"]
impl crate::Writable for ST11_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st11_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st11_tconfb](st11_tconfb) module"]
pub type ST11_TCONFB = crate::Reg<u32, _ST11_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST11_TCONFB;
#[doc = "`read()` method returns [st11_tconfb::R](st11_tconfb::R) reader structure"]
impl crate::Readable for ST11_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st11_tconfb::W](st11_tconfb::W) writer structure"]
impl crate::Writable for ST11_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st11_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st12_tconfa](st12_tconfa) module"]
pub type ST12_TCONFA = crate::Reg<u32, _ST12_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST12_TCONFA;
#[doc = "`read()` method returns [st12_tconfa::R](st12_tconfa::R) reader structure"]
impl crate::Readable for ST12_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st12_tconfa::W](st12_tconfa::W) writer structure"]
impl crate::Writable for ST12_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st12_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st12_tconfb](st12_tconfb) module"]
pub type ST12_TCONFB = crate::Reg<u32, _ST12_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST12_TCONFB;
#[doc = "`read()` method returns [st12_tconfb::R](st12_tconfb::R) reader structure"]
impl crate::Readable for ST12_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st12_tconfb::W](st12_tconfb::W) writer structure"]
impl crate::Writable for ST12_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st12_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st13_tconfa](st13_tconfa) module"]
pub type ST13_TCONFA = crate::Reg<u32, _ST13_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST13_TCONFA;
#[doc = "`read()` method returns [st13_tconfa::R](st13_tconfa::R) reader structure"]
impl crate::Readable for ST13_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st13_tconfa::W](st13_tconfa::W) writer structure"]
impl crate::Writable for ST13_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st13_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st13_tconfb](st13_tconfb) module"]
pub type ST13_TCONFB = crate::Reg<u32, _ST13_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST13_TCONFB;
#[doc = "`read()` method returns [st13_tconfb::R](st13_tconfb::R) reader structure"]
impl crate::Readable for ST13_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st13_tconfb::W](st13_tconfb::W) writer structure"]
impl crate::Writable for ST13_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st13_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st14_tconfa](st14_tconfa) module"]
pub type ST14_TCONFA = crate::Reg<u32, _ST14_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST14_TCONFA;
#[doc = "`read()` method returns [st14_tconfa::R](st14_tconfa::R) reader structure"]
impl crate::Readable for ST14_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st14_tconfa::W](st14_tconfa::W) writer structure"]
impl crate::Writable for ST14_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st14_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st14_tconfb](st14_tconfb) module"]
pub type ST14_TCONFB = crate::Reg<u32, _ST14_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST14_TCONFB;
#[doc = "`read()` method returns [st14_tconfb::R](st14_tconfb::R) reader structure"]
impl crate::Readable for ST14_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st14_tconfb::W](st14_tconfb::W) writer structure"]
impl crate::Writable for ST14_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st14_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st15_tconfa](st15_tconfa) module"]
pub type ST15_TCONFA = crate::Reg<u32, _ST15_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST15_TCONFA;
#[doc = "`read()` method returns [st15_tconfa::R](st15_tconfa::R) reader structure"]
impl crate::Readable for ST15_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st15_tconfa::W](st15_tconfa::W) writer structure"]
impl crate::Writable for ST15_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st15_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st15_tconfb](st15_tconfb) module"]
pub type ST15_TCONFB = crate::Reg<u32, _ST15_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST15_TCONFB;
#[doc = "`read()` method returns [st15_tconfb::R](st15_tconfb::R) reader structure"]
impl crate::Readable for ST15_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st15_tconfb::W](st15_tconfb::W) writer structure"]
impl crate::Writable for ST15_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st15_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st16_tconfa](st16_tconfa) module"]
pub type ST16_TCONFA = crate::Reg<u32, _ST16_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST16_TCONFA;
#[doc = "`read()` method returns [st16_tconfa::R](st16_tconfa::R) reader structure"]
impl crate::Readable for ST16_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st16_tconfa::W](st16_tconfa::W) writer structure"]
impl crate::Writable for ST16_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st16_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st16_tconfb](st16_tconfb) module"]
pub type ST16_TCONFB = crate::Reg<u32, _ST16_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST16_TCONFB;
#[doc = "`read()` method returns [st16_tconfb::R](st16_tconfb::R) reader structure"]
impl crate::Readable for ST16_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st16_tconfb::W](st16_tconfb::W) writer structure"]
impl crate::Writable for ST16_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st16_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st17_tconfa](st17_tconfa) module"]
pub type ST17_TCONFA = crate::Reg<u32, _ST17_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST17_TCONFA;
#[doc = "`read()` method returns [st17_tconfa::R](st17_tconfa::R) reader structure"]
impl crate::Readable for ST17_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st17_tconfa::W](st17_tconfa::W) writer structure"]
impl crate::Writable for ST17_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st17_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st17_tconfb](st17_tconfb) module"]
pub type ST17_TCONFB = crate::Reg<u32, _ST17_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST17_TCONFB;
#[doc = "`read()` method returns [st17_tconfb::R](st17_tconfb::R) reader structure"]
impl crate::Readable for ST17_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st17_tconfb::W](st17_tconfb::W) writer structure"]
impl crate::Writable for ST17_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st17_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st18_tconfa](st18_tconfa) module"]
pub type ST18_TCONFA = crate::Reg<u32, _ST18_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST18_TCONFA;
#[doc = "`read()` method returns [st18_tconfa::R](st18_tconfa::R) reader structure"]
impl crate::Readable for ST18_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st18_tconfa::W](st18_tconfa::W) writer structure"]
impl crate::Writable for ST18_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st18_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st18_tconfb](st18_tconfb) module"]
pub type ST18_TCONFB = crate::Reg<u32, _ST18_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST18_TCONFB;
#[doc = "`read()` method returns [st18_tconfb::R](st18_tconfb::R) reader structure"]
impl crate::Readable for ST18_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st18_tconfb::W](st18_tconfb::W) writer structure"]
impl crate::Writable for ST18_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st18_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st19_tconfa](st19_tconfa) module"]
pub type ST19_TCONFA = crate::Reg<u32, _ST19_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST19_TCONFA;
#[doc = "`read()` method returns [st19_tconfa::R](st19_tconfa::R) reader structure"]
impl crate::Readable for ST19_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st19_tconfa::W](st19_tconfa::W) writer structure"]
impl crate::Writable for ST19_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st19_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st19_tconfb](st19_tconfb) module"]
pub type ST19_TCONFB = crate::Reg<u32, _ST19_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST19_TCONFB;
#[doc = "`read()` method returns [st19_tconfb::R](st19_tconfb::R) reader structure"]
impl crate::Readable for ST19_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st19_tconfb::W](st19_tconfb::W) writer structure"]
impl crate::Writable for ST19_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st19_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st20_tconfa](st20_tconfa) module"]
pub type ST20_TCONFA = crate::Reg<u32, _ST20_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST20_TCONFA;
#[doc = "`read()` method returns [st20_tconfa::R](st20_tconfa::R) reader structure"]
impl crate::Readable for ST20_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st20_tconfa::W](st20_tconfa::W) writer structure"]
impl crate::Writable for ST20_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st20_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st20_tconfb](st20_tconfb) module"]
pub type ST20_TCONFB = crate::Reg<u32, _ST20_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST20_TCONFB;
#[doc = "`read()` method returns [st20_tconfb::R](st20_tconfb::R) reader structure"]
impl crate::Readable for ST20_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st20_tconfb::W](st20_tconfb::W) writer structure"]
impl crate::Writable for ST20_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st20_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st21_tconfa](st21_tconfa) module"]
pub type ST21_TCONFA = crate::Reg<u32, _ST21_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST21_TCONFA;
#[doc = "`read()` method returns [st21_tconfa::R](st21_tconfa::R) reader structure"]
impl crate::Readable for ST21_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st21_tconfa::W](st21_tconfa::W) writer structure"]
impl crate::Writable for ST21_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st21_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st21_tconfb](st21_tconfb) module"]
pub type ST21_TCONFB = crate::Reg<u32, _ST21_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST21_TCONFB;
#[doc = "`read()` method returns [st21_tconfb::R](st21_tconfb::R) reader structure"]
impl crate::Readable for ST21_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st21_tconfb::W](st21_tconfb::W) writer structure"]
impl crate::Writable for ST21_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st21_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st22_tconfa](st22_tconfa) module"]
pub type ST22_TCONFA = crate::Reg<u32, _ST22_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST22_TCONFA;
#[doc = "`read()` method returns [st22_tconfa::R](st22_tconfa::R) reader structure"]
impl crate::Readable for ST22_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st22_tconfa::W](st22_tconfa::W) writer structure"]
impl crate::Writable for ST22_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st22_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st22_tconfb](st22_tconfb) module"]
pub type ST22_TCONFB = crate::Reg<u32, _ST22_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST22_TCONFB;
#[doc = "`read()` method returns [st22_tconfb::R](st22_tconfb::R) reader structure"]
impl crate::Readable for ST22_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st22_tconfb::W](st22_tconfb::W) writer structure"]
impl crate::Writable for ST22_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st22_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st23_tconfa](st23_tconfa) module"]
pub type ST23_TCONFA = crate::Reg<u32, _ST23_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST23_TCONFA;
#[doc = "`read()` method returns [st23_tconfa::R](st23_tconfa::R) reader structure"]
impl crate::Readable for ST23_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st23_tconfa::W](st23_tconfa::W) writer structure"]
impl crate::Writable for ST23_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st23_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st23_tconfb](st23_tconfb) module"]
pub type ST23_TCONFB = crate::Reg<u32, _ST23_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST23_TCONFB;
#[doc = "`read()` method returns [st23_tconfb::R](st23_tconfb::R) reader structure"]
impl crate::Readable for ST23_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st23_tconfb::W](st23_tconfb::W) writer structure"]
impl crate::Writable for ST23_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st23_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st24_tconfa](st24_tconfa) module"]
pub type ST24_TCONFA = crate::Reg<u32, _ST24_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST24_TCONFA;
#[doc = "`read()` method returns [st24_tconfa::R](st24_tconfa::R) reader structure"]
impl crate::Readable for ST24_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st24_tconfa::W](st24_tconfa::W) writer structure"]
impl crate::Writable for ST24_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st24_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st24_tconfb](st24_tconfb) module"]
pub type ST24_TCONFB = crate::Reg<u32, _ST24_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST24_TCONFB;
#[doc = "`read()` method returns [st24_tconfb::R](st24_tconfb::R) reader structure"]
impl crate::Readable for ST24_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st24_tconfb::W](st24_tconfb::W) writer structure"]
impl crate::Writable for ST24_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st24_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st25_tconfa](st25_tconfa) module"]
pub type ST25_TCONFA = crate::Reg<u32, _ST25_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST25_TCONFA;
#[doc = "`read()` method returns [st25_tconfa::R](st25_tconfa::R) reader structure"]
impl crate::Readable for ST25_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st25_tconfa::W](st25_tconfa::W) writer structure"]
impl crate::Writable for ST25_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st25_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st25_tconfb](st25_tconfb) module"]
pub type ST25_TCONFB = crate::Reg<u32, _ST25_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST25_TCONFB;
#[doc = "`read()` method returns [st25_tconfb::R](st25_tconfb::R) reader structure"]
impl crate::Readable for ST25_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st25_tconfb::W](st25_tconfb::W) writer structure"]
impl crate::Writable for ST25_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st25_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st26_tconfa](st26_tconfa) module"]
pub type ST26_TCONFA = crate::Reg<u32, _ST26_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST26_TCONFA;
#[doc = "`read()` method returns [st26_tconfa::R](st26_tconfa::R) reader structure"]
impl crate::Readable for ST26_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st26_tconfa::W](st26_tconfa::W) writer structure"]
impl crate::Writable for ST26_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st26_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st26_tconfb](st26_tconfb) module"]
pub type ST26_TCONFB = crate::Reg<u32, _ST26_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST26_TCONFB;
#[doc = "`read()` method returns [st26_tconfb::R](st26_tconfb::R) reader structure"]
impl crate::Readable for ST26_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st26_tconfb::W](st26_tconfb::W) writer structure"]
impl crate::Writable for ST26_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st26_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st27_tconfa](st27_tconfa) module"]
pub type ST27_TCONFA = crate::Reg<u32, _ST27_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST27_TCONFA;
#[doc = "`read()` method returns [st27_tconfa::R](st27_tconfa::R) reader structure"]
impl crate::Readable for ST27_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st27_tconfa::W](st27_tconfa::W) writer structure"]
impl crate::Writable for ST27_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st27_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st27_tconfb](st27_tconfb) module"]
pub type ST27_TCONFB = crate::Reg<u32, _ST27_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST27_TCONFB;
#[doc = "`read()` method returns [st27_tconfb::R](st27_tconfb::R) reader structure"]
impl crate::Readable for ST27_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st27_tconfb::W](st27_tconfb::W) writer structure"]
impl crate::Writable for ST27_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st27_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st28_tconfa](st28_tconfa) module"]
pub type ST28_TCONFA = crate::Reg<u32, _ST28_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST28_TCONFA;
#[doc = "`read()` method returns [st28_tconfa::R](st28_tconfa::R) reader structure"]
impl crate::Readable for ST28_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st28_tconfa::W](st28_tconfa::W) writer structure"]
impl crate::Writable for ST28_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st28_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st28_tconfb](st28_tconfb) module"]
pub type ST28_TCONFB = crate::Reg<u32, _ST28_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST28_TCONFB;
#[doc = "`read()` method returns [st28_tconfb::R](st28_tconfb::R) reader structure"]
impl crate::Readable for ST28_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st28_tconfb::W](st28_tconfb::W) writer structure"]
impl crate::Writable for ST28_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st28_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st29_tconfa](st29_tconfa) module"]
pub type ST29_TCONFA = crate::Reg<u32, _ST29_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST29_TCONFA;
#[doc = "`read()` method returns [st29_tconfa::R](st29_tconfa::R) reader structure"]
impl crate::Readable for ST29_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st29_tconfa::W](st29_tconfa::W) writer structure"]
impl crate::Writable for ST29_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st29_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st29_tconfb](st29_tconfb) module"]
pub type ST29_TCONFB = crate::Reg<u32, _ST29_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST29_TCONFB;
#[doc = "`read()` method returns [st29_tconfb::R](st29_tconfb::R) reader structure"]
impl crate::Readable for ST29_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st29_tconfb::W](st29_tconfb::W) writer structure"]
impl crate::Writable for ST29_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st29_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st30_tconfa](st30_tconfa) module"]
pub type ST30_TCONFA = crate::Reg<u32, _ST30_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST30_TCONFA;
#[doc = "`read()` method returns [st30_tconfa::R](st30_tconfa::R) reader structure"]
impl crate::Readable for ST30_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st30_tconfa::W](st30_tconfa::W) writer structure"]
impl crate::Writable for ST30_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st30_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st30_tconfb](st30_tconfb) module"]
pub type ST30_TCONFB = crate::Reg<u32, _ST30_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST30_TCONFB;
#[doc = "`read()` method returns [st30_tconfb::R](st30_tconfb::R) reader structure"]
impl crate::Readable for ST30_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st30_tconfb::W](st30_tconfb::W) writer structure"]
impl crate::Writable for ST30_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st30_tconfb;
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st31_tconfa](st31_tconfa) module"]
pub type ST31_TCONFA = crate::Reg<u32, _ST31_TCONFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST31_TCONFA;
#[doc = "`read()` method returns [st31_tconfa::R](st31_tconfa::R) reader structure"]
impl crate::Readable for ST31_TCONFA {}
#[doc = "`write(|w| ..)` method takes [st31_tconfa::W](st31_tconfa::W) writer structure"]
impl crate::Writable for ST31_TCONFA {}
#[doc = "State Transition Configuration a"]
pub mod st31_tconfa;
#[doc = "State Transition Configuration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st31_tconfb](st31_tconfb) module"]
pub type ST31_TCONFB = crate::Reg<u32, _ST31_TCONFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST31_TCONFB;
#[doc = "`read()` method returns [st31_tconfb::R](st31_tconfb::R) reader structure"]
impl crate::Readable for ST31_TCONFB {}
#[doc = "`write(|w| ..)` method takes [st31_tconfb::W](st31_tconfb::W) writer structure"]
impl crate::Writable for ST31_TCONFB {}
#[doc = "State Transition Configuration B"]
pub mod st31_tconfb;
#[doc = "Scan Results\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf0_data](buf0_data) module"]
pub type BUF0_DATA = crate::Reg<u32, _BUF0_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF0_DATA;
#[doc = "`read()` method returns [buf0_data::R](buf0_data::R) reader structure"]
impl crate::Readable for BUF0_DATA {}
#[doc = "`write(|w| ..)` method takes [buf0_data::W](buf0_data::W) writer structure"]
impl crate::Writable for BUF0_DATA {}
#[doc = "Scan Results"]
pub mod buf0_data;
#[doc = "Scan Results\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf1_data](buf1_data) module"]
pub type BUF1_DATA = crate::Reg<u32, _BUF1_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF1_DATA;
#[doc = "`read()` method returns [buf1_data::R](buf1_data::R) reader structure"]
impl crate::Readable for BUF1_DATA {}
#[doc = "`write(|w| ..)` method takes [buf1_data::W](buf1_data::W) writer structure"]
impl crate::Writable for BUF1_DATA {}
#[doc = "Scan Results"]
pub mod buf1_data;
#[doc = "Scan Results\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf2_data](buf2_data) module"]
pub type BUF2_DATA = crate::Reg<u32, _BUF2_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF2_DATA;
#[doc = "`read()` method returns [buf2_data::R](buf2_data::R) reader structure"]
impl crate::Readable for BUF2_DATA {}
#[doc = "`write(|w| ..)` method takes [buf2_data::W](buf2_data::W) writer structure"]
impl crate::Writable for BUF2_DATA {}
#[doc = "Scan Results"]
pub mod buf2_data;
#[doc = "Scan Results\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf3_data](buf3_data) module"]
pub type BUF3_DATA = crate::Reg<u32, _BUF3_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF3_DATA;
#[doc = "`read()` method returns [buf3_data::R](buf3_data::R) reader structure"]
impl crate::Readable for BUF3_DATA {}
#[doc = "`write(|w| ..)` method takes [buf3_data::W](buf3_data::W) writer structure"]
impl crate::Writable for BUF3_DATA {}
#[doc = "Scan Results"]
pub mod buf3_data;
#[doc = "Scan Results\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf4_data](buf4_data) module"]
pub type BUF4_DATA = crate::Reg<u32, _BUF4_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF4_DATA;
#[doc = "`read()` method returns [buf4_data::R](buf4_data::R) reader structure"]
impl crate::Readable for BUF4_DATA {}
#[doc = "`write(|w| ..)` method takes [buf4_data::W](buf4_data::W) writer structure"]
impl crate::Writable for BUF4_DATA {}
#[doc = "Scan Results"]
pub mod buf4_data;
#[doc = "Scan Results\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf5_data](buf5_data) module"]
pub type BUF5_DATA = crate::Reg<u32, _BUF5_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF5_DATA;
#[doc = "`read()` method returns [buf5_data::R](buf5_data::R) reader structure"]
impl crate::Readable for BUF5_DATA {}
#[doc = "`write(|w| ..)` method takes [buf5_data::W](buf5_data::W) writer structure"]
impl crate::Writable for BUF5_DATA {}
#[doc = "Scan Results"]
pub mod buf5_data;
#[doc = "Scan Results\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf6_data](buf6_data) module"]
pub type BUF6_DATA = crate::Reg<u32, _BUF6_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF6_DATA;
#[doc = "`read()` method returns [buf6_data::R](buf6_data::R) reader structure"]
impl crate::Readable for BUF6_DATA {}
#[doc = "`write(|w| ..)` method takes [buf6_data::W](buf6_data::W) writer structure"]
impl crate::Writable for BUF6_DATA {}
#[doc = "Scan Results"]
pub mod buf6_data;
#[doc = "Scan Results\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf7_data](buf7_data) module"]
pub type BUF7_DATA = crate::Reg<u32, _BUF7_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF7_DATA;
#[doc = "`read()` method returns [buf7_data::R](buf7_data::R) reader structure"]
impl crate::Readable for BUF7_DATA {}
#[doc = "`write(|w| ..)` method takes [buf7_data::W](buf7_data::W) writer structure"]
impl crate::Writable for BUF7_DATA {}
#[doc = "Scan Results"]
pub mod buf7_data;
#[doc = "Scan Results\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf8_data](buf8_data) module"]
pub type BUF8_DATA = crate::Reg<u32, _BUF8_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF8_DATA;
#[doc = "`read()` method returns [buf8_data::R](buf8_data::R) reader structure"]
impl crate::Readable for BUF8_DATA {}
#[doc = "`write(|w| ..)` method takes [buf8_data::W](buf8_data::W) writer structure"]
impl crate::Writable for BUF8_DATA {}
#[doc = "Scan Results"]
pub mod buf8_data;
#[doc = "Scan Results\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf9_data](buf9_data) module"]
pub type BUF9_DATA = crate::Reg<u32, _BUF9_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF9_DATA;
#[doc = "`read()` method returns [buf9_data::R](buf9_data::R) reader structure"]
impl crate::Readable for BUF9_DATA {}
#[doc = "`write(|w| ..)` method takes [buf9_data::W](buf9_data::W) writer structure"]
impl crate::Writable for BUF9_DATA {}
#[doc = "Scan Results"]
pub mod buf9_data;
#[doc = "Scan Results\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf10_data](buf10_data) module"]
pub type BUF10_DATA = crate::Reg<u32, _BUF10_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF10_DATA;
#[doc = "`read()` method returns [buf10_data::R](buf10_data::R) reader structure"]
impl crate::Readable for BUF10_DATA {}
#[doc = "`write(|w| ..)` method takes [buf10_data::W](buf10_data::W) writer structure"]
impl crate::Writable for BUF10_DATA {}
#[doc = "Scan Results"]
pub mod buf10_data;
#[doc = "Scan Results\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf11_data](buf11_data) module"]
pub type BUF11_DATA = crate::Reg<u32, _BUF11_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF11_DATA;
#[doc = "`read()` method returns [buf11_data::R](buf11_data::R) reader structure"]
impl crate::Readable for BUF11_DATA {}
#[doc = "`write(|w| ..)` method takes [buf11_data::W](buf11_data::W) writer structure"]
impl crate::Writable for BUF11_DATA {}
#[doc = "Scan Results"]
pub mod buf11_data;
#[doc = "Scan Results\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf12_data](buf12_data) module"]
pub type BUF12_DATA = crate::Reg<u32, _BUF12_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF12_DATA;
#[doc = "`read()` method returns [buf12_data::R](buf12_data::R) reader structure"]
impl crate::Readable for BUF12_DATA {}
#[doc = "`write(|w| ..)` method takes [buf12_data::W](buf12_data::W) writer structure"]
impl crate::Writable for BUF12_DATA {}
#[doc = "Scan Results"]
pub mod buf12_data;
#[doc = "Scan Results\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf13_data](buf13_data) module"]
pub type BUF13_DATA = crate::Reg<u32, _BUF13_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF13_DATA;
#[doc = "`read()` method returns [buf13_data::R](buf13_data::R) reader structure"]
impl crate::Readable for BUF13_DATA {}
#[doc = "`write(|w| ..)` method takes [buf13_data::W](buf13_data::W) writer structure"]
impl crate::Writable for BUF13_DATA {}
#[doc = "Scan Results"]
pub mod buf13_data;
#[doc = "Scan Results\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf14_data](buf14_data) module"]
pub type BUF14_DATA = crate::Reg<u32, _BUF14_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF14_DATA;
#[doc = "`read()` method returns [buf14_data::R](buf14_data::R) reader structure"]
impl crate::Readable for BUF14_DATA {}
#[doc = "`write(|w| ..)` method takes [buf14_data::W](buf14_data::W) writer structure"]
impl crate::Writable for BUF14_DATA {}
#[doc = "Scan Results"]
pub mod buf14_data;
#[doc = "Scan Results\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf15_data](buf15_data) module"]
pub type BUF15_DATA = crate::Reg<u32, _BUF15_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF15_DATA;
#[doc = "`read()` method returns [buf15_data::R](buf15_data::R) reader structure"]
impl crate::Readable for BUF15_DATA {}
#[doc = "`write(|w| ..)` method takes [buf15_data::W](buf15_data::W) writer structure"]
impl crate::Writable for BUF15_DATA {}
#[doc = "Scan Results"]
pub mod buf15_data;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_timing](ch0_timing) module"]
pub type CH0_TIMING = crate::Reg<u32, _CH0_TIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_TIMING;
#[doc = "`read()` method returns [ch0_timing::R](ch0_timing::R) reader structure"]
impl crate::Readable for CH0_TIMING {}
#[doc = "`write(|w| ..)` method takes [ch0_timing::W](ch0_timing::W) writer structure"]
impl crate::Writable for CH0_TIMING {}
#[doc = "Scan Configuration"]
pub mod ch0_timing;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_interact](ch0_interact) module"]
pub type CH0_INTERACT = crate::Reg<u32, _CH0_INTERACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_INTERACT;
#[doc = "`read()` method returns [ch0_interact::R](ch0_interact::R) reader structure"]
impl crate::Readable for CH0_INTERACT {}
#[doc = "`write(|w| ..)` method takes [ch0_interact::W](ch0_interact::W) writer structure"]
impl crate::Writable for CH0_INTERACT {}
#[doc = "Scan Configuration"]
pub mod ch0_interact;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_eval](ch0_eval) module"]
pub type CH0_EVAL = crate::Reg<u32, _CH0_EVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_EVAL;
#[doc = "`read()` method returns [ch0_eval::R](ch0_eval::R) reader structure"]
impl crate::Readable for CH0_EVAL {}
#[doc = "`write(|w| ..)` method takes [ch0_eval::W](ch0_eval::W) writer structure"]
impl crate::Writable for CH0_EVAL {}
#[doc = "Scan Configuration"]
pub mod ch0_eval;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_timing](ch1_timing) module"]
pub type CH1_TIMING = crate::Reg<u32, _CH1_TIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_TIMING;
#[doc = "`read()` method returns [ch1_timing::R](ch1_timing::R) reader structure"]
impl crate::Readable for CH1_TIMING {}
#[doc = "`write(|w| ..)` method takes [ch1_timing::W](ch1_timing::W) writer structure"]
impl crate::Writable for CH1_TIMING {}
#[doc = "Scan Configuration"]
pub mod ch1_timing;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_interact](ch1_interact) module"]
pub type CH1_INTERACT = crate::Reg<u32, _CH1_INTERACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_INTERACT;
#[doc = "`read()` method returns [ch1_interact::R](ch1_interact::R) reader structure"]
impl crate::Readable for CH1_INTERACT {}
#[doc = "`write(|w| ..)` method takes [ch1_interact::W](ch1_interact::W) writer structure"]
impl crate::Writable for CH1_INTERACT {}
#[doc = "Scan Configuration"]
pub mod ch1_interact;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_eval](ch1_eval) module"]
pub type CH1_EVAL = crate::Reg<u32, _CH1_EVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_EVAL;
#[doc = "`read()` method returns [ch1_eval::R](ch1_eval::R) reader structure"]
impl crate::Readable for CH1_EVAL {}
#[doc = "`write(|w| ..)` method takes [ch1_eval::W](ch1_eval::W) writer structure"]
impl crate::Writable for CH1_EVAL {}
#[doc = "Scan Configuration"]
pub mod ch1_eval;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_timing](ch2_timing) module"]
pub type CH2_TIMING = crate::Reg<u32, _CH2_TIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_TIMING;
#[doc = "`read()` method returns [ch2_timing::R](ch2_timing::R) reader structure"]
impl crate::Readable for CH2_TIMING {}
#[doc = "`write(|w| ..)` method takes [ch2_timing::W](ch2_timing::W) writer structure"]
impl crate::Writable for CH2_TIMING {}
#[doc = "Scan Configuration"]
pub mod ch2_timing;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_interact](ch2_interact) module"]
pub type CH2_INTERACT = crate::Reg<u32, _CH2_INTERACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_INTERACT;
#[doc = "`read()` method returns [ch2_interact::R](ch2_interact::R) reader structure"]
impl crate::Readable for CH2_INTERACT {}
#[doc = "`write(|w| ..)` method takes [ch2_interact::W](ch2_interact::W) writer structure"]
impl crate::Writable for CH2_INTERACT {}
#[doc = "Scan Configuration"]
pub mod ch2_interact;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_eval](ch2_eval) module"]
pub type CH2_EVAL = crate::Reg<u32, _CH2_EVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_EVAL;
#[doc = "`read()` method returns [ch2_eval::R](ch2_eval::R) reader structure"]
impl crate::Readable for CH2_EVAL {}
#[doc = "`write(|w| ..)` method takes [ch2_eval::W](ch2_eval::W) writer structure"]
impl crate::Writable for CH2_EVAL {}
#[doc = "Scan Configuration"]
pub mod ch2_eval;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_timing](ch3_timing) module"]
pub type CH3_TIMING = crate::Reg<u32, _CH3_TIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_TIMING;
#[doc = "`read()` method returns [ch3_timing::R](ch3_timing::R) reader structure"]
impl crate::Readable for CH3_TIMING {}
#[doc = "`write(|w| ..)` method takes [ch3_timing::W](ch3_timing::W) writer structure"]
impl crate::Writable for CH3_TIMING {}
#[doc = "Scan Configuration"]
pub mod ch3_timing;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_interact](ch3_interact) module"]
pub type CH3_INTERACT = crate::Reg<u32, _CH3_INTERACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_INTERACT;
#[doc = "`read()` method returns [ch3_interact::R](ch3_interact::R) reader structure"]
impl crate::Readable for CH3_INTERACT {}
#[doc = "`write(|w| ..)` method takes [ch3_interact::W](ch3_interact::W) writer structure"]
impl crate::Writable for CH3_INTERACT {}
#[doc = "Scan Configuration"]
pub mod ch3_interact;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_eval](ch3_eval) module"]
pub type CH3_EVAL = crate::Reg<u32, _CH3_EVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_EVAL;
#[doc = "`read()` method returns [ch3_eval::R](ch3_eval::R) reader structure"]
impl crate::Readable for CH3_EVAL {}
#[doc = "`write(|w| ..)` method takes [ch3_eval::W](ch3_eval::W) writer structure"]
impl crate::Writable for CH3_EVAL {}
#[doc = "Scan Configuration"]
pub mod ch3_eval;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_timing](ch4_timing) module"]
pub type CH4_TIMING = crate::Reg<u32, _CH4_TIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_TIMING;
#[doc = "`read()` method returns [ch4_timing::R](ch4_timing::R) reader structure"]
impl crate::Readable for CH4_TIMING {}
#[doc = "`write(|w| ..)` method takes [ch4_timing::W](ch4_timing::W) writer structure"]
impl crate::Writable for CH4_TIMING {}
#[doc = "Scan Configuration"]
pub mod ch4_timing;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_interact](ch4_interact) module"]
pub type CH4_INTERACT = crate::Reg<u32, _CH4_INTERACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_INTERACT;
#[doc = "`read()` method returns [ch4_interact::R](ch4_interact::R) reader structure"]
impl crate::Readable for CH4_INTERACT {}
#[doc = "`write(|w| ..)` method takes [ch4_interact::W](ch4_interact::W) writer structure"]
impl crate::Writable for CH4_INTERACT {}
#[doc = "Scan Configuration"]
pub mod ch4_interact;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_eval](ch4_eval) module"]
pub type CH4_EVAL = crate::Reg<u32, _CH4_EVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_EVAL;
#[doc = "`read()` method returns [ch4_eval::R](ch4_eval::R) reader structure"]
impl crate::Readable for CH4_EVAL {}
#[doc = "`write(|w| ..)` method takes [ch4_eval::W](ch4_eval::W) writer structure"]
impl crate::Writable for CH4_EVAL {}
#[doc = "Scan Configuration"]
pub mod ch4_eval;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_timing](ch5_timing) module"]
pub type CH5_TIMING = crate::Reg<u32, _CH5_TIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_TIMING;
#[doc = "`read()` method returns [ch5_timing::R](ch5_timing::R) reader structure"]
impl crate::Readable for CH5_TIMING {}
#[doc = "`write(|w| ..)` method takes [ch5_timing::W](ch5_timing::W) writer structure"]
impl crate::Writable for CH5_TIMING {}
#[doc = "Scan Configuration"]
pub mod ch5_timing;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_interact](ch5_interact) module"]
pub type CH5_INTERACT = crate::Reg<u32, _CH5_INTERACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_INTERACT;
#[doc = "`read()` method returns [ch5_interact::R](ch5_interact::R) reader structure"]
impl crate::Readable for CH5_INTERACT {}
#[doc = "`write(|w| ..)` method takes [ch5_interact::W](ch5_interact::W) writer structure"]
impl crate::Writable for CH5_INTERACT {}
#[doc = "Scan Configuration"]
pub mod ch5_interact;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_eval](ch5_eval) module"]
pub type CH5_EVAL = crate::Reg<u32, _CH5_EVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_EVAL;
#[doc = "`read()` method returns [ch5_eval::R](ch5_eval::R) reader structure"]
impl crate::Readable for CH5_EVAL {}
#[doc = "`write(|w| ..)` method takes [ch5_eval::W](ch5_eval::W) writer structure"]
impl crate::Writable for CH5_EVAL {}
#[doc = "Scan Configuration"]
pub mod ch5_eval;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_timing](ch6_timing) module"]
pub type CH6_TIMING = crate::Reg<u32, _CH6_TIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_TIMING;
#[doc = "`read()` method returns [ch6_timing::R](ch6_timing::R) reader structure"]
impl crate::Readable for CH6_TIMING {}
#[doc = "`write(|w| ..)` method takes [ch6_timing::W](ch6_timing::W) writer structure"]
impl crate::Writable for CH6_TIMING {}
#[doc = "Scan Configuration"]
pub mod ch6_timing;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_interact](ch6_interact) module"]
pub type CH6_INTERACT = crate::Reg<u32, _CH6_INTERACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_INTERACT;
#[doc = "`read()` method returns [ch6_interact::R](ch6_interact::R) reader structure"]
impl crate::Readable for CH6_INTERACT {}
#[doc = "`write(|w| ..)` method takes [ch6_interact::W](ch6_interact::W) writer structure"]
impl crate::Writable for CH6_INTERACT {}
#[doc = "Scan Configuration"]
pub mod ch6_interact;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_eval](ch6_eval) module"]
pub type CH6_EVAL = crate::Reg<u32, _CH6_EVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_EVAL;
#[doc = "`read()` method returns [ch6_eval::R](ch6_eval::R) reader structure"]
impl crate::Readable for CH6_EVAL {}
#[doc = "`write(|w| ..)` method takes [ch6_eval::W](ch6_eval::W) writer structure"]
impl crate::Writable for CH6_EVAL {}
#[doc = "Scan Configuration"]
pub mod ch6_eval;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_timing](ch7_timing) module"]
pub type CH7_TIMING = crate::Reg<u32, _CH7_TIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_TIMING;
#[doc = "`read()` method returns [ch7_timing::R](ch7_timing::R) reader structure"]
impl crate::Readable for CH7_TIMING {}
#[doc = "`write(|w| ..)` method takes [ch7_timing::W](ch7_timing::W) writer structure"]
impl crate::Writable for CH7_TIMING {}
#[doc = "Scan Configuration"]
pub mod ch7_timing;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_interact](ch7_interact) module"]
pub type CH7_INTERACT = crate::Reg<u32, _CH7_INTERACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_INTERACT;
#[doc = "`read()` method returns [ch7_interact::R](ch7_interact::R) reader structure"]
impl crate::Readable for CH7_INTERACT {}
#[doc = "`write(|w| ..)` method takes [ch7_interact::W](ch7_interact::W) writer structure"]
impl crate::Writable for CH7_INTERACT {}
#[doc = "Scan Configuration"]
pub mod ch7_interact;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_eval](ch7_eval) module"]
pub type CH7_EVAL = crate::Reg<u32, _CH7_EVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_EVAL;
#[doc = "`read()` method returns [ch7_eval::R](ch7_eval::R) reader structure"]
impl crate::Readable for CH7_EVAL {}
#[doc = "`write(|w| ..)` method takes [ch7_eval::W](ch7_eval::W) writer structure"]
impl crate::Writable for CH7_EVAL {}
#[doc = "Scan Configuration"]
pub mod ch7_eval;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_timing](ch8_timing) module"]
pub type CH8_TIMING = crate::Reg<u32, _CH8_TIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_TIMING;
#[doc = "`read()` method returns [ch8_timing::R](ch8_timing::R) reader structure"]
impl crate::Readable for CH8_TIMING {}
#[doc = "`write(|w| ..)` method takes [ch8_timing::W](ch8_timing::W) writer structure"]
impl crate::Writable for CH8_TIMING {}
#[doc = "Scan Configuration"]
pub mod ch8_timing;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_interact](ch8_interact) module"]
pub type CH8_INTERACT = crate::Reg<u32, _CH8_INTERACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_INTERACT;
#[doc = "`read()` method returns [ch8_interact::R](ch8_interact::R) reader structure"]
impl crate::Readable for CH8_INTERACT {}
#[doc = "`write(|w| ..)` method takes [ch8_interact::W](ch8_interact::W) writer structure"]
impl crate::Writable for CH8_INTERACT {}
#[doc = "Scan Configuration"]
pub mod ch8_interact;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_eval](ch8_eval) module"]
pub type CH8_EVAL = crate::Reg<u32, _CH8_EVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_EVAL;
#[doc = "`read()` method returns [ch8_eval::R](ch8_eval::R) reader structure"]
impl crate::Readable for CH8_EVAL {}
#[doc = "`write(|w| ..)` method takes [ch8_eval::W](ch8_eval::W) writer structure"]
impl crate::Writable for CH8_EVAL {}
#[doc = "Scan Configuration"]
pub mod ch8_eval;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_timing](ch9_timing) module"]
pub type CH9_TIMING = crate::Reg<u32, _CH9_TIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_TIMING;
#[doc = "`read()` method returns [ch9_timing::R](ch9_timing::R) reader structure"]
impl crate::Readable for CH9_TIMING {}
#[doc = "`write(|w| ..)` method takes [ch9_timing::W](ch9_timing::W) writer structure"]
impl crate::Writable for CH9_TIMING {}
#[doc = "Scan Configuration"]
pub mod ch9_timing;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_interact](ch9_interact) module"]
pub type CH9_INTERACT = crate::Reg<u32, _CH9_INTERACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_INTERACT;
#[doc = "`read()` method returns [ch9_interact::R](ch9_interact::R) reader structure"]
impl crate::Readable for CH9_INTERACT {}
#[doc = "`write(|w| ..)` method takes [ch9_interact::W](ch9_interact::W) writer structure"]
impl crate::Writable for CH9_INTERACT {}
#[doc = "Scan Configuration"]
pub mod ch9_interact;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_eval](ch9_eval) module"]
pub type CH9_EVAL = crate::Reg<u32, _CH9_EVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_EVAL;
#[doc = "`read()` method returns [ch9_eval::R](ch9_eval::R) reader structure"]
impl crate::Readable for CH9_EVAL {}
#[doc = "`write(|w| ..)` method takes [ch9_eval::W](ch9_eval::W) writer structure"]
impl crate::Writable for CH9_EVAL {}
#[doc = "Scan Configuration"]
pub mod ch9_eval;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_timing](ch10_timing) module"]
pub type CH10_TIMING = crate::Reg<u32, _CH10_TIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_TIMING;
#[doc = "`read()` method returns [ch10_timing::R](ch10_timing::R) reader structure"]
impl crate::Readable for CH10_TIMING {}
#[doc = "`write(|w| ..)` method takes [ch10_timing::W](ch10_timing::W) writer structure"]
impl crate::Writable for CH10_TIMING {}
#[doc = "Scan Configuration"]
pub mod ch10_timing;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_interact](ch10_interact) module"]
pub type CH10_INTERACT = crate::Reg<u32, _CH10_INTERACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_INTERACT;
#[doc = "`read()` method returns [ch10_interact::R](ch10_interact::R) reader structure"]
impl crate::Readable for CH10_INTERACT {}
#[doc = "`write(|w| ..)` method takes [ch10_interact::W](ch10_interact::W) writer structure"]
impl crate::Writable for CH10_INTERACT {}
#[doc = "Scan Configuration"]
pub mod ch10_interact;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_eval](ch10_eval) module"]
pub type CH10_EVAL = crate::Reg<u32, _CH10_EVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_EVAL;
#[doc = "`read()` method returns [ch10_eval::R](ch10_eval::R) reader structure"]
impl crate::Readable for CH10_EVAL {}
#[doc = "`write(|w| ..)` method takes [ch10_eval::W](ch10_eval::W) writer structure"]
impl crate::Writable for CH10_EVAL {}
#[doc = "Scan Configuration"]
pub mod ch10_eval;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_timing](ch11_timing) module"]
pub type CH11_TIMING = crate::Reg<u32, _CH11_TIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_TIMING;
#[doc = "`read()` method returns [ch11_timing::R](ch11_timing::R) reader structure"]
impl crate::Readable for CH11_TIMING {}
#[doc = "`write(|w| ..)` method takes [ch11_timing::W](ch11_timing::W) writer structure"]
impl crate::Writable for CH11_TIMING {}
#[doc = "Scan Configuration"]
pub mod ch11_timing;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_interact](ch11_interact) module"]
pub type CH11_INTERACT = crate::Reg<u32, _CH11_INTERACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_INTERACT;
#[doc = "`read()` method returns [ch11_interact::R](ch11_interact::R) reader structure"]
impl crate::Readable for CH11_INTERACT {}
#[doc = "`write(|w| ..)` method takes [ch11_interact::W](ch11_interact::W) writer structure"]
impl crate::Writable for CH11_INTERACT {}
#[doc = "Scan Configuration"]
pub mod ch11_interact;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_eval](ch11_eval) module"]
pub type CH11_EVAL = crate::Reg<u32, _CH11_EVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_EVAL;
#[doc = "`read()` method returns [ch11_eval::R](ch11_eval::R) reader structure"]
impl crate::Readable for CH11_EVAL {}
#[doc = "`write(|w| ..)` method takes [ch11_eval::W](ch11_eval::W) writer structure"]
impl crate::Writable for CH11_EVAL {}
#[doc = "Scan Configuration"]
pub mod ch11_eval;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch12_timing](ch12_timing) module"]
pub type CH12_TIMING = crate::Reg<u32, _CH12_TIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH12_TIMING;
#[doc = "`read()` method returns [ch12_timing::R](ch12_timing::R) reader structure"]
impl crate::Readable for CH12_TIMING {}
#[doc = "`write(|w| ..)` method takes [ch12_timing::W](ch12_timing::W) writer structure"]
impl crate::Writable for CH12_TIMING {}
#[doc = "Scan Configuration"]
pub mod ch12_timing;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch12_interact](ch12_interact) module"]
pub type CH12_INTERACT = crate::Reg<u32, _CH12_INTERACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH12_INTERACT;
#[doc = "`read()` method returns [ch12_interact::R](ch12_interact::R) reader structure"]
impl crate::Readable for CH12_INTERACT {}
#[doc = "`write(|w| ..)` method takes [ch12_interact::W](ch12_interact::W) writer structure"]
impl crate::Writable for CH12_INTERACT {}
#[doc = "Scan Configuration"]
pub mod ch12_interact;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch12_eval](ch12_eval) module"]
pub type CH12_EVAL = crate::Reg<u32, _CH12_EVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH12_EVAL;
#[doc = "`read()` method returns [ch12_eval::R](ch12_eval::R) reader structure"]
impl crate::Readable for CH12_EVAL {}
#[doc = "`write(|w| ..)` method takes [ch12_eval::W](ch12_eval::W) writer structure"]
impl crate::Writable for CH12_EVAL {}
#[doc = "Scan Configuration"]
pub mod ch12_eval;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch13_timing](ch13_timing) module"]
pub type CH13_TIMING = crate::Reg<u32, _CH13_TIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH13_TIMING;
#[doc = "`read()` method returns [ch13_timing::R](ch13_timing::R) reader structure"]
impl crate::Readable for CH13_TIMING {}
#[doc = "`write(|w| ..)` method takes [ch13_timing::W](ch13_timing::W) writer structure"]
impl crate::Writable for CH13_TIMING {}
#[doc = "Scan Configuration"]
pub mod ch13_timing;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch13_interact](ch13_interact) module"]
pub type CH13_INTERACT = crate::Reg<u32, _CH13_INTERACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH13_INTERACT;
#[doc = "`read()` method returns [ch13_interact::R](ch13_interact::R) reader structure"]
impl crate::Readable for CH13_INTERACT {}
#[doc = "`write(|w| ..)` method takes [ch13_interact::W](ch13_interact::W) writer structure"]
impl crate::Writable for CH13_INTERACT {}
#[doc = "Scan Configuration"]
pub mod ch13_interact;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch13_eval](ch13_eval) module"]
pub type CH13_EVAL = crate::Reg<u32, _CH13_EVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH13_EVAL;
#[doc = "`read()` method returns [ch13_eval::R](ch13_eval::R) reader structure"]
impl crate::Readable for CH13_EVAL {}
#[doc = "`write(|w| ..)` method takes [ch13_eval::W](ch13_eval::W) writer structure"]
impl crate::Writable for CH13_EVAL {}
#[doc = "Scan Configuration"]
pub mod ch13_eval;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch14_timing](ch14_timing) module"]
pub type CH14_TIMING = crate::Reg<u32, _CH14_TIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH14_TIMING;
#[doc = "`read()` method returns [ch14_timing::R](ch14_timing::R) reader structure"]
impl crate::Readable for CH14_TIMING {}
#[doc = "`write(|w| ..)` method takes [ch14_timing::W](ch14_timing::W) writer structure"]
impl crate::Writable for CH14_TIMING {}
#[doc = "Scan Configuration"]
pub mod ch14_timing;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch14_interact](ch14_interact) module"]
pub type CH14_INTERACT = crate::Reg<u32, _CH14_INTERACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH14_INTERACT;
#[doc = "`read()` method returns [ch14_interact::R](ch14_interact::R) reader structure"]
impl crate::Readable for CH14_INTERACT {}
#[doc = "`write(|w| ..)` method takes [ch14_interact::W](ch14_interact::W) writer structure"]
impl crate::Writable for CH14_INTERACT {}
#[doc = "Scan Configuration"]
pub mod ch14_interact;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch14_eval](ch14_eval) module"]
pub type CH14_EVAL = crate::Reg<u32, _CH14_EVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH14_EVAL;
#[doc = "`read()` method returns [ch14_eval::R](ch14_eval::R) reader structure"]
impl crate::Readable for CH14_EVAL {}
#[doc = "`write(|w| ..)` method takes [ch14_eval::W](ch14_eval::W) writer structure"]
impl crate::Writable for CH14_EVAL {}
#[doc = "Scan Configuration"]
pub mod ch14_eval;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch15_timing](ch15_timing) module"]
pub type CH15_TIMING = crate::Reg<u32, _CH15_TIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH15_TIMING;
#[doc = "`read()` method returns [ch15_timing::R](ch15_timing::R) reader structure"]
impl crate::Readable for CH15_TIMING {}
#[doc = "`write(|w| ..)` method takes [ch15_timing::W](ch15_timing::W) writer structure"]
impl crate::Writable for CH15_TIMING {}
#[doc = "Scan Configuration"]
pub mod ch15_timing;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch15_interact](ch15_interact) module"]
pub type CH15_INTERACT = crate::Reg<u32, _CH15_INTERACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH15_INTERACT;
#[doc = "`read()` method returns [ch15_interact::R](ch15_interact::R) reader structure"]
impl crate::Readable for CH15_INTERACT {}
#[doc = "`write(|w| ..)` method takes [ch15_interact::W](ch15_interact::W) writer structure"]
impl crate::Writable for CH15_INTERACT {}
#[doc = "Scan Configuration"]
pub mod ch15_interact;
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch15_eval](ch15_eval) module"]
pub type CH15_EVAL = crate::Reg<u32, _CH15_EVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH15_EVAL;
#[doc = "`read()` method returns [ch15_eval::R](ch15_eval::R) reader structure"]
impl crate::Readable for CH15_EVAL {}
#[doc = "`write(|w| ..)` method takes [ch15_eval::W](ch15_eval::W) writer structure"]
impl crate::Writable for CH15_EVAL {}
#[doc = "Scan Configuration"]
pub mod ch15_eval;
