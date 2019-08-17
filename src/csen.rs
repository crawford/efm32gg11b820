#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Timing Control"]
    pub timctrl: TIMCTRL,
    #[doc = "0x08 - Command"]
    pub cmd: CMD,
    #[doc = "0x0c - Status"]
    pub status: STATUS,
    #[doc = "0x10 - PRS Select"]
    pub prssel: PRSSEL,
    #[doc = "0x14 - Output Data"]
    pub data: DATA,
    #[doc = "0x18 - Scan Channel Mask 0"]
    pub scanmask0: SCANMASK0,
    #[doc = "0x1c - Scan Input Selection 0"]
    pub scaninputsel0: SCANINPUTSEL0,
    #[doc = "0x20 - Scan Channel Mask 1"]
    pub scanmask1: SCANMASK1,
    #[doc = "0x24 - Scan Input Selection 1"]
    pub scaninputsel1: SCANINPUTSEL1,
    #[doc = "0x28 - APORT Request Status"]
    pub aportreq: APORTREQ,
    #[doc = "0x2c - APORT Request Conflict"]
    pub aportconflict: APORTCONFLICT,
    #[doc = "0x30 - Comparator Threshold"]
    pub cmpthr: CMPTHR,
    #[doc = "0x34 - Exponential Moving Average"]
    pub ema: EMA,
    #[doc = "0x38 - Exponential Moving Average Control"]
    pub emactrl: EMACTRL,
    #[doc = "0x3c - Single Conversion Control"]
    pub singlectrl: SINGLECTRL,
    #[doc = "0x40 - Delta Modulation Baseline"]
    pub dmbaseline: DMBASELINE,
    #[doc = "0x44 - Delta Modulation Configuration"]
    pub dmcfg: DMCFG,
    #[doc = "0x48 - Analog Control"]
    pub anactrl: ANACTRL,
    _reserved19: [u8; 8usize],
    #[doc = "0x54 - Interrupt Flag"]
    pub if_: IF,
    #[doc = "0x58 - Interrupt Flag Set"]
    pub ifs: IFS,
    #[doc = "0x5c - Interrupt Flag Clear"]
    pub ifc: IFC,
    #[doc = "0x60 - Interrupt Enable"]
    pub ien: IEN,
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control"]
pub mod ctrl;
#[doc = "Timing Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timctrl](timctrl) module"]
pub type TIMCTRL = crate::Reg<u32, _TIMCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCTRL;
#[doc = "`read()` method returns [timctrl::R](timctrl::R) reader structure"]
impl crate::Readable for TIMCTRL {}
#[doc = "`write(|w| ..)` method takes [timctrl::W](timctrl::W) writer structure"]
impl crate::Writable for TIMCTRL {}
#[doc = "Timing Control"]
pub mod timctrl;
#[doc = "Command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Command"]
pub mod cmd;
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status"]
pub mod status;
#[doc = "PRS Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prssel](prssel) module"]
pub type PRSSEL = crate::Reg<u32, _PRSSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRSSEL;
#[doc = "`read()` method returns [prssel::R](prssel::R) reader structure"]
impl crate::Readable for PRSSEL {}
#[doc = "`write(|w| ..)` method takes [prssel::W](prssel::W) writer structure"]
impl crate::Writable for PRSSEL {}
#[doc = "PRS Select"]
pub mod prssel;
#[doc = "Output Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "Output Data"]
pub mod data;
#[doc = "Scan Channel Mask 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scanmask0](scanmask0) module"]
pub type SCANMASK0 = crate::Reg<u32, _SCANMASK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCANMASK0;
#[doc = "`read()` method returns [scanmask0::R](scanmask0::R) reader structure"]
impl crate::Readable for SCANMASK0 {}
#[doc = "`write(|w| ..)` method takes [scanmask0::W](scanmask0::W) writer structure"]
impl crate::Writable for SCANMASK0 {}
#[doc = "Scan Channel Mask 0"]
pub mod scanmask0;
#[doc = "Scan Input Selection 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scaninputsel0](scaninputsel0) module"]
pub type SCANINPUTSEL0 = crate::Reg<u32, _SCANINPUTSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCANINPUTSEL0;
#[doc = "`read()` method returns [scaninputsel0::R](scaninputsel0::R) reader structure"]
impl crate::Readable for SCANINPUTSEL0 {}
#[doc = "`write(|w| ..)` method takes [scaninputsel0::W](scaninputsel0::W) writer structure"]
impl crate::Writable for SCANINPUTSEL0 {}
#[doc = "Scan Input Selection 0"]
pub mod scaninputsel0;
#[doc = "Scan Channel Mask 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scanmask1](scanmask1) module"]
pub type SCANMASK1 = crate::Reg<u32, _SCANMASK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCANMASK1;
#[doc = "`read()` method returns [scanmask1::R](scanmask1::R) reader structure"]
impl crate::Readable for SCANMASK1 {}
#[doc = "`write(|w| ..)` method takes [scanmask1::W](scanmask1::W) writer structure"]
impl crate::Writable for SCANMASK1 {}
#[doc = "Scan Channel Mask 1"]
pub mod scanmask1;
#[doc = "Scan Input Selection 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scaninputsel1](scaninputsel1) module"]
pub type SCANINPUTSEL1 = crate::Reg<u32, _SCANINPUTSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCANINPUTSEL1;
#[doc = "`read()` method returns [scaninputsel1::R](scaninputsel1::R) reader structure"]
impl crate::Readable for SCANINPUTSEL1 {}
#[doc = "`write(|w| ..)` method takes [scaninputsel1::W](scaninputsel1::W) writer structure"]
impl crate::Writable for SCANINPUTSEL1 {}
#[doc = "Scan Input Selection 1"]
pub mod scaninputsel1;
#[doc = "APORT Request Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aportreq](aportreq) module"]
pub type APORTREQ = crate::Reg<u32, _APORTREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APORTREQ;
#[doc = "`read()` method returns [aportreq::R](aportreq::R) reader structure"]
impl crate::Readable for APORTREQ {}
#[doc = "APORT Request Status"]
pub mod aportreq;
#[doc = "APORT Request Conflict\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aportconflict](aportconflict) module"]
pub type APORTCONFLICT = crate::Reg<u32, _APORTCONFLICT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APORTCONFLICT;
#[doc = "`read()` method returns [aportconflict::R](aportconflict::R) reader structure"]
impl crate::Readable for APORTCONFLICT {}
#[doc = "APORT Request Conflict"]
pub mod aportconflict;
#[doc = "Comparator Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmpthr](cmpthr) module"]
pub type CMPTHR = crate::Reg<u32, _CMPTHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPTHR;
#[doc = "`read()` method returns [cmpthr::R](cmpthr::R) reader structure"]
impl crate::Readable for CMPTHR {}
#[doc = "`write(|w| ..)` method takes [cmpthr::W](cmpthr::W) writer structure"]
impl crate::Writable for CMPTHR {}
#[doc = "Comparator Threshold"]
pub mod cmpthr;
#[doc = "Exponential Moving Average\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ema](ema) module"]
pub type EMA = crate::Reg<u32, _EMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMA;
#[doc = "`read()` method returns [ema::R](ema::R) reader structure"]
impl crate::Readable for EMA {}
#[doc = "`write(|w| ..)` method takes [ema::W](ema::W) writer structure"]
impl crate::Writable for EMA {}
#[doc = "Exponential Moving Average"]
pub mod ema;
#[doc = "Exponential Moving Average Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [emactrl](emactrl) module"]
pub type EMACTRL = crate::Reg<u32, _EMACTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMACTRL;
#[doc = "`read()` method returns [emactrl::R](emactrl::R) reader structure"]
impl crate::Readable for EMACTRL {}
#[doc = "`write(|w| ..)` method takes [emactrl::W](emactrl::W) writer structure"]
impl crate::Writable for EMACTRL {}
#[doc = "Exponential Moving Average Control"]
pub mod emactrl;
#[doc = "Single Conversion Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [singlectrl](singlectrl) module"]
pub type SINGLECTRL = crate::Reg<u32, _SINGLECTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SINGLECTRL;
#[doc = "`read()` method returns [singlectrl::R](singlectrl::R) reader structure"]
impl crate::Readable for SINGLECTRL {}
#[doc = "`write(|w| ..)` method takes [singlectrl::W](singlectrl::W) writer structure"]
impl crate::Writable for SINGLECTRL {}
#[doc = "Single Conversion Control"]
pub mod singlectrl;
#[doc = "Delta Modulation Baseline\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmbaseline](dmbaseline) module"]
pub type DMBASELINE = crate::Reg<u32, _DMBASELINE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMBASELINE;
#[doc = "`read()` method returns [dmbaseline::R](dmbaseline::R) reader structure"]
impl crate::Readable for DMBASELINE {}
#[doc = "`write(|w| ..)` method takes [dmbaseline::W](dmbaseline::W) writer structure"]
impl crate::Writable for DMBASELINE {}
#[doc = "Delta Modulation Baseline"]
pub mod dmbaseline;
#[doc = "Delta Modulation Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmcfg](dmcfg) module"]
pub type DMCFG = crate::Reg<u32, _DMCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMCFG;
#[doc = "`read()` method returns [dmcfg::R](dmcfg::R) reader structure"]
impl crate::Readable for DMCFG {}
#[doc = "`write(|w| ..)` method takes [dmcfg::W](dmcfg::W) writer structure"]
impl crate::Writable for DMCFG {}
#[doc = "Delta Modulation Configuration"]
pub mod dmcfg;
#[doc = "Analog Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [anactrl](anactrl) module"]
pub type ANACTRL = crate::Reg<u32, _ANACTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANACTRL;
#[doc = "`read()` method returns [anactrl::R](anactrl::R) reader structure"]
impl crate::Readable for ANACTRL {}
#[doc = "`write(|w| ..)` method takes [anactrl::W](anactrl::W) writer structure"]
impl crate::Writable for ANACTRL {}
#[doc = "Analog Control"]
pub mod anactrl;
#[doc = "Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [if_](if_) module"]
pub type IF = crate::Reg<u32, _IF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF;
#[doc = "`read()` method returns [if_::R](if_::R) reader structure"]
impl crate::Readable for IF {}
#[doc = "Interrupt Flag"]
pub mod if_;
#[doc = "Interrupt Flag Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ifs](ifs) module"]
pub type IFS = crate::Reg<u32, _IFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFS;
#[doc = "`write(|w| ..)` method takes [ifs::W](ifs::W) writer structure"]
impl crate::Writable for IFS {}
#[doc = "Interrupt Flag Set"]
pub mod ifs;
#[doc = "Interrupt Flag Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ifc](ifc) module"]
pub type IFC = crate::Reg<u32, _IFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFC;
#[doc = "`write(|w| ..)` method takes [ifc::W](ifc::W) writer structure"]
impl crate::Writable for IFC {}
#[doc = "Interrupt Flag Clear"]
pub mod ifc;
#[doc = "Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ien](ien) module"]
pub type IEN = crate::Reg<u32, _IEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEN;
#[doc = "`read()` method returns [ien::R](ien::R) reader structure"]
impl crate::Readable for IEN {}
#[doc = "`write(|w| ..)` method takes [ien::W](ien::W) writer structure"]
impl crate::Writable for IEN {}
#[doc = "Interrupt Enable"]
pub mod ien;
