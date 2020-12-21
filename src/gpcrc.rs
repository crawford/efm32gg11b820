#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x08 - CRC Init Value"]
    pub init: INIT,
    #[doc = "0x0c - CRC Polynomial Value"]
    pub poly: POLY,
    #[doc = "0x10 - Input 32-bit Data Register"]
    pub inputdata: INPUTDATA,
    #[doc = "0x14 - Input 16-bit Data Register"]
    pub inputdatahword: INPUTDATAHWORD,
    #[doc = "0x18 - Input 8-bit Data Register"]
    pub inputdatabyte: INPUTDATABYTE,
    #[doc = "0x1c - CRC Data Register"]
    pub data: DATA,
    #[doc = "0x20 - CRC Data Reverse Register"]
    pub datarev: DATAREV,
    #[doc = "0x24 - CRC Data Byte Reverse Register"]
    pub databyterev: DATABYTEREV,
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
#[doc = "CRC Init Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [init](init) module"]
pub type INIT = crate::Reg<u32, _INIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INIT;
#[doc = "`read()` method returns [init::R](init::R) reader structure"]
impl crate::Readable for INIT {}
#[doc = "`write(|w| ..)` method takes [init::W](init::W) writer structure"]
impl crate::Writable for INIT {}
#[doc = "CRC Init Value"]
pub mod init;
#[doc = "CRC Polynomial Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [poly](poly) module"]
pub type POLY = crate::Reg<u32, _POLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POLY;
#[doc = "`read()` method returns [poly::R](poly::R) reader structure"]
impl crate::Readable for POLY {}
#[doc = "`write(|w| ..)` method takes [poly::W](poly::W) writer structure"]
impl crate::Writable for POLY {}
#[doc = "CRC Polynomial Value"]
pub mod poly;
#[doc = "Input 32-bit Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inputdata](inputdata) module"]
pub type INPUTDATA = crate::Reg<u32, _INPUTDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INPUTDATA;
#[doc = "`read()` method returns [inputdata::R](inputdata::R) reader structure"]
impl crate::Readable for INPUTDATA {}
#[doc = "`write(|w| ..)` method takes [inputdata::W](inputdata::W) writer structure"]
impl crate::Writable for INPUTDATA {}
#[doc = "Input 32-bit Data Register"]
pub mod inputdata;
#[doc = "Input 16-bit Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inputdatahword](inputdatahword) module"]
pub type INPUTDATAHWORD = crate::Reg<u32, _INPUTDATAHWORD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INPUTDATAHWORD;
#[doc = "`read()` method returns [inputdatahword::R](inputdatahword::R) reader structure"]
impl crate::Readable for INPUTDATAHWORD {}
#[doc = "`write(|w| ..)` method takes [inputdatahword::W](inputdatahword::W) writer structure"]
impl crate::Writable for INPUTDATAHWORD {}
#[doc = "Input 16-bit Data Register"]
pub mod inputdatahword;
#[doc = "Input 8-bit Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inputdatabyte](inputdatabyte) module"]
pub type INPUTDATABYTE = crate::Reg<u32, _INPUTDATABYTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INPUTDATABYTE;
#[doc = "`read()` method returns [inputdatabyte::R](inputdatabyte::R) reader structure"]
impl crate::Readable for INPUTDATABYTE {}
#[doc = "`write(|w| ..)` method takes [inputdatabyte::W](inputdatabyte::W) writer structure"]
impl crate::Writable for INPUTDATABYTE {}
#[doc = "Input 8-bit Data Register"]
pub mod inputdatabyte;
#[doc = "CRC Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "CRC Data Register"]
pub mod data;
#[doc = "CRC Data Reverse Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datarev](datarev) module"]
pub type DATAREV = crate::Reg<u32, _DATAREV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAREV;
#[doc = "`read()` method returns [datarev::R](datarev::R) reader structure"]
impl crate::Readable for DATAREV {}
#[doc = "CRC Data Reverse Register"]
pub mod datarev;
#[doc = "CRC Data Byte Reverse Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [databyterev](databyterev) module"]
pub type DATABYTEREV = crate::Reg<u32, _DATABYTEREV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATABYTEREV;
#[doc = "`read()` method returns [databyterev::R](databyterev::R) reader structure"]
impl crate::Readable for DATABYTEREV {}
#[doc = "CRC Data Byte Reverse Register"]
pub mod databyterev;
