#[doc = r"Register block"]
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
    _reserved13: [u8; 108usize],
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
    _reserved20: [u8; 4usize],
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
    _reserved27: [u8; 4usize],
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
    _reserved34: [u8; 4usize],
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
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status Register"]
pub mod status;
#[doc = "Channel 0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0ctrl](ch0ctrl) module"]
pub type CH0CTRL = crate::Reg<u32, _CH0CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0CTRL;
#[doc = "`read()` method returns [ch0ctrl::R](ch0ctrl::R) reader structure"]
impl crate::Readable for CH0CTRL {}
#[doc = "`write(|w| ..)` method takes [ch0ctrl::W](ch0ctrl::W) writer structure"]
impl crate::Writable for CH0CTRL {}
#[doc = "Channel 0 Control Register"]
pub mod ch0ctrl;
#[doc = "Channel 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1ctrl](ch1ctrl) module"]
pub type CH1CTRL = crate::Reg<u32, _CH1CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1CTRL;
#[doc = "`read()` method returns [ch1ctrl::R](ch1ctrl::R) reader structure"]
impl crate::Readable for CH1CTRL {}
#[doc = "`write(|w| ..)` method takes [ch1ctrl::W](ch1ctrl::W) writer structure"]
impl crate::Writable for CH1CTRL {}
#[doc = "Channel 1 Control Register"]
pub mod ch1ctrl;
#[doc = "Command Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Command Register"]
pub mod cmd;
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
#[doc = "Channel 0 Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0data](ch0data) module"]
pub type CH0DATA = crate::Reg<u32, _CH0DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0DATA;
#[doc = "`read()` method returns [ch0data::R](ch0data::R) reader structure"]
impl crate::Readable for CH0DATA {}
#[doc = "`write(|w| ..)` method takes [ch0data::W](ch0data::W) writer structure"]
impl crate::Writable for CH0DATA {}
#[doc = "Channel 0 Data Register"]
pub mod ch0data;
#[doc = "Channel 1 Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1data](ch1data) module"]
pub type CH1DATA = crate::Reg<u32, _CH1DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1DATA;
#[doc = "`read()` method returns [ch1data::R](ch1data::R) reader structure"]
impl crate::Readable for CH1DATA {}
#[doc = "`write(|w| ..)` method takes [ch1data::W](ch1data::W) writer structure"]
impl crate::Writable for CH1DATA {}
#[doc = "Channel 1 Data Register"]
pub mod ch1data;
#[doc = "Combined Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [combdata](combdata) module"]
pub type COMBDATA = crate::Reg<u32, _COMBDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMBDATA;
#[doc = "`read()` method returns [combdata::R](combdata::R) reader structure"]
impl crate::Readable for COMBDATA {}
#[doc = "`write(|w| ..)` method takes [combdata::W](combdata::W) writer structure"]
impl crate::Writable for COMBDATA {}
#[doc = "Combined Data Register"]
pub mod combdata;
#[doc = "Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal](cal) module"]
pub type CAL = crate::Reg<u32, _CAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAL;
#[doc = "`read()` method returns [cal::R](cal::R) reader structure"]
impl crate::Readable for CAL {}
#[doc = "`write(|w| ..)` method takes [cal::W](cal::W) writer structure"]
impl crate::Writable for CAL {}
#[doc = "Calibration Register"]
pub mod cal;
#[doc = "Operational Amplifier APORT Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa0_aportreq](opa0_aportreq) module"]
pub type OPA0_APORTREQ = crate::Reg<u32, _OPA0_APORTREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA0_APORTREQ;
#[doc = "`read()` method returns [opa0_aportreq::R](opa0_aportreq::R) reader structure"]
impl crate::Readable for OPA0_APORTREQ {}
#[doc = "Operational Amplifier APORT Request Status Register"]
pub mod opa0_aportreq;
#[doc = "Operational Amplifier APORT Conflict Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa0_aportconflict](opa0_aportconflict) module"]
pub type OPA0_APORTCONFLICT = crate::Reg<u32, _OPA0_APORTCONFLICT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA0_APORTCONFLICT;
#[doc = "`read()` method returns [opa0_aportconflict::R](opa0_aportconflict::R) reader structure"]
impl crate::Readable for OPA0_APORTCONFLICT {}
#[doc = "Operational Amplifier APORT Conflict Status Register"]
pub mod opa0_aportconflict;
#[doc = "Operational Amplifier Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa0_ctrl](opa0_ctrl) module"]
pub type OPA0_CTRL = crate::Reg<u32, _OPA0_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA0_CTRL;
#[doc = "`read()` method returns [opa0_ctrl::R](opa0_ctrl::R) reader structure"]
impl crate::Readable for OPA0_CTRL {}
#[doc = "`write(|w| ..)` method takes [opa0_ctrl::W](opa0_ctrl::W) writer structure"]
impl crate::Writable for OPA0_CTRL {}
#[doc = "Operational Amplifier Control Register"]
pub mod opa0_ctrl;
#[doc = "Operational Amplifier Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa0_timer](opa0_timer) module"]
pub type OPA0_TIMER = crate::Reg<u32, _OPA0_TIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA0_TIMER;
#[doc = "`read()` method returns [opa0_timer::R](opa0_timer::R) reader structure"]
impl crate::Readable for OPA0_TIMER {}
#[doc = "`write(|w| ..)` method takes [opa0_timer::W](opa0_timer::W) writer structure"]
impl crate::Writable for OPA0_TIMER {}
#[doc = "Operational Amplifier Timer Control Register"]
pub mod opa0_timer;
#[doc = "Operational Amplifier Mux Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa0_mux](opa0_mux) module"]
pub type OPA0_MUX = crate::Reg<u32, _OPA0_MUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA0_MUX;
#[doc = "`read()` method returns [opa0_mux::R](opa0_mux::R) reader structure"]
impl crate::Readable for OPA0_MUX {}
#[doc = "`write(|w| ..)` method takes [opa0_mux::W](opa0_mux::W) writer structure"]
impl crate::Writable for OPA0_MUX {}
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa0_mux;
#[doc = "Operational Amplifier Output Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa0_out](opa0_out) module"]
pub type OPA0_OUT = crate::Reg<u32, _OPA0_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA0_OUT;
#[doc = "`read()` method returns [opa0_out::R](opa0_out::R) reader structure"]
impl crate::Readable for OPA0_OUT {}
#[doc = "`write(|w| ..)` method takes [opa0_out::W](opa0_out::W) writer structure"]
impl crate::Writable for OPA0_OUT {}
#[doc = "Operational Amplifier Output Configuration Register"]
pub mod opa0_out;
#[doc = "Operational Amplifier Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa0_cal](opa0_cal) module"]
pub type OPA0_CAL = crate::Reg<u32, _OPA0_CAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA0_CAL;
#[doc = "`read()` method returns [opa0_cal::R](opa0_cal::R) reader structure"]
impl crate::Readable for OPA0_CAL {}
#[doc = "`write(|w| ..)` method takes [opa0_cal::W](opa0_cal::W) writer structure"]
impl crate::Writable for OPA0_CAL {}
#[doc = "Operational Amplifier Calibration Register"]
pub mod opa0_cal;
#[doc = "Operational Amplifier APORT Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa1_aportreq](opa1_aportreq) module"]
pub type OPA1_APORTREQ = crate::Reg<u32, _OPA1_APORTREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA1_APORTREQ;
#[doc = "`read()` method returns [opa1_aportreq::R](opa1_aportreq::R) reader structure"]
impl crate::Readable for OPA1_APORTREQ {}
#[doc = "Operational Amplifier APORT Request Status Register"]
pub mod opa1_aportreq;
#[doc = "Operational Amplifier APORT Conflict Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa1_aportconflict](opa1_aportconflict) module"]
pub type OPA1_APORTCONFLICT = crate::Reg<u32, _OPA1_APORTCONFLICT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA1_APORTCONFLICT;
#[doc = "`read()` method returns [opa1_aportconflict::R](opa1_aportconflict::R) reader structure"]
impl crate::Readable for OPA1_APORTCONFLICT {}
#[doc = "Operational Amplifier APORT Conflict Status Register"]
pub mod opa1_aportconflict;
#[doc = "Operational Amplifier Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa1_ctrl](opa1_ctrl) module"]
pub type OPA1_CTRL = crate::Reg<u32, _OPA1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA1_CTRL;
#[doc = "`read()` method returns [opa1_ctrl::R](opa1_ctrl::R) reader structure"]
impl crate::Readable for OPA1_CTRL {}
#[doc = "`write(|w| ..)` method takes [opa1_ctrl::W](opa1_ctrl::W) writer structure"]
impl crate::Writable for OPA1_CTRL {}
#[doc = "Operational Amplifier Control Register"]
pub mod opa1_ctrl;
#[doc = "Operational Amplifier Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa1_timer](opa1_timer) module"]
pub type OPA1_TIMER = crate::Reg<u32, _OPA1_TIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA1_TIMER;
#[doc = "`read()` method returns [opa1_timer::R](opa1_timer::R) reader structure"]
impl crate::Readable for OPA1_TIMER {}
#[doc = "`write(|w| ..)` method takes [opa1_timer::W](opa1_timer::W) writer structure"]
impl crate::Writable for OPA1_TIMER {}
#[doc = "Operational Amplifier Timer Control Register"]
pub mod opa1_timer;
#[doc = "Operational Amplifier Mux Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa1_mux](opa1_mux) module"]
pub type OPA1_MUX = crate::Reg<u32, _OPA1_MUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA1_MUX;
#[doc = "`read()` method returns [opa1_mux::R](opa1_mux::R) reader structure"]
impl crate::Readable for OPA1_MUX {}
#[doc = "`write(|w| ..)` method takes [opa1_mux::W](opa1_mux::W) writer structure"]
impl crate::Writable for OPA1_MUX {}
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa1_mux;
#[doc = "Operational Amplifier Output Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa1_out](opa1_out) module"]
pub type OPA1_OUT = crate::Reg<u32, _OPA1_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA1_OUT;
#[doc = "`read()` method returns [opa1_out::R](opa1_out::R) reader structure"]
impl crate::Readable for OPA1_OUT {}
#[doc = "`write(|w| ..)` method takes [opa1_out::W](opa1_out::W) writer structure"]
impl crate::Writable for OPA1_OUT {}
#[doc = "Operational Amplifier Output Configuration Register"]
pub mod opa1_out;
#[doc = "Operational Amplifier Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa1_cal](opa1_cal) module"]
pub type OPA1_CAL = crate::Reg<u32, _OPA1_CAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA1_CAL;
#[doc = "`read()` method returns [opa1_cal::R](opa1_cal::R) reader structure"]
impl crate::Readable for OPA1_CAL {}
#[doc = "`write(|w| ..)` method takes [opa1_cal::W](opa1_cal::W) writer structure"]
impl crate::Writable for OPA1_CAL {}
#[doc = "Operational Amplifier Calibration Register"]
pub mod opa1_cal;
#[doc = "Operational Amplifier APORT Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa2_aportreq](opa2_aportreq) module"]
pub type OPA2_APORTREQ = crate::Reg<u32, _OPA2_APORTREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA2_APORTREQ;
#[doc = "`read()` method returns [opa2_aportreq::R](opa2_aportreq::R) reader structure"]
impl crate::Readable for OPA2_APORTREQ {}
#[doc = "Operational Amplifier APORT Request Status Register"]
pub mod opa2_aportreq;
#[doc = "Operational Amplifier APORT Conflict Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa2_aportconflict](opa2_aportconflict) module"]
pub type OPA2_APORTCONFLICT = crate::Reg<u32, _OPA2_APORTCONFLICT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA2_APORTCONFLICT;
#[doc = "`read()` method returns [opa2_aportconflict::R](opa2_aportconflict::R) reader structure"]
impl crate::Readable for OPA2_APORTCONFLICT {}
#[doc = "Operational Amplifier APORT Conflict Status Register"]
pub mod opa2_aportconflict;
#[doc = "Operational Amplifier Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa2_ctrl](opa2_ctrl) module"]
pub type OPA2_CTRL = crate::Reg<u32, _OPA2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA2_CTRL;
#[doc = "`read()` method returns [opa2_ctrl::R](opa2_ctrl::R) reader structure"]
impl crate::Readable for OPA2_CTRL {}
#[doc = "`write(|w| ..)` method takes [opa2_ctrl::W](opa2_ctrl::W) writer structure"]
impl crate::Writable for OPA2_CTRL {}
#[doc = "Operational Amplifier Control Register"]
pub mod opa2_ctrl;
#[doc = "Operational Amplifier Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa2_timer](opa2_timer) module"]
pub type OPA2_TIMER = crate::Reg<u32, _OPA2_TIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA2_TIMER;
#[doc = "`read()` method returns [opa2_timer::R](opa2_timer::R) reader structure"]
impl crate::Readable for OPA2_TIMER {}
#[doc = "`write(|w| ..)` method takes [opa2_timer::W](opa2_timer::W) writer structure"]
impl crate::Writable for OPA2_TIMER {}
#[doc = "Operational Amplifier Timer Control Register"]
pub mod opa2_timer;
#[doc = "Operational Amplifier Mux Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa2_mux](opa2_mux) module"]
pub type OPA2_MUX = crate::Reg<u32, _OPA2_MUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA2_MUX;
#[doc = "`read()` method returns [opa2_mux::R](opa2_mux::R) reader structure"]
impl crate::Readable for OPA2_MUX {}
#[doc = "`write(|w| ..)` method takes [opa2_mux::W](opa2_mux::W) writer structure"]
impl crate::Writable for OPA2_MUX {}
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa2_mux;
#[doc = "Operational Amplifier Output Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa2_out](opa2_out) module"]
pub type OPA2_OUT = crate::Reg<u32, _OPA2_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA2_OUT;
#[doc = "`read()` method returns [opa2_out::R](opa2_out::R) reader structure"]
impl crate::Readable for OPA2_OUT {}
#[doc = "`write(|w| ..)` method takes [opa2_out::W](opa2_out::W) writer structure"]
impl crate::Writable for OPA2_OUT {}
#[doc = "Operational Amplifier Output Configuration Register"]
pub mod opa2_out;
#[doc = "Operational Amplifier Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa2_cal](opa2_cal) module"]
pub type OPA2_CAL = crate::Reg<u32, _OPA2_CAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA2_CAL;
#[doc = "`read()` method returns [opa2_cal::R](opa2_cal::R) reader structure"]
impl crate::Readable for OPA2_CAL {}
#[doc = "`write(|w| ..)` method takes [opa2_cal::W](opa2_cal::W) writer structure"]
impl crate::Writable for OPA2_CAL {}
#[doc = "Operational Amplifier Calibration Register"]
pub mod opa2_cal;
#[doc = "Operational Amplifier APORT Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa3_aportreq](opa3_aportreq) module"]
pub type OPA3_APORTREQ = crate::Reg<u32, _OPA3_APORTREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA3_APORTREQ;
#[doc = "`read()` method returns [opa3_aportreq::R](opa3_aportreq::R) reader structure"]
impl crate::Readable for OPA3_APORTREQ {}
#[doc = "Operational Amplifier APORT Request Status Register"]
pub mod opa3_aportreq;
#[doc = "Operational Amplifier APORT Conflict Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa3_aportconflict](opa3_aportconflict) module"]
pub type OPA3_APORTCONFLICT = crate::Reg<u32, _OPA3_APORTCONFLICT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA3_APORTCONFLICT;
#[doc = "`read()` method returns [opa3_aportconflict::R](opa3_aportconflict::R) reader structure"]
impl crate::Readable for OPA3_APORTCONFLICT {}
#[doc = "Operational Amplifier APORT Conflict Status Register"]
pub mod opa3_aportconflict;
#[doc = "Operational Amplifier Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa3_ctrl](opa3_ctrl) module"]
pub type OPA3_CTRL = crate::Reg<u32, _OPA3_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA3_CTRL;
#[doc = "`read()` method returns [opa3_ctrl::R](opa3_ctrl::R) reader structure"]
impl crate::Readable for OPA3_CTRL {}
#[doc = "`write(|w| ..)` method takes [opa3_ctrl::W](opa3_ctrl::W) writer structure"]
impl crate::Writable for OPA3_CTRL {}
#[doc = "Operational Amplifier Control Register"]
pub mod opa3_ctrl;
#[doc = "Operational Amplifier Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa3_timer](opa3_timer) module"]
pub type OPA3_TIMER = crate::Reg<u32, _OPA3_TIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA3_TIMER;
#[doc = "`read()` method returns [opa3_timer::R](opa3_timer::R) reader structure"]
impl crate::Readable for OPA3_TIMER {}
#[doc = "`write(|w| ..)` method takes [opa3_timer::W](opa3_timer::W) writer structure"]
impl crate::Writable for OPA3_TIMER {}
#[doc = "Operational Amplifier Timer Control Register"]
pub mod opa3_timer;
#[doc = "Operational Amplifier Mux Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa3_mux](opa3_mux) module"]
pub type OPA3_MUX = crate::Reg<u32, _OPA3_MUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA3_MUX;
#[doc = "`read()` method returns [opa3_mux::R](opa3_mux::R) reader structure"]
impl crate::Readable for OPA3_MUX {}
#[doc = "`write(|w| ..)` method takes [opa3_mux::W](opa3_mux::W) writer structure"]
impl crate::Writable for OPA3_MUX {}
#[doc = "Operational Amplifier Mux Configuration Register"]
pub mod opa3_mux;
#[doc = "Operational Amplifier Output Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa3_out](opa3_out) module"]
pub type OPA3_OUT = crate::Reg<u32, _OPA3_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA3_OUT;
#[doc = "`read()` method returns [opa3_out::R](opa3_out::R) reader structure"]
impl crate::Readable for OPA3_OUT {}
#[doc = "`write(|w| ..)` method takes [opa3_out::W](opa3_out::W) writer structure"]
impl crate::Writable for OPA3_OUT {}
#[doc = "Operational Amplifier Output Configuration Register"]
pub mod opa3_out;
#[doc = "Operational Amplifier Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa3_cal](opa3_cal) module"]
pub type OPA3_CAL = crate::Reg<u32, _OPA3_CAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPA3_CAL;
#[doc = "`read()` method returns [opa3_cal::R](opa3_cal::R) reader structure"]
impl crate::Readable for OPA3_CAL {}
#[doc = "`write(|w| ..)` method takes [opa3_cal::W](opa3_cal::W) writer structure"]
impl crate::Writable for OPA3_CAL {}
#[doc = "Operational Amplifier Calibration Register"]
pub mod opa3_cal;
