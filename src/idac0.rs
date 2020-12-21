#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Current Programming Register"]
    pub curprog: CURPROG,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - Duty Cycle Configuration Register"]
    pub dutyconfig: DUTYCONFIG,
    _reserved3: [u8; 8usize],
    #[doc = "0x18 - Status Register"]
    pub status: STATUS,
    _reserved4: [u8; 4usize],
    #[doc = "0x20 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x24 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x28 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x2c - Interrupt Enable Register"]
    pub ien: IEN,
    _reserved8: [u8; 4usize],
    #[doc = "0x34 - APORT Request Status Register"]
    pub aportreq: APORTREQ,
    #[doc = "0x38 - APORT Request Status Register"]
    pub aportconflict: APORTCONFLICT,
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
#[doc = "Current Programming Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [curprog](curprog) module"]
pub type CURPROG = crate::Reg<u32, _CURPROG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CURPROG;
#[doc = "`read()` method returns [curprog::R](curprog::R) reader structure"]
impl crate::Readable for CURPROG {}
#[doc = "`write(|w| ..)` method takes [curprog::W](curprog::W) writer structure"]
impl crate::Writable for CURPROG {}
#[doc = "Current Programming Register"]
pub mod curprog;
#[doc = "Duty Cycle Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dutyconfig](dutyconfig) module"]
pub type DUTYCONFIG = crate::Reg<u32, _DUTYCONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DUTYCONFIG;
#[doc = "`read()` method returns [dutyconfig::R](dutyconfig::R) reader structure"]
impl crate::Readable for DUTYCONFIG {}
#[doc = "`write(|w| ..)` method takes [dutyconfig::W](dutyconfig::W) writer structure"]
impl crate::Writable for DUTYCONFIG {}
#[doc = "Duty Cycle Configuration Register"]
pub mod dutyconfig;
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
#[doc = "APORT Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aportconflict](aportconflict) module"]
pub type APORTCONFLICT = crate::Reg<u32, _APORTCONFLICT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APORTCONFLICT;
#[doc = "`read()` method returns [aportconflict::R](aportconflict::R) reader structure"]
impl crate::Readable for APORTCONFLICT {}
#[doc = "APORT Request Status Register"]
pub mod aportconflict;
