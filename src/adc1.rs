#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x0c - Status Register"]
    pub status: STATUS,
    #[doc = "0x10 - Single Channel Control Register"]
    pub singlectrl: SINGLECTRL,
    #[doc = "0x14 - Single Channel Control Register Continued"]
    pub singlectrlx: SINGLECTRLX,
    #[doc = "0x18 - Scan Control Register"]
    pub scanctrl: SCANCTRL,
    #[doc = "0x1c - Scan Control Register Continued"]
    pub scanctrlx: SCANCTRLX,
    #[doc = "0x20 - Scan Sequence Input Mask Register"]
    pub scanmask: SCANMASK,
    #[doc = "0x24 - Input Selection Register for Scan Mode"]
    pub scaninputsel: SCANINPUTSEL,
    #[doc = "0x28 - Negative Input Select Register for Scan"]
    pub scannegsel: SCANNEGSEL,
    #[doc = "0x2c - Compare Threshold Register"]
    pub cmpthr: CMPTHR,
    #[doc = "0x30 - Bias Programming Register for Various Analog Blocks Used in ADC Operation"]
    pub biasprog: BIASPROG,
    #[doc = "0x34 - Calibration Register"]
    pub cal: CAL,
    #[doc = "0x38 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x3c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x40 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x44 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x48 - Single Conversion Result Data"]
    pub singledata: SINGLEDATA,
    #[doc = "0x4c - Scan Conversion Result Data"]
    pub scandata: SCANDATA,
    #[doc = "0x50 - Single Conversion Result Data Peek Register"]
    pub singledatap: SINGLEDATAP,
    #[doc = "0x54 - Scan Sequence Result Data Peek Register"]
    pub scandatap: SCANDATAP,
    _reserved21: [u8; 16usize],
    #[doc = "0x68 - Scan Sequence Result Data + Data Source Register"]
    pub scandatax: SCANDATAX,
    #[doc = "0x6c - Scan Sequence Result Data + Data Source Peek Register"]
    pub scandataxp: SCANDATAXP,
    _reserved23: [u8; 12usize],
    #[doc = "0x7c - APORT Request Status Register"]
    pub aportreq: APORTREQ,
    #[doc = "0x80 - APORT Conflict Status Register"]
    pub aportconflict: APORTCONFLICT,
    #[doc = "0x84 - Single FIFO Count Register"]
    pub singlefifocount: SINGLEFIFOCOUNT,
    #[doc = "0x88 - Scan FIFO Count Register"]
    pub scanfifocount: SCANFIFOCOUNT,
    #[doc = "0x8c - Single FIFO Clear Register"]
    pub singlefifoclear: SINGLEFIFOCLEAR,
    #[doc = "0x90 - Scan FIFO Clear Register"]
    pub scanfifoclear: SCANFIFOCLEAR,
    #[doc = "0x94 - APORT Bus Master Disable Register"]
    pub aportmasterdis: APORTMASTERDIS,
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
#[doc = "Command Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status Register"]
pub mod status;
#[doc = "Single Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singlectrl](singlectrl) module"]
pub type SINGLECTRL = crate::Reg<u32, _SINGLECTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SINGLECTRL;
#[doc = "`read()` method returns [singlectrl::R](singlectrl::R) reader structure"]
impl crate::Readable for SINGLECTRL {}
#[doc = "`write(|w| ..)` method takes [singlectrl::W](singlectrl::W) writer structure"]
impl crate::Writable for SINGLECTRL {}
#[doc = "Single Channel Control Register"]
pub mod singlectrl;
#[doc = "Single Channel Control Register Continued\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singlectrlx](singlectrlx) module"]
pub type SINGLECTRLX = crate::Reg<u32, _SINGLECTRLX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SINGLECTRLX;
#[doc = "`read()` method returns [singlectrlx::R](singlectrlx::R) reader structure"]
impl crate::Readable for SINGLECTRLX {}
#[doc = "`write(|w| ..)` method takes [singlectrlx::W](singlectrlx::W) writer structure"]
impl crate::Writable for SINGLECTRLX {}
#[doc = "Single Channel Control Register Continued"]
pub mod singlectrlx;
#[doc = "Scan Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scanctrl](scanctrl) module"]
pub type SCANCTRL = crate::Reg<u32, _SCANCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCANCTRL;
#[doc = "`read()` method returns [scanctrl::R](scanctrl::R) reader structure"]
impl crate::Readable for SCANCTRL {}
#[doc = "`write(|w| ..)` method takes [scanctrl::W](scanctrl::W) writer structure"]
impl crate::Writable for SCANCTRL {}
#[doc = "Scan Control Register"]
pub mod scanctrl;
#[doc = "Scan Control Register Continued\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scanctrlx](scanctrlx) module"]
pub type SCANCTRLX = crate::Reg<u32, _SCANCTRLX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCANCTRLX;
#[doc = "`read()` method returns [scanctrlx::R](scanctrlx::R) reader structure"]
impl crate::Readable for SCANCTRLX {}
#[doc = "`write(|w| ..)` method takes [scanctrlx::W](scanctrlx::W) writer structure"]
impl crate::Writable for SCANCTRLX {}
#[doc = "Scan Control Register Continued"]
pub mod scanctrlx;
#[doc = "Scan Sequence Input Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scanmask](scanmask) module"]
pub type SCANMASK = crate::Reg<u32, _SCANMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCANMASK;
#[doc = "`read()` method returns [scanmask::R](scanmask::R) reader structure"]
impl crate::Readable for SCANMASK {}
#[doc = "`write(|w| ..)` method takes [scanmask::W](scanmask::W) writer structure"]
impl crate::Writable for SCANMASK {}
#[doc = "Scan Sequence Input Mask Register"]
pub mod scanmask;
#[doc = "Input Selection Register for Scan Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scaninputsel](scaninputsel) module"]
pub type SCANINPUTSEL = crate::Reg<u32, _SCANINPUTSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCANINPUTSEL;
#[doc = "`read()` method returns [scaninputsel::R](scaninputsel::R) reader structure"]
impl crate::Readable for SCANINPUTSEL {}
#[doc = "`write(|w| ..)` method takes [scaninputsel::W](scaninputsel::W) writer structure"]
impl crate::Writable for SCANINPUTSEL {}
#[doc = "Input Selection Register for Scan Mode"]
pub mod scaninputsel;
#[doc = "Negative Input Select Register for Scan\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scannegsel](scannegsel) module"]
pub type SCANNEGSEL = crate::Reg<u32, _SCANNEGSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCANNEGSEL;
#[doc = "`read()` method returns [scannegsel::R](scannegsel::R) reader structure"]
impl crate::Readable for SCANNEGSEL {}
#[doc = "`write(|w| ..)` method takes [scannegsel::W](scannegsel::W) writer structure"]
impl crate::Writable for SCANNEGSEL {}
#[doc = "Negative Input Select Register for Scan"]
pub mod scannegsel;
#[doc = "Compare Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpthr](cmpthr) module"]
pub type CMPTHR = crate::Reg<u32, _CMPTHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPTHR;
#[doc = "`read()` method returns [cmpthr::R](cmpthr::R) reader structure"]
impl crate::Readable for CMPTHR {}
#[doc = "`write(|w| ..)` method takes [cmpthr::W](cmpthr::W) writer structure"]
impl crate::Writable for CMPTHR {}
#[doc = "Compare Threshold Register"]
pub mod cmpthr;
#[doc = "Bias Programming Register for Various Analog Blocks Used in ADC Operation\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [biasprog](biasprog) module"]
pub type BIASPROG = crate::Reg<u32, _BIASPROG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BIASPROG;
#[doc = "`read()` method returns [biasprog::R](biasprog::R) reader structure"]
impl crate::Readable for BIASPROG {}
#[doc = "`write(|w| ..)` method takes [biasprog::W](biasprog::W) writer structure"]
impl crate::Writable for BIASPROG {}
#[doc = "Bias Programming Register for Various Analog Blocks Used in ADC Operation"]
pub mod biasprog;
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
#[doc = "Single Conversion Result Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singledata](singledata) module"]
pub type SINGLEDATA = crate::Reg<u32, _SINGLEDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SINGLEDATA;
#[doc = "`read()` method returns [singledata::R](singledata::R) reader structure"]
impl crate::Readable for SINGLEDATA {}
#[doc = "Single Conversion Result Data"]
pub mod singledata;
#[doc = "Scan Conversion Result Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scandata](scandata) module"]
pub type SCANDATA = crate::Reg<u32, _SCANDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCANDATA;
#[doc = "`read()` method returns [scandata::R](scandata::R) reader structure"]
impl crate::Readable for SCANDATA {}
#[doc = "Scan Conversion Result Data"]
pub mod scandata;
#[doc = "Single Conversion Result Data Peek Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singledatap](singledatap) module"]
pub type SINGLEDATAP = crate::Reg<u32, _SINGLEDATAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SINGLEDATAP;
#[doc = "`read()` method returns [singledatap::R](singledatap::R) reader structure"]
impl crate::Readable for SINGLEDATAP {}
#[doc = "Single Conversion Result Data Peek Register"]
pub mod singledatap;
#[doc = "Scan Sequence Result Data Peek Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scandatap](scandatap) module"]
pub type SCANDATAP = crate::Reg<u32, _SCANDATAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCANDATAP;
#[doc = "`read()` method returns [scandatap::R](scandatap::R) reader structure"]
impl crate::Readable for SCANDATAP {}
#[doc = "Scan Sequence Result Data Peek Register"]
pub mod scandatap;
#[doc = "Scan Sequence Result Data + Data Source Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scandatax](scandatax) module"]
pub type SCANDATAX = crate::Reg<u32, _SCANDATAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCANDATAX;
#[doc = "`read()` method returns [scandatax::R](scandatax::R) reader structure"]
impl crate::Readable for SCANDATAX {}
#[doc = "Scan Sequence Result Data + Data Source Register"]
pub mod scandatax;
#[doc = "Scan Sequence Result Data + Data Source Peek Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scandataxp](scandataxp) module"]
pub type SCANDATAXP = crate::Reg<u32, _SCANDATAXP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCANDATAXP;
#[doc = "`read()` method returns [scandataxp::R](scandataxp::R) reader structure"]
impl crate::Readable for SCANDATAXP {}
#[doc = "Scan Sequence Result Data + Data Source Peek Register"]
pub mod scandataxp;
#[doc = "APORT Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aportreq](aportreq) module"]
pub type APORTREQ = crate::Reg<u32, _APORTREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APORTREQ;
#[doc = "`read()` method returns [aportreq::R](aportreq::R) reader structure"]
impl crate::Readable for APORTREQ {}
#[doc = "APORT Request Status Register"]
pub mod aportreq;
#[doc = "APORT Conflict Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aportconflict](aportconflict) module"]
pub type APORTCONFLICT = crate::Reg<u32, _APORTCONFLICT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APORTCONFLICT;
#[doc = "`read()` method returns [aportconflict::R](aportconflict::R) reader structure"]
impl crate::Readable for APORTCONFLICT {}
#[doc = "APORT Conflict Status Register"]
pub mod aportconflict;
#[doc = "Single FIFO Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singlefifocount](singlefifocount) module"]
pub type SINGLEFIFOCOUNT = crate::Reg<u32, _SINGLEFIFOCOUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SINGLEFIFOCOUNT;
#[doc = "`read()` method returns [singlefifocount::R](singlefifocount::R) reader structure"]
impl crate::Readable for SINGLEFIFOCOUNT {}
#[doc = "Single FIFO Count Register"]
pub mod singlefifocount;
#[doc = "Scan FIFO Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scanfifocount](scanfifocount) module"]
pub type SCANFIFOCOUNT = crate::Reg<u32, _SCANFIFOCOUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCANFIFOCOUNT;
#[doc = "`read()` method returns [scanfifocount::R](scanfifocount::R) reader structure"]
impl crate::Readable for SCANFIFOCOUNT {}
#[doc = "Scan FIFO Count Register"]
pub mod scanfifocount;
#[doc = "Single FIFO Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singlefifoclear](singlefifoclear) module"]
pub type SINGLEFIFOCLEAR = crate::Reg<u32, _SINGLEFIFOCLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SINGLEFIFOCLEAR;
#[doc = "`write(|w| ..)` method takes [singlefifoclear::W](singlefifoclear::W) writer structure"]
impl crate::Writable for SINGLEFIFOCLEAR {}
#[doc = "Single FIFO Clear Register"]
pub mod singlefifoclear;
#[doc = "Scan FIFO Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scanfifoclear](scanfifoclear) module"]
pub type SCANFIFOCLEAR = crate::Reg<u32, _SCANFIFOCLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCANFIFOCLEAR;
#[doc = "`write(|w| ..)` method takes [scanfifoclear::W](scanfifoclear::W) writer structure"]
impl crate::Writable for SCANFIFOCLEAR {}
#[doc = "Scan FIFO Clear Register"]
pub mod scanfifoclear;
#[doc = "APORT Bus Master Disable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aportmasterdis](aportmasterdis) module"]
pub type APORTMASTERDIS = crate::Reg<u32, _APORTMASTERDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APORTMASTERDIS;
#[doc = "`read()` method returns [aportmasterdis::R](aportmasterdis::R) reader structure"]
impl crate::Readable for APORTMASTERDIS {}
#[doc = "`write(|w| ..)` method takes [aportmasterdis::W](aportmasterdis::W) writer structure"]
impl crate::Writable for APORTMASTERDIS {}
#[doc = "APORT Bus Master Disable Register"]
pub mod aportmasterdis;
