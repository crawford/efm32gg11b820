#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Input Selection Register"]
    pub inputsel: INPUTSEL,
    #[doc = "0x08 - Status Register"]
    pub status: STATUS,
    #[doc = "0x0c - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x10 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x14 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x18 - Interrupt Enable Register"]
    pub ien: IEN,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - APORT Request Status Register"]
    pub aportreq: APORTREQ,
    #[doc = "0x24 - APORT Conflict Status Register"]
    pub aportconflict: APORTCONFLICT,
    #[doc = "0x28 - Hysteresis 0 Register"]
    pub hysteresis0: HYSTERESIS0,
    #[doc = "0x2c - Hysteresis 1 Register"]
    pub hysteresis1: HYSTERESIS1,
    _reserved11: [u8; 16usize],
    #[doc = "0x40 - I/O Routing Pine Enable Register"]
    pub routepen: ROUTEPEN,
    #[doc = "0x44 - I/O Routing Location Register"]
    pub routeloc0: ROUTELOC0,
    #[doc = "0x48 - External Override Interface Control"]
    pub extifctrl: EXTIFCTRL,
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
#[doc = "Input Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inputsel](inputsel) module"]
pub type INPUTSEL = crate::Reg<u32, _INPUTSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INPUTSEL;
#[doc = "`read()` method returns [inputsel::R](inputsel::R) reader structure"]
impl crate::Readable for INPUTSEL {}
#[doc = "`write(|w| ..)` method takes [inputsel::W](inputsel::W) writer structure"]
impl crate::Writable for INPUTSEL {}
#[doc = "Input Selection Register"]
pub mod inputsel;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status Register"]
pub mod status;
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
#[doc = "Hysteresis 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hysteresis0](hysteresis0) module"]
pub type HYSTERESIS0 = crate::Reg<u32, _HYSTERESIS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HYSTERESIS0;
#[doc = "`read()` method returns [hysteresis0::R](hysteresis0::R) reader structure"]
impl crate::Readable for HYSTERESIS0 {}
#[doc = "`write(|w| ..)` method takes [hysteresis0::W](hysteresis0::W) writer structure"]
impl crate::Writable for HYSTERESIS0 {}
#[doc = "Hysteresis 0 Register"]
pub mod hysteresis0;
#[doc = "Hysteresis 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hysteresis1](hysteresis1) module"]
pub type HYSTERESIS1 = crate::Reg<u32, _HYSTERESIS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HYSTERESIS1;
#[doc = "`read()` method returns [hysteresis1::R](hysteresis1::R) reader structure"]
impl crate::Readable for HYSTERESIS1 {}
#[doc = "`write(|w| ..)` method takes [hysteresis1::W](hysteresis1::W) writer structure"]
impl crate::Writable for HYSTERESIS1 {}
#[doc = "Hysteresis 1 Register"]
pub mod hysteresis1;
#[doc = "I/O Routing Pine Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routepen](routepen) module"]
pub type ROUTEPEN = crate::Reg<u32, _ROUTEPEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTEPEN;
#[doc = "`read()` method returns [routepen::R](routepen::R) reader structure"]
impl crate::Readable for ROUTEPEN {}
#[doc = "`write(|w| ..)` method takes [routepen::W](routepen::W) writer structure"]
impl crate::Writable for ROUTEPEN {}
#[doc = "I/O Routing Pine Enable Register"]
pub mod routepen;
#[doc = "I/O Routing Location Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routeloc0](routeloc0) module"]
pub type ROUTELOC0 = crate::Reg<u32, _ROUTELOC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTELOC0;
#[doc = "`read()` method returns [routeloc0::R](routeloc0::R) reader structure"]
impl crate::Readable for ROUTELOC0 {}
#[doc = "`write(|w| ..)` method takes [routeloc0::W](routeloc0::W) writer structure"]
impl crate::Writable for ROUTELOC0 {}
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "External Override Interface Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extifctrl](extifctrl) module"]
pub type EXTIFCTRL = crate::Reg<u32, _EXTIFCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTIFCTRL;
#[doc = "`read()` method returns [extifctrl::R](extifctrl::R) reader structure"]
impl crate::Readable for EXTIFCTRL {}
#[doc = "`write(|w| ..)` method takes [extifctrl::W](extifctrl::W) writer structure"]
impl crate::Writable for EXTIFCTRL {}
#[doc = "External Override Interface Control"]
pub mod extifctrl;
