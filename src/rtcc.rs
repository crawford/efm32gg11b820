#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Pre-Counter Value Register"]
    pub precnt: PRECNT,
    #[doc = "0x08 - Counter Value Register"]
    pub cnt: CNT,
    #[doc = "0x0c - Combined Pre-Counter and Counter Value Register"]
    pub combcnt: COMBCNT,
    #[doc = "0x10 - Time of Day Register"]
    pub time: TIME,
    #[doc = "0x14 - Date Register"]
    pub date: DATE,
    #[doc = "0x18 - RTCC Interrupt Flags"]
    pub if_: IF,
    #[doc = "0x1c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x20 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x24 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x28 - Status Register"]
    pub status: STATUS,
    #[doc = "0x2c - Command Register"]
    pub cmd: CMD,
    #[doc = "0x30 - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x34 - Retention RAM Power-down Register"]
    pub powerdown: POWERDOWN,
    #[doc = "0x38 - Configuration Lock Register"]
    pub lock: LOCK,
    #[doc = "0x3c - Wake Up Enable"]
    pub em4wuen: EM4WUEN,
    #[doc = "0x40 - CC Channel Control Register"]
    pub cc0_ctrl: CC0_CTRL,
    #[doc = "0x44 - Capture/Compare Value Register"]
    pub cc0_ccv: CC0_CCV,
    #[doc = "0x48 - Capture/Compare Time Register"]
    pub cc0_time: CC0_TIME,
    #[doc = "0x4c - Capture/Compare Date Register"]
    pub cc0_date: CC0_DATE,
    #[doc = "0x50 - CC Channel Control Register"]
    pub cc1_ctrl: CC1_CTRL,
    #[doc = "0x54 - Capture/Compare Value Register"]
    pub cc1_ccv: CC1_CCV,
    #[doc = "0x58 - Capture/Compare Time Register"]
    pub cc1_time: CC1_TIME,
    #[doc = "0x5c - Capture/Compare Date Register"]
    pub cc1_date: CC1_DATE,
    #[doc = "0x60 - CC Channel Control Register"]
    pub cc2_ctrl: CC2_CTRL,
    #[doc = "0x64 - Capture/Compare Value Register"]
    pub cc2_ccv: CC2_CCV,
    #[doc = "0x68 - Capture/Compare Time Register"]
    pub cc2_time: CC2_TIME,
    #[doc = "0x6c - Capture/Compare Date Register"]
    pub cc2_date: CC2_DATE,
    _reserved28: [u8; 148usize],
    #[doc = "0x104 - Retention Register"]
    pub ret0_reg: RET0_REG,
    #[doc = "0x108 - Retention Register"]
    pub ret1_reg: RET1_REG,
    #[doc = "0x10c - Retention Register"]
    pub ret2_reg: RET2_REG,
    #[doc = "0x110 - Retention Register"]
    pub ret3_reg: RET3_REG,
    #[doc = "0x114 - Retention Register"]
    pub ret4_reg: RET4_REG,
    #[doc = "0x118 - Retention Register"]
    pub ret5_reg: RET5_REG,
    #[doc = "0x11c - Retention Register"]
    pub ret6_reg: RET6_REG,
    #[doc = "0x120 - Retention Register"]
    pub ret7_reg: RET7_REG,
    #[doc = "0x124 - Retention Register"]
    pub ret8_reg: RET8_REG,
    #[doc = "0x128 - Retention Register"]
    pub ret9_reg: RET9_REG,
    #[doc = "0x12c - Retention Register"]
    pub ret10_reg: RET10_REG,
    #[doc = "0x130 - Retention Register"]
    pub ret11_reg: RET11_REG,
    #[doc = "0x134 - Retention Register"]
    pub ret12_reg: RET12_REG,
    #[doc = "0x138 - Retention Register"]
    pub ret13_reg: RET13_REG,
    #[doc = "0x13c - Retention Register"]
    pub ret14_reg: RET14_REG,
    #[doc = "0x140 - Retention Register"]
    pub ret15_reg: RET15_REG,
    #[doc = "0x144 - Retention Register"]
    pub ret16_reg: RET16_REG,
    #[doc = "0x148 - Retention Register"]
    pub ret17_reg: RET17_REG,
    #[doc = "0x14c - Retention Register"]
    pub ret18_reg: RET18_REG,
    #[doc = "0x150 - Retention Register"]
    pub ret19_reg: RET19_REG,
    #[doc = "0x154 - Retention Register"]
    pub ret20_reg: RET20_REG,
    #[doc = "0x158 - Retention Register"]
    pub ret21_reg: RET21_REG,
    #[doc = "0x15c - Retention Register"]
    pub ret22_reg: RET22_REG,
    #[doc = "0x160 - Retention Register"]
    pub ret23_reg: RET23_REG,
    #[doc = "0x164 - Retention Register"]
    pub ret24_reg: RET24_REG,
    #[doc = "0x168 - Retention Register"]
    pub ret25_reg: RET25_REG,
    #[doc = "0x16c - Retention Register"]
    pub ret26_reg: RET26_REG,
    #[doc = "0x170 - Retention Register"]
    pub ret27_reg: RET27_REG,
    #[doc = "0x174 - Retention Register"]
    pub ret28_reg: RET28_REG,
    #[doc = "0x178 - Retention Register"]
    pub ret29_reg: RET29_REG,
    #[doc = "0x17c - Retention Register"]
    pub ret30_reg: RET30_REG,
    #[doc = "0x180 - Retention Register"]
    pub ret31_reg: RET31_REG,
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
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
#[doc = "Pre-Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [precnt](precnt) module"]
pub type PRECNT = crate::Reg<u32, _PRECNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRECNT;
#[doc = "`read()` method returns [precnt::R](precnt::R) reader structure"]
impl crate::Readable for PRECNT {}
#[doc = "`write(|w| ..)` method takes [precnt::W](precnt::W) writer structure"]
impl crate::Writable for PRECNT {}
#[doc = "Pre-Counter Value Register"]
pub mod precnt;
#[doc = "Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cnt](cnt) module"]
pub type CNT = crate::Reg<u32, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
#[doc = "`read()` method returns [cnt::R](cnt::R) reader structure"]
impl crate::Readable for CNT {}
#[doc = "`write(|w| ..)` method takes [cnt::W](cnt::W) writer structure"]
impl crate::Writable for CNT {}
#[doc = "Counter Value Register"]
pub mod cnt;
#[doc = "Combined Pre-Counter and Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [combcnt](combcnt) module"]
pub type COMBCNT = crate::Reg<u32, _COMBCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMBCNT;
#[doc = "`read()` method returns [combcnt::R](combcnt::R) reader structure"]
impl crate::Readable for COMBCNT {}
#[doc = "Combined Pre-Counter and Counter Value Register"]
pub mod combcnt;
#[doc = "Time of Day Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [time](time) module"]
pub type TIME = crate::Reg<u32, _TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIME;
#[doc = "`read()` method returns [time::R](time::R) reader structure"]
impl crate::Readable for TIME {}
#[doc = "`write(|w| ..)` method takes [time::W](time::W) writer structure"]
impl crate::Writable for TIME {}
#[doc = "Time of Day Register"]
pub mod time;
#[doc = "Date Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [date](date) module"]
pub type DATE = crate::Reg<u32, _DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATE;
#[doc = "`read()` method returns [date::R](date::R) reader structure"]
impl crate::Readable for DATE {}
#[doc = "`write(|w| ..)` method takes [date::W](date::W) writer structure"]
impl crate::Writable for DATE {}
#[doc = "Date Register"]
pub mod date;
#[doc = "RTCC Interrupt Flags\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [if_](if_) module"]
pub type IF = crate::Reg<u32, _IF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF;
#[doc = "`read()` method returns [if_::R](if_::R) reader structure"]
impl crate::Readable for IF {}
#[doc = "RTCC Interrupt Flags"]
pub mod if_;
#[doc = "Interrupt Flag Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ifs](ifs) module"]
pub type IFS = crate::Reg<u32, _IFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFS;
#[doc = "`write(|w| ..)` method takes [ifs::W](ifs::W) writer structure"]
impl crate::Writable for IFS {}
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "Interrupt Flag Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ifc](ifc) module"]
pub type IFC = crate::Reg<u32, _IFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFC;
#[doc = "`write(|w| ..)` method takes [ifc::W](ifc::W) writer structure"]
impl crate::Writable for IFC {}
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ien](ien) module"]
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
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status Register"]
pub mod status;
#[doc = "Command Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "Synchronization Busy Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syncbusy](syncbusy) module"]
pub type SYNCBUSY = crate::Reg<u32, _SYNCBUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNCBUSY;
#[doc = "`read()` method returns [syncbusy::R](syncbusy::R) reader structure"]
impl crate::Readable for SYNCBUSY {}
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "Retention RAM Power-down Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [powerdown](powerdown) module"]
pub type POWERDOWN = crate::Reg<u32, _POWERDOWN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POWERDOWN;
#[doc = "`read()` method returns [powerdown::R](powerdown::R) reader structure"]
impl crate::Readable for POWERDOWN {}
#[doc = "`write(|w| ..)` method takes [powerdown::W](powerdown::W) writer structure"]
impl crate::Writable for POWERDOWN {}
#[doc = "Retention RAM Power-down Register"]
pub mod powerdown;
#[doc = "Configuration Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lock](lock) module"]
pub type LOCK = crate::Reg<u32, _LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCK;
#[doc = "`read()` method returns [lock::R](lock::R) reader structure"]
impl crate::Readable for LOCK {}
#[doc = "`write(|w| ..)` method takes [lock::W](lock::W) writer structure"]
impl crate::Writable for LOCK {}
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "Wake Up Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [em4wuen](em4wuen) module"]
pub type EM4WUEN = crate::Reg<u32, _EM4WUEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EM4WUEN;
#[doc = "`read()` method returns [em4wuen::R](em4wuen::R) reader structure"]
impl crate::Readable for EM4WUEN {}
#[doc = "`write(|w| ..)` method takes [em4wuen::W](em4wuen::W) writer structure"]
impl crate::Writable for EM4WUEN {}
#[doc = "Wake Up Enable"]
pub mod em4wuen;
#[doc = "CC Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc0_ctrl](cc0_ctrl) module"]
pub type CC0_CTRL = crate::Reg<u32, _CC0_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC0_CTRL;
#[doc = "`read()` method returns [cc0_ctrl::R](cc0_ctrl::R) reader structure"]
impl crate::Readable for CC0_CTRL {}
#[doc = "`write(|w| ..)` method takes [cc0_ctrl::W](cc0_ctrl::W) writer structure"]
impl crate::Writable for CC0_CTRL {}
#[doc = "CC Channel Control Register"]
pub mod cc0_ctrl;
#[doc = "Capture/Compare Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc0_ccv](cc0_ccv) module"]
pub type CC0_CCV = crate::Reg<u32, _CC0_CCV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC0_CCV;
#[doc = "`read()` method returns [cc0_ccv::R](cc0_ccv::R) reader structure"]
impl crate::Readable for CC0_CCV {}
#[doc = "`write(|w| ..)` method takes [cc0_ccv::W](cc0_ccv::W) writer structure"]
impl crate::Writable for CC0_CCV {}
#[doc = "Capture/Compare Value Register"]
pub mod cc0_ccv;
#[doc = "Capture/Compare Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc0_time](cc0_time) module"]
pub type CC0_TIME = crate::Reg<u32, _CC0_TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC0_TIME;
#[doc = "`read()` method returns [cc0_time::R](cc0_time::R) reader structure"]
impl crate::Readable for CC0_TIME {}
#[doc = "`write(|w| ..)` method takes [cc0_time::W](cc0_time::W) writer structure"]
impl crate::Writable for CC0_TIME {}
#[doc = "Capture/Compare Time Register"]
pub mod cc0_time;
#[doc = "Capture/Compare Date Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc0_date](cc0_date) module"]
pub type CC0_DATE = crate::Reg<u32, _CC0_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC0_DATE;
#[doc = "`read()` method returns [cc0_date::R](cc0_date::R) reader structure"]
impl crate::Readable for CC0_DATE {}
#[doc = "`write(|w| ..)` method takes [cc0_date::W](cc0_date::W) writer structure"]
impl crate::Writable for CC0_DATE {}
#[doc = "Capture/Compare Date Register"]
pub mod cc0_date;
#[doc = "CC Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc1_ctrl](cc1_ctrl) module"]
pub type CC1_CTRL = crate::Reg<u32, _CC1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC1_CTRL;
#[doc = "`read()` method returns [cc1_ctrl::R](cc1_ctrl::R) reader structure"]
impl crate::Readable for CC1_CTRL {}
#[doc = "`write(|w| ..)` method takes [cc1_ctrl::W](cc1_ctrl::W) writer structure"]
impl crate::Writable for CC1_CTRL {}
#[doc = "CC Channel Control Register"]
pub mod cc1_ctrl;
#[doc = "Capture/Compare Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc1_ccv](cc1_ccv) module"]
pub type CC1_CCV = crate::Reg<u32, _CC1_CCV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC1_CCV;
#[doc = "`read()` method returns [cc1_ccv::R](cc1_ccv::R) reader structure"]
impl crate::Readable for CC1_CCV {}
#[doc = "`write(|w| ..)` method takes [cc1_ccv::W](cc1_ccv::W) writer structure"]
impl crate::Writable for CC1_CCV {}
#[doc = "Capture/Compare Value Register"]
pub mod cc1_ccv;
#[doc = "Capture/Compare Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc1_time](cc1_time) module"]
pub type CC1_TIME = crate::Reg<u32, _CC1_TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC1_TIME;
#[doc = "`read()` method returns [cc1_time::R](cc1_time::R) reader structure"]
impl crate::Readable for CC1_TIME {}
#[doc = "`write(|w| ..)` method takes [cc1_time::W](cc1_time::W) writer structure"]
impl crate::Writable for CC1_TIME {}
#[doc = "Capture/Compare Time Register"]
pub mod cc1_time;
#[doc = "Capture/Compare Date Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc1_date](cc1_date) module"]
pub type CC1_DATE = crate::Reg<u32, _CC1_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC1_DATE;
#[doc = "`read()` method returns [cc1_date::R](cc1_date::R) reader structure"]
impl crate::Readable for CC1_DATE {}
#[doc = "`write(|w| ..)` method takes [cc1_date::W](cc1_date::W) writer structure"]
impl crate::Writable for CC1_DATE {}
#[doc = "Capture/Compare Date Register"]
pub mod cc1_date;
#[doc = "CC Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc2_ctrl](cc2_ctrl) module"]
pub type CC2_CTRL = crate::Reg<u32, _CC2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC2_CTRL;
#[doc = "`read()` method returns [cc2_ctrl::R](cc2_ctrl::R) reader structure"]
impl crate::Readable for CC2_CTRL {}
#[doc = "`write(|w| ..)` method takes [cc2_ctrl::W](cc2_ctrl::W) writer structure"]
impl crate::Writable for CC2_CTRL {}
#[doc = "CC Channel Control Register"]
pub mod cc2_ctrl;
#[doc = "Capture/Compare Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc2_ccv](cc2_ccv) module"]
pub type CC2_CCV = crate::Reg<u32, _CC2_CCV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC2_CCV;
#[doc = "`read()` method returns [cc2_ccv::R](cc2_ccv::R) reader structure"]
impl crate::Readable for CC2_CCV {}
#[doc = "`write(|w| ..)` method takes [cc2_ccv::W](cc2_ccv::W) writer structure"]
impl crate::Writable for CC2_CCV {}
#[doc = "Capture/Compare Value Register"]
pub mod cc2_ccv;
#[doc = "Capture/Compare Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc2_time](cc2_time) module"]
pub type CC2_TIME = crate::Reg<u32, _CC2_TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC2_TIME;
#[doc = "`read()` method returns [cc2_time::R](cc2_time::R) reader structure"]
impl crate::Readable for CC2_TIME {}
#[doc = "`write(|w| ..)` method takes [cc2_time::W](cc2_time::W) writer structure"]
impl crate::Writable for CC2_TIME {}
#[doc = "Capture/Compare Time Register"]
pub mod cc2_time;
#[doc = "Capture/Compare Date Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cc2_date](cc2_date) module"]
pub type CC2_DATE = crate::Reg<u32, _CC2_DATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC2_DATE;
#[doc = "`read()` method returns [cc2_date::R](cc2_date::R) reader structure"]
impl crate::Readable for CC2_DATE {}
#[doc = "`write(|w| ..)` method takes [cc2_date::W](cc2_date::W) writer structure"]
impl crate::Writable for CC2_DATE {}
#[doc = "Capture/Compare Date Register"]
pub mod cc2_date;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret0_reg](ret0_reg) module"]
pub type RET0_REG = crate::Reg<u32, _RET0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET0_REG;
#[doc = "`read()` method returns [ret0_reg::R](ret0_reg::R) reader structure"]
impl crate::Readable for RET0_REG {}
#[doc = "`write(|w| ..)` method takes [ret0_reg::W](ret0_reg::W) writer structure"]
impl crate::Writable for RET0_REG {}
#[doc = "Retention Register"]
pub mod ret0_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret1_reg](ret1_reg) module"]
pub type RET1_REG = crate::Reg<u32, _RET1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET1_REG;
#[doc = "`read()` method returns [ret1_reg::R](ret1_reg::R) reader structure"]
impl crate::Readable for RET1_REG {}
#[doc = "`write(|w| ..)` method takes [ret1_reg::W](ret1_reg::W) writer structure"]
impl crate::Writable for RET1_REG {}
#[doc = "Retention Register"]
pub mod ret1_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret2_reg](ret2_reg) module"]
pub type RET2_REG = crate::Reg<u32, _RET2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET2_REG;
#[doc = "`read()` method returns [ret2_reg::R](ret2_reg::R) reader structure"]
impl crate::Readable for RET2_REG {}
#[doc = "`write(|w| ..)` method takes [ret2_reg::W](ret2_reg::W) writer structure"]
impl crate::Writable for RET2_REG {}
#[doc = "Retention Register"]
pub mod ret2_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret3_reg](ret3_reg) module"]
pub type RET3_REG = crate::Reg<u32, _RET3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET3_REG;
#[doc = "`read()` method returns [ret3_reg::R](ret3_reg::R) reader structure"]
impl crate::Readable for RET3_REG {}
#[doc = "`write(|w| ..)` method takes [ret3_reg::W](ret3_reg::W) writer structure"]
impl crate::Writable for RET3_REG {}
#[doc = "Retention Register"]
pub mod ret3_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret4_reg](ret4_reg) module"]
pub type RET4_REG = crate::Reg<u32, _RET4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET4_REG;
#[doc = "`read()` method returns [ret4_reg::R](ret4_reg::R) reader structure"]
impl crate::Readable for RET4_REG {}
#[doc = "`write(|w| ..)` method takes [ret4_reg::W](ret4_reg::W) writer structure"]
impl crate::Writable for RET4_REG {}
#[doc = "Retention Register"]
pub mod ret4_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret5_reg](ret5_reg) module"]
pub type RET5_REG = crate::Reg<u32, _RET5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET5_REG;
#[doc = "`read()` method returns [ret5_reg::R](ret5_reg::R) reader structure"]
impl crate::Readable for RET5_REG {}
#[doc = "`write(|w| ..)` method takes [ret5_reg::W](ret5_reg::W) writer structure"]
impl crate::Writable for RET5_REG {}
#[doc = "Retention Register"]
pub mod ret5_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret6_reg](ret6_reg) module"]
pub type RET6_REG = crate::Reg<u32, _RET6_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET6_REG;
#[doc = "`read()` method returns [ret6_reg::R](ret6_reg::R) reader structure"]
impl crate::Readable for RET6_REG {}
#[doc = "`write(|w| ..)` method takes [ret6_reg::W](ret6_reg::W) writer structure"]
impl crate::Writable for RET6_REG {}
#[doc = "Retention Register"]
pub mod ret6_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret7_reg](ret7_reg) module"]
pub type RET7_REG = crate::Reg<u32, _RET7_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET7_REG;
#[doc = "`read()` method returns [ret7_reg::R](ret7_reg::R) reader structure"]
impl crate::Readable for RET7_REG {}
#[doc = "`write(|w| ..)` method takes [ret7_reg::W](ret7_reg::W) writer structure"]
impl crate::Writable for RET7_REG {}
#[doc = "Retention Register"]
pub mod ret7_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret8_reg](ret8_reg) module"]
pub type RET8_REG = crate::Reg<u32, _RET8_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET8_REG;
#[doc = "`read()` method returns [ret8_reg::R](ret8_reg::R) reader structure"]
impl crate::Readable for RET8_REG {}
#[doc = "`write(|w| ..)` method takes [ret8_reg::W](ret8_reg::W) writer structure"]
impl crate::Writable for RET8_REG {}
#[doc = "Retention Register"]
pub mod ret8_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret9_reg](ret9_reg) module"]
pub type RET9_REG = crate::Reg<u32, _RET9_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET9_REG;
#[doc = "`read()` method returns [ret9_reg::R](ret9_reg::R) reader structure"]
impl crate::Readable for RET9_REG {}
#[doc = "`write(|w| ..)` method takes [ret9_reg::W](ret9_reg::W) writer structure"]
impl crate::Writable for RET9_REG {}
#[doc = "Retention Register"]
pub mod ret9_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret10_reg](ret10_reg) module"]
pub type RET10_REG = crate::Reg<u32, _RET10_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET10_REG;
#[doc = "`read()` method returns [ret10_reg::R](ret10_reg::R) reader structure"]
impl crate::Readable for RET10_REG {}
#[doc = "`write(|w| ..)` method takes [ret10_reg::W](ret10_reg::W) writer structure"]
impl crate::Writable for RET10_REG {}
#[doc = "Retention Register"]
pub mod ret10_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret11_reg](ret11_reg) module"]
pub type RET11_REG = crate::Reg<u32, _RET11_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET11_REG;
#[doc = "`read()` method returns [ret11_reg::R](ret11_reg::R) reader structure"]
impl crate::Readable for RET11_REG {}
#[doc = "`write(|w| ..)` method takes [ret11_reg::W](ret11_reg::W) writer structure"]
impl crate::Writable for RET11_REG {}
#[doc = "Retention Register"]
pub mod ret11_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret12_reg](ret12_reg) module"]
pub type RET12_REG = crate::Reg<u32, _RET12_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET12_REG;
#[doc = "`read()` method returns [ret12_reg::R](ret12_reg::R) reader structure"]
impl crate::Readable for RET12_REG {}
#[doc = "`write(|w| ..)` method takes [ret12_reg::W](ret12_reg::W) writer structure"]
impl crate::Writable for RET12_REG {}
#[doc = "Retention Register"]
pub mod ret12_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret13_reg](ret13_reg) module"]
pub type RET13_REG = crate::Reg<u32, _RET13_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET13_REG;
#[doc = "`read()` method returns [ret13_reg::R](ret13_reg::R) reader structure"]
impl crate::Readable for RET13_REG {}
#[doc = "`write(|w| ..)` method takes [ret13_reg::W](ret13_reg::W) writer structure"]
impl crate::Writable for RET13_REG {}
#[doc = "Retention Register"]
pub mod ret13_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret14_reg](ret14_reg) module"]
pub type RET14_REG = crate::Reg<u32, _RET14_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET14_REG;
#[doc = "`read()` method returns [ret14_reg::R](ret14_reg::R) reader structure"]
impl crate::Readable for RET14_REG {}
#[doc = "`write(|w| ..)` method takes [ret14_reg::W](ret14_reg::W) writer structure"]
impl crate::Writable for RET14_REG {}
#[doc = "Retention Register"]
pub mod ret14_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret15_reg](ret15_reg) module"]
pub type RET15_REG = crate::Reg<u32, _RET15_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET15_REG;
#[doc = "`read()` method returns [ret15_reg::R](ret15_reg::R) reader structure"]
impl crate::Readable for RET15_REG {}
#[doc = "`write(|w| ..)` method takes [ret15_reg::W](ret15_reg::W) writer structure"]
impl crate::Writable for RET15_REG {}
#[doc = "Retention Register"]
pub mod ret15_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret16_reg](ret16_reg) module"]
pub type RET16_REG = crate::Reg<u32, _RET16_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET16_REG;
#[doc = "`read()` method returns [ret16_reg::R](ret16_reg::R) reader structure"]
impl crate::Readable for RET16_REG {}
#[doc = "`write(|w| ..)` method takes [ret16_reg::W](ret16_reg::W) writer structure"]
impl crate::Writable for RET16_REG {}
#[doc = "Retention Register"]
pub mod ret16_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret17_reg](ret17_reg) module"]
pub type RET17_REG = crate::Reg<u32, _RET17_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET17_REG;
#[doc = "`read()` method returns [ret17_reg::R](ret17_reg::R) reader structure"]
impl crate::Readable for RET17_REG {}
#[doc = "`write(|w| ..)` method takes [ret17_reg::W](ret17_reg::W) writer structure"]
impl crate::Writable for RET17_REG {}
#[doc = "Retention Register"]
pub mod ret17_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret18_reg](ret18_reg) module"]
pub type RET18_REG = crate::Reg<u32, _RET18_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET18_REG;
#[doc = "`read()` method returns [ret18_reg::R](ret18_reg::R) reader structure"]
impl crate::Readable for RET18_REG {}
#[doc = "`write(|w| ..)` method takes [ret18_reg::W](ret18_reg::W) writer structure"]
impl crate::Writable for RET18_REG {}
#[doc = "Retention Register"]
pub mod ret18_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret19_reg](ret19_reg) module"]
pub type RET19_REG = crate::Reg<u32, _RET19_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET19_REG;
#[doc = "`read()` method returns [ret19_reg::R](ret19_reg::R) reader structure"]
impl crate::Readable for RET19_REG {}
#[doc = "`write(|w| ..)` method takes [ret19_reg::W](ret19_reg::W) writer structure"]
impl crate::Writable for RET19_REG {}
#[doc = "Retention Register"]
pub mod ret19_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret20_reg](ret20_reg) module"]
pub type RET20_REG = crate::Reg<u32, _RET20_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET20_REG;
#[doc = "`read()` method returns [ret20_reg::R](ret20_reg::R) reader structure"]
impl crate::Readable for RET20_REG {}
#[doc = "`write(|w| ..)` method takes [ret20_reg::W](ret20_reg::W) writer structure"]
impl crate::Writable for RET20_REG {}
#[doc = "Retention Register"]
pub mod ret20_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret21_reg](ret21_reg) module"]
pub type RET21_REG = crate::Reg<u32, _RET21_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET21_REG;
#[doc = "`read()` method returns [ret21_reg::R](ret21_reg::R) reader structure"]
impl crate::Readable for RET21_REG {}
#[doc = "`write(|w| ..)` method takes [ret21_reg::W](ret21_reg::W) writer structure"]
impl crate::Writable for RET21_REG {}
#[doc = "Retention Register"]
pub mod ret21_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret22_reg](ret22_reg) module"]
pub type RET22_REG = crate::Reg<u32, _RET22_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET22_REG;
#[doc = "`read()` method returns [ret22_reg::R](ret22_reg::R) reader structure"]
impl crate::Readable for RET22_REG {}
#[doc = "`write(|w| ..)` method takes [ret22_reg::W](ret22_reg::W) writer structure"]
impl crate::Writable for RET22_REG {}
#[doc = "Retention Register"]
pub mod ret22_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret23_reg](ret23_reg) module"]
pub type RET23_REG = crate::Reg<u32, _RET23_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET23_REG;
#[doc = "`read()` method returns [ret23_reg::R](ret23_reg::R) reader structure"]
impl crate::Readable for RET23_REG {}
#[doc = "`write(|w| ..)` method takes [ret23_reg::W](ret23_reg::W) writer structure"]
impl crate::Writable for RET23_REG {}
#[doc = "Retention Register"]
pub mod ret23_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret24_reg](ret24_reg) module"]
pub type RET24_REG = crate::Reg<u32, _RET24_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET24_REG;
#[doc = "`read()` method returns [ret24_reg::R](ret24_reg::R) reader structure"]
impl crate::Readable for RET24_REG {}
#[doc = "`write(|w| ..)` method takes [ret24_reg::W](ret24_reg::W) writer structure"]
impl crate::Writable for RET24_REG {}
#[doc = "Retention Register"]
pub mod ret24_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret25_reg](ret25_reg) module"]
pub type RET25_REG = crate::Reg<u32, _RET25_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET25_REG;
#[doc = "`read()` method returns [ret25_reg::R](ret25_reg::R) reader structure"]
impl crate::Readable for RET25_REG {}
#[doc = "`write(|w| ..)` method takes [ret25_reg::W](ret25_reg::W) writer structure"]
impl crate::Writable for RET25_REG {}
#[doc = "Retention Register"]
pub mod ret25_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret26_reg](ret26_reg) module"]
pub type RET26_REG = crate::Reg<u32, _RET26_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET26_REG;
#[doc = "`read()` method returns [ret26_reg::R](ret26_reg::R) reader structure"]
impl crate::Readable for RET26_REG {}
#[doc = "`write(|w| ..)` method takes [ret26_reg::W](ret26_reg::W) writer structure"]
impl crate::Writable for RET26_REG {}
#[doc = "Retention Register"]
pub mod ret26_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret27_reg](ret27_reg) module"]
pub type RET27_REG = crate::Reg<u32, _RET27_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET27_REG;
#[doc = "`read()` method returns [ret27_reg::R](ret27_reg::R) reader structure"]
impl crate::Readable for RET27_REG {}
#[doc = "`write(|w| ..)` method takes [ret27_reg::W](ret27_reg::W) writer structure"]
impl crate::Writable for RET27_REG {}
#[doc = "Retention Register"]
pub mod ret27_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret28_reg](ret28_reg) module"]
pub type RET28_REG = crate::Reg<u32, _RET28_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET28_REG;
#[doc = "`read()` method returns [ret28_reg::R](ret28_reg::R) reader structure"]
impl crate::Readable for RET28_REG {}
#[doc = "`write(|w| ..)` method takes [ret28_reg::W](ret28_reg::W) writer structure"]
impl crate::Writable for RET28_REG {}
#[doc = "Retention Register"]
pub mod ret28_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret29_reg](ret29_reg) module"]
pub type RET29_REG = crate::Reg<u32, _RET29_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET29_REG;
#[doc = "`read()` method returns [ret29_reg::R](ret29_reg::R) reader structure"]
impl crate::Readable for RET29_REG {}
#[doc = "`write(|w| ..)` method takes [ret29_reg::W](ret29_reg::W) writer structure"]
impl crate::Writable for RET29_REG {}
#[doc = "Retention Register"]
pub mod ret29_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret30_reg](ret30_reg) module"]
pub type RET30_REG = crate::Reg<u32, _RET30_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET30_REG;
#[doc = "`read()` method returns [ret30_reg::R](ret30_reg::R) reader structure"]
impl crate::Readable for RET30_REG {}
#[doc = "`write(|w| ..)` method takes [ret30_reg::W](ret30_reg::W) writer structure"]
impl crate::Writable for RET30_REG {}
#[doc = "Retention Register"]
pub mod ret30_reg;
#[doc = "Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ret31_reg](ret31_reg) module"]
pub type RET31_REG = crate::Reg<u32, _RET31_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RET31_REG;
#[doc = "`read()` method returns [ret31_reg::R](ret31_reg::R) reader structure"]
impl crate::Readable for RET31_REG {}
#[doc = "`write(|w| ..)` method takes [ret31_reg::W](ret31_reg::W) writer structure"]
impl crate::Writable for RET31_REG {}
#[doc = "Retention Register"]
pub mod ret31_reg;
